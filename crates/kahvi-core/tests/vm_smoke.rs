//! Smoke tests of the compiler and VM with a minimal hand-built realm (the
//! real built-ins live in kahvi-builtins). Each case is a script whose
//! completion value, converted with ToString, must match.

use kahvi_core::function::create_builtin_function;
use kahvi_core::object::ObjectKind;
use kahvi_core::prelude::*;
use kahvi_core::vm;

fn noop(_: &mut Agent, _: Value, _: &[Value], _: Option<Object>) -> JsResult<Value> {
    Ok(Value::Undefined)
}

fn thrower(agent: &mut Agent, _: Value, _: &[Value], _: Option<Object>) -> JsResult<Value> {
    Err(agent.throw_type_error("restricted"))
}

fn gen_next(agent: &mut Agent, this: Value, args: &[Value], _: Option<Object>) -> JsResult<Value> {
    vm::generator_resume(agent, this, args.get_or_undefined(0))
}

fn gen_return(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    _: Option<Object>,
) -> JsResult<Value> {
    vm::generator_resume_abrupt(agent, this, args.get_or_undefined(0), true)
}

fn return_this(_: &mut Agent, this: Value, _: &[Value], _: Option<Object>) -> JsResult<Value> {
    Ok(this)
}

fn indirect_eval(
    agent: &mut Agent,
    _: Value,
    args: &[Value],
    _: Option<Object>,
) -> JsResult<Value> {
    vm::perform_eval(agent, args.get_or_undefined(0), false, None)
}

fn install(agent: &mut Agent, realm: RealmId) -> JsResult<()> {
    let object_proto = agent.object_with_kind(None, ObjectKind::ImmutablePrototype);
    agent.set_intrinsic(realm, "%Object.prototype%", object_proto);
    let empty = PropertyKey::String(agent.names.empty);
    let fp = create_builtin_function(
        agent,
        noop,
        0,
        empty,
        vec![],
        Some(realm),
        Some(object_proto),
        None,
        false,
    );
    agent.set_intrinsic(realm, "%Function.prototype%", fp);
    let ap = agent.object_with_kind(Some(object_proto), ObjectKind::Array);
    let len = PropertyKey::String(agent.names.length);
    agent.obj_mut(ap).properties.insert(
        len,
        Property::Data {
            value: Value::Number(0.0),
            writable: true,
            enumerable: false,
            configurable: false,
        },
    );
    agent.set_intrinsic(realm, "%Array.prototype%", ap);
    for name in [
        "%AsyncFunction.prototype%",
        "%GeneratorFunction.prototype%",
        "%AsyncGeneratorFunction.prototype%",
    ] {
        let o = agent.ordinary_object_create(Some(fp));
        agent.set_intrinsic(realm, name, o);
    }
    let gp = agent.ordinary_object_create(Some(object_proto));
    let next = builtin(agent, gen_next, 1, "next");
    let ret = builtin(agent, gen_return, 1, "return");
    let k = agent.key("next");
    agent.obj_mut(gp).properties.insert(
        k,
        Property::Data {
            value: Value::Object(next),
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
    let k = agent.key("return");
    agent.obj_mut(gp).properties.insert(
        k,
        Property::Data {
            value: Value::Object(ret),
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
    let iter = builtin(agent, return_this, 0, "[Symbol.iterator]");
    let k = PropertyKey::Symbol(agent.symbols_wk.iterator);
    agent.obj_mut(gp).properties.insert(
        k,
        Property::Data {
            value: Value::Object(iter),
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
    agent.set_intrinsic(realm, "%GeneratorPrototype%", gp);
    agent.set_intrinsic(realm, "%AsyncGeneratorPrototype%", gp);
    for name in [
        "%Error.prototype%",
        "%EvalError.prototype%",
        "%RangeError.prototype%",
        "%ReferenceError.prototype%",
        "%SyntaxError.prototype%",
        "%TypeError.prototype%",
        "%URIError.prototype%",
    ] {
        let o = agent.ordinary_object_create(Some(object_proto));
        let n = agent.key("name");
        let s = agent.string(&name[1..name.len() - 11]);
        agent.obj_mut(o).properties.insert(
            n,
            Property::Data {
                value: Value::String(s),
                writable: true,
                enumerable: false,
                configurable: true,
            },
        );
        agent.set_intrinsic(realm, name, o);
    }
    let t = builtin(agent, thrower, 0, "");
    agent.set_intrinsic(realm, "%ThrowTypeError%", t);
    let v = builtin(agent, noop, 0, "values");
    agent.set_intrinsic(realm, "%Array.prototype.values%", v);
    let eval = builtin(agent, indirect_eval, 1, "eval");
    agent.set_intrinsic(realm, "%eval%", eval);
    let global = agent.realm(realm).global_object;
    let k = agent.key("eval");
    agent.obj_mut(global).properties.insert(
        k,
        Property::Data {
            value: Value::Object(eval),
            writable: true,
            enumerable: false,
            configurable: true,
        },
    );
    Ok(())
}

fn run(src: &str) -> String {
    let mut agent = Agent::new();
    agent.set_intrinsics_installer(install);
    let realm = agent.create_realm().expect("realm");
    let script = match vm::parse_script(&mut agent, src, "test", realm) {
        Ok(s) => s,
        Err(e) => return describe_error(&mut agent, e),
    };
    agent.contexts.push(kahvi_core::agent::ExecutionContext {
        function: None,
        realm,
        script_or_module: None,
    });
    let r = vm::evaluate_script(&mut agent, &script);
    let out = match r {
        Ok(v) => {
            let s = ops::to_string(&mut agent, v).expect("ToString");
            agent.str_lossy(s)
        }
        Err(e) => describe_error(&mut agent, e),
    };
    agent.contexts.pop();
    out
}

fn describe_error(agent: &mut Agent, e: JsError) -> String {
    match e {
        JsError::Throw(Value::Object(o)) => {
            let n = agent.key("name");
            let m = agent.key("message");
            let name = ops::get(agent, o, n)
                .ok()
                .and_then(|v| v.as_string())
                .map(|s| agent.str_lossy(s))
                .unwrap_or_default();
            let msg = ops::get(agent, o, m)
                .ok()
                .and_then(|v| v.as_string())
                .map(|s| agent.str_lossy(s))
                .unwrap_or_default();
            format!("throw {name}: {msg}")
        }
        JsError::Throw(v) => {
            let s = ops::to_string(agent, v)
                .map(|s| agent.str_lossy(s))
                .unwrap_or_default();
            format!("throw {s}")
        }
        JsError::NotImplemented(m) => format!("not implemented: {m}"),
    }
}

#[test]
fn smoke() {
    let cases: &[(&str, &str)] = &[
        ("1 + 2", "3"),
        ("'a' + 1", "a1"),
        ("var x = 5; x * 2", "10"),
        ("let a = 1; { let a = 2; } a", "1"),
        (
            "function f(n) { return n < 2 ? n : f(n - 1) + f(n - 2); } f(15)",
            "610",
        ),
        (
            "var s = 0; for (var i = 0; i < 10; i++) { if (i == 5) continue; if (i == 8) break; s += i; } s",
            "23",
        ),
        (
            "var fs = []; for (let i = 0; i < 3; i++) fs[i] = () => i; fs[0]() + fs[1]() + fs[2]()",
            "3",
        ),
        (
            "function mk() { var c = 0; return function() { return ++c; }; } var g = mk(); g(); g(); g()",
            "3",
        ),
        ("var r; try { throw 1; } catch (e) { r = e + 1; } r", "2"),
        (
            "var log = ''; function f() { try { return 'a'; } finally { log += 'f'; } } f() + log",
            "af",
        ),
        (
            "var log = ''; out: for (var i = 0; i < 3; i++) { try { for (;;) { try { continue out; } finally { log += i; } } } finally { log += '|'; } } log",
            "0|1|2|",
        ),
        (
            "var o = { a: 1, get b() { return this.a + 1; }, ['c' + 1]: 3 }; o.a + o.b + o.c1",
            "6",
        ),
        (
            "function* two() { yield 2; } var { a, b: [c, d = 4], ...rest } = { a: 1, b: two(), e: 5, f: 6 }; a + c + d + rest.e + rest.f",
            "18",
        ),
        (
            "function* g() { yield 1; yield 2; } var x, y; [x, , y = 9] = g(); x + y",
            "10",
        ),
        (
            "var closed = 0; function* g() { try { yield 1; yield 2; } finally { closed++; } } for (var v of g()) { if (v == 1) break; } var [p] = g(); closed + p",
            "3",
        ),
        (
            "var o = {}; function* g() { yield 1; yield 2; } [o.a, ...o.r] = g(); o.a + o.r[0]",
            "3",
        ),
        (
            "switch (3) { case 1: 'one'; case 3: 'three'; case 4: 'four'; break; default: 'd'; }",
            "four",
        ),
        (
            "typeof undefinedVariable + typeof null + typeof function(){}",
            "undefinedobjectfunction",
        ),
        (
            "try { undefinedVariable } catch (e) { e.name }",
            "ReferenceError",
        ),
        ("try { null.x } catch (e) { e.name }", "TypeError"),
        (
            "class A { #x = 1; static s = 10; constructor(y) { this.y = y; } get x() { return this.#x; } m() { return this.#x + this.y; } } new A(2).m() + A.s",
            "13",
        ),
        (
            "class A { constructor() { this.a = 1; } f() { return 'A'; } } class B extends A { constructor() { super(); this.b = 2; } f() { return super.f() + 'B'; } } var b = new B(); b.f() + b.a + b.b",
            "AB12",
        ),
        ("class A {} class B extends A { x = 5 } new B().x", "5"),
        (
            "function* g() { var x = yield 1; yield x * 2; } var it = g(); it.next().value + it.next(21).value",
            "43",
        ),
        (
            "function* g() { try { yield 1; } finally { log = 'closed'; } } var log; var it = g(); it.next(); it.return(5).value + log",
            "5closed",
        ),
        (
            "var f = function() {}; var o = { m() {} }; f.name + o.m.name + (() => {}).name",
            "fm",
        ),
        ("var x = 1; eval('var x = 2; x + 1') + x", "5"),
        (
            "function f() { var a = 1; eval('var a = 2'); return a; } f()",
            "2",
        ),
        ("(function() { 'use strict'; return this; })()", "undefined"),
        (
            "function f(a, b = a + 1, ...r) { return a + b + r.length; } f(1) + f(1, 1, 1, 1)",
            "7",
        ),
        ("function f(a) { arguments[0] = 9; return a; } f(1)", "9"),
        ("var o = { x: 1 }; with (o) { x = 2; } o.x", "2"),
        (
            "var t = 0; for (var k in { a: 1, b: 2 }) t += k.length; t",
            "2",
        ),
        ("`a${1 + 1}b${'c'}`", "a2bc"),
        (
            "var o = null; o?.x?.y + '' + (o ?? 'dflt')",
            "undefineddflt",
        ),
        ("var a = 0; a ||= 5; a &&= a + 1; a", "6"),
        ("1; do { 2; break; } while (false)", "2"),
        ("1; if (false) 2;", "undefined"),
        ("label: { 1; break label; }", "1"),
        ("2 ** 10 + (7 % 3) + (-7 >> 1) + (-1 >>> 28)", "1036"),
        ("'b' > 'a' && 1 < 2 && !(0/0 < 1)", "true"),
        (
            "var log = ''; var f = () => { log += 'x'; }; (f)(); f?.(); log",
            "xx",
        ),
        (
            "var o = { f() { return this === o; } }; (o.f)() && o['f']()",
            "true",
        ),
        (
            "function F() { this.a = 1; } F.prototype.b = 2; var o = new F(); o.a + o.b + (o instanceof F)",
            "4",
        ),
        (
            "let x = 1; try { (function() { x2 = 1; 'use strict'; })(); typeof x2 } catch (e) { e.name }",
            "number",
        ),
        (
            "'use strict'; try { undeclared = 1; } catch (e) { e.name }",
            "ReferenceError",
        ),
        (
            "const c = 1; try { c = 2; } catch (e) { e.name }",
            "TypeError",
        ),
        (
            "try { tdz; let tdz = 1; } catch (e) { e.name }",
            "ReferenceError",
        ),
        ("var o = { a: 1 }; delete o.a; 'a' in o", "false"),
        (
            "class C { static #p() { return 7; } static m() { return C.#p(); } } C.m()",
            "7",
        ),
        ("class C { static { this.x = 3; } } C.x", "3"),
        ("var C = class { constructor() {} }; C.name", "C"),
        ("try { class C {} C(); } catch (e) { e.name }", "TypeError"),
    ];
    let mut failures = Vec::new();
    for (src, expected) in cases {
        let got = run(src);
        if got != *expected {
            failures.push(format!(
                "{src}\n    expected {expected}\n    got      {got}"
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} cases failed:\n{}",
        failures.len(),
        cases.len(),
        failures.join("\n")
    );
}
