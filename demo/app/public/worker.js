// Runs one program with kahvi compiled to WebAssembly. The page starts a new
// worker for each run and terminates it to stop a program.
let instance;
const timers = new Map();
const decoder = new TextDecoder();
const memory = () => new Uint8Array(instance.exports.memory.buffer);

function maybeDone() {
  if (timers.size === 0) postMessage({ done: true });
}

function fail() {
  for (const t of timers.values()) clearTimeout(t); // also clears intervals
  timers.clear();
  postMessage({ done: true });
}

const host = {
  write(stream, ptr, len) {
    postMessage({ stream, text: decoder.decode(memory().slice(ptr, ptr + len)) });
  },
  now: () => Date.now(),
  set_timer(id, ms, repeat) {
    const fire = () => {
      if (!instance.exports.kahvi_fire(id)) return fail();
      if (!repeat) {
        timers.delete(id);
        maybeDone();
      }
    };
    timers.set(id, repeat ? setInterval(fire, ms) : setTimeout(fire, ms));
  },
  clear_timer(id) {
    clearTimeout(timers.get(id));
    timers.delete(id);
    // Checked once the current callback returns.
    queueMicrotask(maybeDone);
  },
};

onmessage = ({ data: { module, code } }) => {
  instance = new WebAssembly.Instance(module, { host });
  const bytes = new TextEncoder().encode(code);
  const ptr = instance.exports.kahvi_alloc(bytes.length);
  memory().set(bytes, ptr);
  const start = performance.now();
  const ok = instance.exports.kahvi_run(ptr, bytes.length);
  postMessage({ ran: performance.now() - start });
  if (!ok) return fail();
  maybeDone();
};
