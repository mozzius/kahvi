// A Scheme-like interpreter: tokenizer, reader, environments as Maps,
// closures, tail calls in a trampoline loop, and a small standard library
// written in the language itself.
import { show } from "./lib/util.mjs";

const Sym = (name) => Symbol.for(name);
const isSym = (x) => typeof x === "symbol";

function tokenize(src) {
  return src.replace(/;[^\n]*/g, "").replace(/([()'])/g, " $1 ").trim().split(/\s+/);
}
function read(tokens) {
  const tok = tokens.shift();
  if (tok === "(") {
    const list = [];
    while (tokens[0] !== ")") {
      if (!tokens.length) throw new SyntaxError("unexpected end of input");
      list.push(read(tokens));
    }
    tokens.shift();
    return list;
  }
  if (tok === ")") throw new SyntaxError("unexpected )");
  if (tok === "'") return [Sym("quote"), read(tokens)];
  if (tok === "#t" || tok === "#f") return tok === "#t";
  if (/^-?\d+(\.\d+)?$/.test(tok)) return Number(tok);
  if (/^".*"$/.test(tok)) return tok.slice(1, -1);
  return Sym(tok);
}
function readAll(src) {
  const tokens = tokenize(src), forms = [];
  while (tokens.length) forms.push(read(tokens));
  return forms;
}

class Env {
  constructor(parent = null) { this.vars = new Map(); this.parent = parent; }
  lookup(s) {
    for (let e = this; e; e = e.parent) if (e.vars.has(s)) return e.vars.get(s);
    throw new ReferenceError(`unbound: ${Symbol.keyFor(s)}`);
  }
  set(s, v) {
    for (let e = this; e; e = e.parent) if (e.vars.has(s)) return void e.vars.set(s, v);
    throw new ReferenceError(`unbound: ${Symbol.keyFor(s)}`);
  }
}

class Lambda {
  constructor(params, body, env) { Object.assign(this, { params, body, env }); }
}

function evaluate(x, env) {
  for (;;) {
    if (isSym(x)) return env.lookup(x);
    if (!Array.isArray(x)) return x;
    const [op, ...args] = x;
    switch (op) {
      case Sym("quote"): return args[0];
      case Sym("if"): x = evaluate(args[0], env) !== false ? args[1] : args[2] ?? false; continue;
      case Sym("define"):
        if (Array.isArray(args[0])) {
          const [name, ...params] = args[0];
          env.vars.set(name, new Lambda(params, [Sym("begin"), ...args.slice(1)], env));
        } else env.vars.set(args[0], evaluate(args[1], env));
        return undefined;
      case Sym("set!"): env.set(args[0], evaluate(args[1], env)); return undefined;
      case Sym("lambda"): return new Lambda(args[0], [Sym("begin"), ...args.slice(1)], env);
      case Sym("begin"):
        for (const e of args.slice(0, -1)) evaluate(e, env);
        x = args[args.length - 1];
        continue;
      case Sym("let"): {
        const inner = new Env(env);
        for (const [n, v] of args[0]) inner.vars.set(n, evaluate(v, env));
        env = inner;
        x = [Sym("begin"), ...args.slice(1)];
        continue;
      }
      case Sym("cond"):
        x = false;
        for (const [test, ...body] of args) {
          if (test === Sym("else") || evaluate(test, env) !== false) { x = [Sym("begin"), ...body]; break; }
        }
        continue;
      default: {
        const f = evaluate(op, env);
        const vals = args.map((a) => evaluate(a, env));
        if (typeof f === "function") return f(...vals);
        if (!(f instanceof Lambda)) throw new TypeError("not a procedure: " + toString(f));
        const inner = new Env(f.env);
        f.params.forEach((p, i) => inner.vars.set(p, vals[i]));
        env = inner;
        x = f.body;
      }
    }
  }
}

function toString(v) {
  if (v === true) return "#t";
  if (v === false) return "#f";
  if (isSym(v)) return Symbol.keyFor(v);
  if (Array.isArray(v)) return "(" + v.map(toString).join(" ") + ")";
  if (v instanceof Lambda) return "#<lambda>";
  return String(v);
}

const global = new Env();
const builtins = {
  "+": (...a) => a.reduce((x, y) => x + y, 0), "-": (a, b) => (b === undefined ? -a : a - b),
  "*": (...a) => a.reduce((x, y) => x * y, 1), "/": (a, b) => a / b, "<": (a, b) => a < b,
  ">": (a, b) => a > b, "=": (a, b) => a === b, "<=": (a, b) => a <= b, "mod": (a, b) => a % b,
  "car": (l) => l[0], "cdr": (l) => l.slice(1), "cons": (a, l) => [a, ...l], "null?": (l) => l.length === 0,
  "list": (...a) => a, "length": (l) => l.length, "display": (v) => void out.push(toString(v)),
  "string-append": (...s) => s.join(""), "number->string": (n) => String(n), "apply": (f, l) => call(f, l),
};
function call(f, args) {
  return typeof f === "function" ? f(...args) : evaluate([f, ...args.map((a) => [Sym("quote"), a])], global);
}
for (const [k, v] of Object.entries(builtins)) global.vars.set(Sym(k), v);
const out = [];

const program = `
(define (map f l) (if (null? l) '() (cons (f (car l)) (map f (cdr l)))))
(define (filter p l) (cond ((null? l) '()) ((p (car l)) (cons (car l) (filter p (cdr l)))) (else (filter p (cdr l)))))
(define (foldl f acc l) (if (null? l) acc (foldl f (f acc (car l)) (cdr l))))
(define (range a b) (if (>= a b) '() (cons a (range (+ a 1) b))))
(define (>= a b) (if (< a b) #f #t))
(define (append a b) (if (null? a) b (cons (car a) (append (cdr a) b))))
(define (qsort l)
  (if (null? l) '()
      (let ((p (car l)) (rest (cdr l)))
        (append (qsort (filter (lambda (x) (< x p)) rest))
                (cons p (qsort (filter (lambda (x) (>= x p)) rest)))))))
(define (fib n) (if (< n 2) n (+ (fib (- n 1)) (fib (- n 2)))))
(define (count-down n acc) (if (= n 0) acc (count-down (- n 1) (+ acc 1))))
(define (make-counter)
  (let ((n 0)) (lambda () (set! n (+ n 1)) n)))
(define c (make-counter))
(c) (c)
(display (fib 20))
(display (qsort '(33 4 15 8 42 16 23 1 99 0 -5 7)))
(display (foldl + 0 (map (lambda (x) (* x x)) (range 0 100))))
(display (count-down 100000 0))
(display (c))
(display (filter (lambda (n) (= 0 (mod n 7))) (range 1 60)))
(display (apply string-append (map number->string (range 0 10))))
(display (let ((x 2) (y 3)) (let ((x 7) (z (+ x y))) (* z x))))
`;
for (const form of readAll(program)) evaluate(form, global);
for (const line of out) console.log(line);
for (const bad of ["(car", "(undefined-fn 1)", ")"]) {
  try {
    for (const f of readAll(bad)) evaluate(f, global);
  } catch (e) {
    show("error", e.constructor.name + " " + e.message);
  }
}
