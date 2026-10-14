// An exception thrown by a timer callback ends the process with status 1
// after reporting it; later timers do not run.
// exit: 1
console.log('before');
setTimeout(() => {
  throw new TypeError('from a timer');
}, 1);
setTimeout(() => console.log('not reached'), 20);
