// A module that throws (here after a top-level await) reports the exception.
// exit: 1
await null;
console.log('awaited');
throw new Error('top level');
