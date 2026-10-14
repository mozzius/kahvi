// A top-level await that can never settle ends with status 13 (as in Node).
// exit: 13
console.log('waiting');
await new Promise(() => {});
