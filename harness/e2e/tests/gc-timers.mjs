// Timer callbacks and their arguments survive garbage collection while they
// are pending (they are held by the host, not by any JavaScript value).
let done = 0;
for (let i = 0; i < 20; i++) {
  setTimeout((o) => {
    if (o.id !== i || o.data.length !== 50) throw new Error('lost ' + i);
    done++;
  }, i % 5, { id: i, data: Array.from({ length: 50 }, (_, j) => ({ j })) });
}
let garbage = [];
for (let i = 0; i < 2000; i++) garbage.push({ i, s: 'x' + i });
garbage = null;
setTimeout(() => console.log('done', done), 30);
