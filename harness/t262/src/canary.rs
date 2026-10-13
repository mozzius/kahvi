//! Canaries: synthetic tests with known outcomes, regenerated with random
//! identifiers and messages on every run so an engine cannot recognise them.
//!
//! "Should fail" canaries guard against an engine (or a runner bug) that makes
//! everything look green: if any of them passes, the whole run is invalid.
//! "Should pass" canaries are informational until the engine can run the
//! harness.

use crate::util::Rng;

pub struct Canary {
    pub name: &'static str,
    pub should_pass: bool,
    pub source: String,
}

pub fn generate(rng: &mut Rng) -> Vec<Canary> {
    let id = rng.ident();
    let f = rng.ident();
    let n = rng.next() % 100_000;
    let msg = rng.hex();
    let fm = |extra: &str| format!("/*---\ndescription: kahvi runner canary\n{extra}---*/\n");
    let mk = |name, should_pass, extra: &str, body: String| Canary {
        name,
        should_pass,
        source: format!("{}{}\n", fm(extra), body),
    };
    vec![
        mk(
            "fail-throw",
            false,
            "",
            format!("var {id} = {n};\nthrow new Test262Error(\"{msg}\");"),
        ),
        mk(
            "fail-throw-midway",
            false,
            "",
            format!(
                "var {id} = 0;\n{id}++;\nif ({id} === 1) {{ throw new Test262Error(\"{msg}\"); }}\n{id}++;"
            ),
        ),
        mk(
            "fail-assert",
            false,
            "",
            format!("assert.sameValue({n}, {}, \"{msg}\");", n + 1),
        ),
        mk(
            "fail-throw-in-function",
            false,
            "",
            format!("function {f}() {{ throw new Test262Error(\"{msg}\"); }}\n{f}();"),
        ),
        mk(
            "fail-async-never-done",
            false,
            "flags: [async]\n",
            format!("var {id} = Promise.resolve({n});\n{id}.then(function () {{}});"),
        ),
        mk(
            "fail-async-reports-failure",
            false,
            "flags: [async]\n",
            format!(
                "Promise.reject(new Test262Error(\"{msg}\")).then(function () {{ $DONE(); }}, $DONE);"
            ),
        ),
        mk(
            "fail-negative-but-valid",
            false,
            "negative:\n  phase: parse\n  type: SyntaxError\n",
            format!("$DONOTEVALUATE();\nvar {id} = {n};"),
        ),
        mk(
            "fail-negative-wrong-type",
            false,
            "negative:\n  phase: runtime\n  type: TypeError\n",
            format!("throw new RangeError(\"{msg}\");"),
        ),
        mk(
            "fail-raw-throw",
            false,
            "flags: [raw]\n",
            format!("throw new Error(\"{msg}\");"),
        ),
        mk(
            "fail-module-throw",
            false,
            "flags: [module]\n",
            format!("export var {id} = {n};\nthrow new Test262Error(\"{msg}\");"),
        ),
        mk(
            "pass-basic",
            true,
            "",
            format!("var {id} = {n};\nassert.sameValue({id}, {n});"),
        ),
        mk(
            "pass-strict",
            true,
            "flags: [onlyStrict]\n",
            "assert.sameValue(function () { return this; }(), undefined);".to_string(),
        ),
        mk(
            "pass-async",
            true,
            "flags: [async]\n",
            format!(
                "Promise.resolve({n}).then(function (v) {{ assert.sameValue(v, {n}); }}).then($DONE, $DONE);"
            ),
        ),
        mk(
            "pass-negative-parse",
            true,
            "negative:\n  phase: parse\n  type: SyntaxError\n",
            format!("$DONOTEVALUATE();\nvar var = {n};"),
        ),
        mk(
            "pass-module",
            true,
            "flags: [module]\n",
            format!("export var {id} = {n};\nassert.sameValue({id}, {n});"),
        ),
    ]
}
