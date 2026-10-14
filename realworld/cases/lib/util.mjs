// Helpers shared by the cases. Output is plain strings only, so it does not
// depend on how a runtime's console formats objects.

// 32-bit FNV-1a over UTF-16 code units, as 8 hex digits: a short stand-in
// for output too long to print whole.
export function fnv1a(s) {
  let h = 0x811c9dc5;
  for (let i = 0; i < s.length; i++) {
    h ^= s.charCodeAt(i);
    h = Math.imul(h, 0x01000193) >>> 0;
  }
  return h.toString(16).padStart(8, "0");
}

export function show(label, value) {
  console.log(label + ": " + (typeof value === "string" ? value : JSON.stringify(value)));
}

// JSON.stringify that writes BigInts (ESTree literals carry them) as "123n".
export function json(value) {
  return JSON.stringify(value, (k, v) => (typeof v === "bigint" ? v + "n" : v));
}
