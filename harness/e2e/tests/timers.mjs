// Event loop order: synchronous code, then microtasks (queueMicrotask and
// promise jobs in queue order), then timers by due time and creation order,
// each followed by the microtasks it queued. Cleared timers never run.
const log = [];
setTimeout(() => log.push('t10'), 10);
setTimeout(() => {
  log.push('t0');
  Promise.resolve().then(() => log.push('t0 micro'));
}, 0);
setTimeout((a, b) => log.push(`args ${a} ${b}`), 1, 'x', 'y');
const never = setTimeout(() => log.push('never'), 1);
clearTimeout(never);
let ticks = 0;
const iv = setInterval(() => {
  log.push(`tick ${++ticks}`);
  if (ticks === 3) clearInterval(iv);
}, 2);
queueMicrotask(() => log.push('micro 1'));
Promise.resolve().then(() => log.push('promise'));
queueMicrotask(() => log.push('micro 2'));
log.push('sync');
setTimeout(() => console.log(log.join('\n')), 50);
console.log(typeof setTimeout(() => {}, 0));
try {
  setTimeout('code', 0);
} catch (e) {
  console.log(e.constructor.name);
}
