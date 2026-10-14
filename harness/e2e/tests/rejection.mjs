// A rejection with no handler by the end of the microtasks ends the process;
// one handled later in the same turn does not.
// exit: 1
const late = Promise.reject(new Error('handled late'));
late.catch(() => console.log('caught late'));
Promise.reject(new SyntaxError('nobody listens'));
