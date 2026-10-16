// The web classes a fetch-style server needs (Headers, Request, Response)
// and the `Kahvi` global, for kahvi-serve. Bodies are kept whole in memory:
// `body` is a Uint8Array (or null), not a ReadableStream.
(function (global, register, respond, readFile) {
  "use strict";
  const define = (name, value) =>
    Object.defineProperty(global, name, { value, writable: true, enumerable: false, configurable: true });
  const encoder = new TextEncoder();
  const decoder = new TextDecoder();

  const TOKEN = /^[!#$%&'*+\-.^_`|~0-9A-Za-z]+$/;
  const normalizeValue = (v) => `${v}`.replace(/^[\t\n\r ]+|[\t\n\r ]+$/g, "");

  class Headers {
    #map = new Map(); // lower-case name -> values
    constructor(init = undefined) {
      if (init == null) return;
      if (init instanceof Headers) {
        for (const [name, values] of init.#map) this.#map.set(name, [...values]);
      } else if (typeof init[Symbol.iterator] === "function") {
        for (const pair of init) {
          const [name, value, ...rest] = [...pair];
          if (rest.length || value === undefined) throw new TypeError("Headers: each pair needs a name and a value");
          this.append(name, value);
        }
      } else {
        for (const name of Object.keys(init)) this.append(name, init[name]);
      }
    }
    static #name(name) {
      name = `${name}`;
      if (!TOKEN.test(name)) throw new TypeError(`Headers: invalid name ${JSON.stringify(name)}`);
      return name.toLowerCase();
    }
    append(name, value) {
      name = Headers.#name(name);
      const values = this.#map.get(name);
      if (values) values.push(normalizeValue(value));
      else this.#map.set(name, [normalizeValue(value)]);
    }
    set(name, value) {
      this.#map.set(Headers.#name(name), [normalizeValue(value)]);
    }
    get(name) {
      const values = this.#map.get(Headers.#name(name));
      return values ? values.join(", ") : null;
    }
    getSetCookie() {
      return [...(this.#map.get("set-cookie") ?? [])];
    }
    has(name) {
      return this.#map.has(Headers.#name(name));
    }
    delete(name) {
      this.#map.delete(Headers.#name(name));
    }
    *entries() {
      for (const name of [...this.#map.keys()].sort()) {
        if (name === "set-cookie") {
          for (const v of this.#map.get(name)) yield [name, v];
        } else {
          yield [name, this.#map.get(name).join(", ")];
        }
      }
    }
    *keys() {
      for (const [name] of this.entries()) yield name;
    }
    *values() {
      for (const [, value] of this.entries()) yield value;
    }
    forEach(callback, thisArg = undefined) {
      for (const [name, value] of this.entries()) callback.call(thisArg, value, name, this);
    }
    [Symbol.iterator]() {
      return this.entries();
    }
  }
  Object.defineProperty(Headers.prototype, Symbol.toStringTag, { value: "Headers", configurable: true });

  // A body as bytes, and the content type it implies.
  function extractBody(body) {
    if (body == null) return [null, null];
    if (typeof body === "string") return [encoder.encode(body), "text/plain;charset=UTF-8"];
    if (body instanceof ArrayBuffer) return [new Uint8Array(body.slice(0)), null];
    if (ArrayBuffer.isView(body)) {
      return [new Uint8Array(body.buffer.slice(body.byteOffset, body.byteOffset + body.byteLength)), null];
    }
    throw new TypeError("kahvi-serve: bodies must be strings, ArrayBuffers or typed arrays (no streams)");
  }

  class Body {
    #bytes;
    #used = false;
    constructor(bytes) {
      this.#bytes = bytes;
    }
    get body() {
      return this.#bytes;
    }
    get bodyUsed() {
      return this.#used;
    }
    #consume() {
      if (this.#used) return Promise.reject(new TypeError("Body has already been consumed"));
      this.#used = true;
      return Promise.resolve(this.#bytes ?? new Uint8Array(0));
    }
    async arrayBuffer() {
      const b = await this.#consume();
      return b.buffer.slice(b.byteOffset, b.byteOffset + b.byteLength);
    }
    async bytes() {
      return new Uint8Array(await this.arrayBuffer());
    }
    async text() {
      return decoder.decode(await this.#consume());
    }
    async json() {
      return JSON.parse(await this.text());
    }
  }

  class Request extends Body {
    #url;
    #method;
    #headers;
    #signal;
    constructor(input, init = {}) {
      const from = input instanceof Request ? input : null;
      let bytes = from ? from.body : null;
      let type = null;
      if (init.body !== undefined) [bytes, type] = extractBody(init.body);
      super(bytes);
      this.#url = from ? from.url : `${input}`;
      this.#method = `${init.method ?? from?.method ?? "GET"}`.toUpperCase();
      this.#headers = new Headers(init.headers ?? from?.headers);
      if (type && !this.#headers.has("content-type")) this.#headers.set("content-type", type);
      this.#signal = init.signal ?? from?.signal ?? null;
    }
    get url() {
      return this.#url;
    }
    get method() {
      return this.#method;
    }
    get headers() {
      return this.#headers;
    }
    get signal() {
      return this.#signal;
    }
    clone() {
      return new Request(this);
    }
  }
  Object.defineProperty(Request.prototype, Symbol.toStringTag, { value: "Request", configurable: true });

  class Response extends Body {
    #status;
    #statusText;
    #headers;
    constructor(body = null, init = {}) {
      const [bytes, type] = extractBody(body);
      super(bytes);
      const status = init.status ?? 200;
      if (!Number.isInteger(status) || status < 200 || status > 599) {
        throw new RangeError(`Response: invalid status ${status}`);
      }
      this.#status = status;
      this.#statusText = `${init.statusText ?? ""}`;
      this.#headers = new Headers(init.headers);
      if (type && !this.#headers.has("content-type")) this.#headers.set("content-type", type);
    }
    static json(data, init = {}) {
      const headers = new Headers(init.headers);
      if (!headers.has("content-type")) headers.set("content-type", "application/json");
      return new Response(JSON.stringify(data), { ...init, headers });
    }
    static redirect(url, status = 302) {
      return new Response(null, { status, headers: { location: `${url}` } });
    }
    get status() {
      return this.#status;
    }
    get statusText() {
      return this.#statusText;
    }
    get ok() {
      return this.#status >= 200 && this.#status <= 299;
    }
    get headers() {
      return this.#headers;
    }
    get type() {
      return "default";
    }
    get redirected() {
      return false;
    }
    clone() {
      return new Response(this.body, this);
    }
  }
  Object.defineProperty(Response.prototype, Symbol.toStringTag, { value: "Response", configurable: true });

  define("Headers", Headers);
  define("Request", Request);
  define("Response", Response);
  define("Kahvi", Object.freeze({
    // Serve HTTP with `handler(request)` (or `handler.fetch(request)`),
    // which returns a Response or a promise of one.
    serve(handler) {
      const fetch = typeof handler === "function" ? handler : handler?.fetch?.bind(handler);
      if (typeof fetch !== "function") throw new TypeError("Kahvi.serve: expected a function or { fetch }");
      register(fetch);
    },
    // The bytes of a file, as an ArrayBuffer.
    readFile(path) {
      return readFile(`${path}`);
    },
  }));

  // Called by the host for each request; reports through `respond`.
  return async function dispatch(fetch, method, url, head, body) {
    let res;
    try {
      const headers = new Headers();
      for (const line of head.split("\r\n")) {
        const colon = line.indexOf(":");
        if (colon > 0) headers.append(line.slice(0, colon), line.slice(colon + 1));
      }
      const req = new Request(url, { method, headers, body: method === "GET" || method === "HEAD" ? undefined : body });
      res = await fetch(req);
      if (!(res instanceof Response)) throw new TypeError("the handler did not return a Response");
    } catch (e) {
      console.error("kahvi-serve: request failed:", e);
      res = new Response("Internal Server Error\n", { status: 500 });
    }
    let out = "";
    for (const [name, value] of res.headers) out += `${name}: ${value}\r\n`;
    const bytes = res.body ?? new Uint8Array(0);
    respond(res.status, res.statusText, out, bytes.buffer.slice(bytes.byteOffset, bytes.byteOffset + bytes.byteLength));
  };
})
