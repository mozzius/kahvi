//! CreateDynamicFunction (§20.2.1.1.1): the `Function`, `GeneratorFunction`,
//! `AsyncFunction` and `AsyncGeneratorFunction` constructors.

use std::sync::Arc;

use super::call::ordinary_function_create;
use super::eval::{line_col, register_script};
use crate::agent::Agent;
use crate::compiler;
use crate::error::JsResult;
use crate::function::{make_constructor, set_function_name};
use crate::object::{Property, PropertyKey};
use crate::ops::{get_prototype_from_constructor, to_string};
use crate::value::{Object, Value};

/// The _kind_ argument of CreateDynamicFunction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DynamicFunctionKind {
    Normal,
    Generator,
    Async,
    AsyncGenerator,
}

/// CreateDynamicFunction(ctor, newTarget, kind, paramArgs, bodyArg) (§20.2.1.1.1)
pub fn create_dynamic_function(
    agent: &mut Agent,
    ctor: Object,
    new_target: Option<Object>,
    kind: DynamicFunctionKind,
    param_args: &[Value],
    body_arg: Value,
) -> JsResult<Object> {
    // 1. If newTarget is undefined, set newTarget to ctor.
    let new_target = new_target.unwrap_or(ctor);
    // 2-5. The prefix, grammars and fallbackProto for kind.
    let (prefix, fallback_proto, is_async, is_generator) = match kind {
        DynamicFunctionKind::Normal => ("function", "%Function.prototype%", false, false),
        DynamicFunctionKind::Generator => {
            ("function*", "%GeneratorFunction.prototype%", false, true)
        }
        DynamicFunctionKind::Async => ("async function", "%AsyncFunction.prototype%", true, false),
        DynamicFunctionKind::AsyncGenerator => (
            "async function*",
            "%AsyncGeneratorFunction.prototype%",
            true,
            true,
        ),
    };
    // 6-8. Let paramStrings be the result of ? ToString on each element of paramArgs.
    let mut param_strings = Vec::with_capacity(param_args.len());
    for &arg in param_args {
        let s = to_string(agent, arg)?;
        param_strings.push(agent.str_lossy(s).to_string());
    }
    // 9. Let bodyString be ? ToString(bodyArg).
    let body = to_string(agent, body_arg)?;
    let body_string = agent.str_lossy(body).to_string();
    // 10. Let currentRealm be the current Realm Record.
    let current_realm = agent.current_realm();
    // 11. Perform ? HostEnsureCanCompileStrings(currentRealm, paramStrings, bodyString, false).
    //     (No host hook restricts compilation yet.)
    // 12-13. Let paramString be the paramStrings joined with ",".
    let param_string = param_strings.join(",");
    // 14. Let bodyParseString be the string-concatenation of 0x000A (LINE FEED), bodyString, and 0x000A (LINE FEED).
    let body_parse_string = format!("\n{body_string}\n");
    // 15-16. Let sourceString be prefix, " anonymous(", paramString, LF, ") {", bodyParseString and "}".
    let source_string = format!("{prefix} anonymous({param_string}\n) {{{body_parse_string}}}");
    // 17-24. Parse the parameters and the body alone, then sourceText as exprGrammar;
    //        any List of errors is a SyntaxError.
    let f = match kahvi_parser::parse_dynamic_function(
        &param_string,
        &body_parse_string,
        &source_string,
        is_async,
        is_generator,
    ) {
        Ok(f) => f,
        Err(e) => {
            let (line, col) = line_col(&source_string, e.pos);
            let msg = format!("{} ({}:{})", e.message, line, col);
            return Err(agent.throw_syntax_error(&msg));
        }
    };
    // 25. Let funcProto be ? GetPrototypeFromConstructor(newTarget, fallbackProto).
    let func_proto = get_prototype_from_constructor(agent, new_target, fallback_proto)?;
    // 26. Let envRecord be currentRealm.[[GlobalEnv]].
    let env = agent.realm(current_realm).global_env;
    // 27. Let privateEnv be null.
    // 28. Let F be OrdinaryFunctionCreate(funcProto, sourceText, params, body, non-lexical-this, envRecord, privateEnv).
    let source: Arc<str> = Arc::from(source_string);
    let script = register_script(agent, current_realm, source.clone(), "anonymous");
    let code = compiler::compile_dynamic_function(agent, &f, source, script);
    let func = ordinary_function_create(agent, func_proto, code, env, None);
    // 29. Perform SetFunctionName(F, "anonymous").
    let anonymous = agent.key("anonymous");
    set_function_name(agent, func, anonymous, None);
    // 30-31. If kind is generator or async-generator, then
    if is_generator {
        // a. Let prototype be OrdinaryObjectCreate(%GeneratorPrototype%) or %AsyncGeneratorPrototype%.
        let intrinsic = if is_async {
            "%AsyncGeneratorPrototype%"
        } else {
            "%GeneratorPrototype%"
        };
        let proto_proto = agent.intrinsic(intrinsic);
        let prototype = agent.ordinary_object_create(Some(proto_proto));
        // b. Perform ! DefinePropertyOrThrow(F, "prototype", PropertyDescriptor { [[Value]]: prototype,
        //    [[Writable]]: true, [[Enumerable]]: false, [[Configurable]]: false }).
        let key = PropertyKey::String(agent.names.prototype);
        agent.obj_mut(func).properties.insert(
            key,
            Property::Data {
                value: Value::Object(prototype),
                writable: true,
                enumerable: false,
                configurable: false,
            },
        );
    } else if kind == DynamicFunctionKind::Normal {
        // 32. Else if kind is normal, perform MakeConstructor(F).
        make_constructor(agent, func, true, None);
    }
    // 33. NOTE: Functions whose kind is async are not constructable and have no "prototype" property.
    // 34. Return F.
    Ok(func)
}
