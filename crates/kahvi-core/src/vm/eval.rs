//! Script evaluation (§16.1.6 ScriptEvaluation) and `eval` (§19.2.1 PerformEval).

use std::sync::Arc;

use kahvi_parser::{ParseOptions, ast};

use super::{Frame, FrameResult, ScriptRecord, instantiate, run_frame};
use crate::agent::{Agent, ExecutionContext};
use crate::compiler;
use crate::env::Environment;
use crate::error::JsResult;
use crate::object::ObjectKind;
use crate::realm::RealmId;
use crate::value::Value;
use crate::{JsError, ScriptOrModule};

/// A parsed script ready to evaluate (ParseScript's Script Record).
pub struct ParsedScript {
    pub id: u32,
    pub code: Arc<compiler::FunctionCode>,
}

fn syntax_error(agent: &mut Agent, e: &kahvi_parser::SyntaxError, source: &str) -> JsError {
    let (line, col) = line_col(source, e.pos);
    let msg = format!("{} ({}:{})", e.message, line, col);
    agent.throw_syntax_error(&msg)
}

/// 1-based line and column of a byte offset.
pub fn line_col(source: &str, pos: usize) -> (usize, usize) {
    let before = &source[..pos.min(source.len())];
    let line = before.matches('\n').count() + 1;
    let col = before
        .rfind('\n')
        .map(|i| before.len() - i)
        .unwrap_or(before.len() + 1);
    (line, col)
}

pub(super) fn register_script(
    agent: &mut Agent,
    realm: RealmId,
    source: Arc<str>,
    name: &str,
    collectable: bool,
) -> u32 {
    if collectable {
        // A freed record of the same realm and name: whatever might still
        // name the old one (a dynamic import's referrer) sees no difference.
        if let Some(id) = agent
            .gc
            .free_scripts
            .get_mut(&(realm.0, name.to_string()))
            .and_then(|v| v.pop())
        {
            agent.scripts[id as usize].source = source;
            return id;
        }
    }
    agent.scripts.push(ScriptRecord {
        realm,
        source,
        name: name.to_string(),
        module: None,
        collectable,
    });
    (agent.scripts.len() - 1) as u32
}

/// ParseScript(sourceText, realm, hostDefined) (§16.1.5); a SyntaxError is
/// thrown as a SyntaxError object of `realm`.
pub fn parse_script(
    agent: &mut Agent,
    source: &str,
    name: &str,
    realm: RealmId,
) -> JsResult<ParsedScript> {
    let opts = ParseOptions {
        lazy: true,
        ..ParseOptions::default()
    };
    let ast = match kahvi_parser::parse_script(source, &opts) {
        Ok(ast) => ast,
        Err(e) => {
            agent.contexts.push(ExecutionContext {
                function: None,
                realm,
                script_or_module: None,
            });
            let err = syntax_error(agent, &e, source);
            agent.contexts.pop();
            return Err(err);
        }
    };
    let source: Arc<str> = Arc::from(source);
    let id = register_script(agent, realm, source.clone(), name, false);
    let code = compiler::compile_script(agent, &ast, source, id);
    Ok(ParsedScript { id, code })
}

/// ScriptEvaluation(scriptRecord) (§16.1.6)
pub fn evaluate_script(agent: &mut Agent, script: &ParsedScript) -> JsResult<Value> {
    let realm = agent.scripts[script.id as usize].realm;
    let global_env = agent.realm(realm).global_env;
    // 2-9. scriptContext with the global environment.
    agent.contexts.push(ExecutionContext {
        function: None,
        realm,
        script_or_module: Some(ScriptOrModule::Script(script.id)),
    });
    let result = (|| {
        // 12. Let result be Completion(GlobalDeclarationInstantiation(script, globalEnv)).
        let annex_b =
            instantiate::global_declaration_instantiation(agent, &script.code, global_env)?;
        // 13. If result is a normal completion, set result to Completion(Evaluation of script).
        let mut frame = Frame::new(script.code.clone(), global_env, None, None);
        frame.annex_b = annex_b;
        match run_frame(agent, &mut frame)? {
            FrameResult::Return(v) => Ok(v),
            _ => Ok(Value::Undefined),
        }
    })();
    // 15. Remove scriptContext from the execution context stack.
    agent.contexts.pop();
    result
}

/// PerformEval(x, strictCaller, direct) (§19.2.1.1). `caller` is the frame
/// of a direct eval.
pub fn perform_eval(
    agent: &mut Agent,
    x: Value,
    strict_caller: bool,
    caller: Option<&mut Frame>,
) -> JsResult<Value> {
    // 2. If x is not a String, return x.
    let Value::String(s) = x else { return Ok(x) };
    let (source, lone_surrogates) = source_text(agent.str(s));
    let direct = caller.is_some();
    // 6-7. What the eval code may contain depends on the enclosing function.
    let mut opts = ParseOptions {
        strict: direct && strict_caller,
        lone_surrogates,
        ..ParseOptions::default()
    };
    let mut caller_state = None;
    if let Some(frame) = caller {
        let this_env = agent.get_this_environment(frame.env);
        if let Some(data) = this_env.function_data(agent) {
            let f = data.function_object;
            opts.allow_new_target = true;
            if let ObjectKind::Function(e) = &agent.obj(f).kind {
                opts.allow_super_property = e.home_object.is_some();
                opts.allow_super_call =
                    e.constructor_kind == crate::function::ConstructorKind::Derived;
                opts.in_class_field_init = e.class_field_initializer_name.is_some();
                opts.allow_arguments = !opts.in_class_field_init;
            }
        }
        let mut names = Vec::new();
        let mut p = frame.private_env.clone();
        while let Some(env) = p {
            for n in &env.names {
                names.push(
                    agent
                        .str_lossy(n.description)
                        .trim_start_matches('#')
                        .to_string(),
                );
            }
            p = env.outer.clone();
        }
        opts.private_names = names;
        caller_state = Some((
            frame.env,
            frame.var_env,
            frame.private_env.clone(),
            frame.code.script,
        ));
    }
    let realm = agent.current_realm();
    let ast: ast::Script = match kahvi_parser::parse_script(&source, &opts) {
        Ok(a) => a,
        Err(e) => return Err(syntax_error(agent, &e, &source)),
    };
    // 13. strictEval.
    let strict_eval = ast.strict;
    let source: Arc<str> = Arc::from(source.as_str());
    let id = register_script(agent, realm, source.clone(), "eval", true);
    let code = compiler::compile_eval(agent, &ast, source, id);
    // 16-17. The environments.
    let (lex_env, mut var_env, private_env, script) = match caller_state {
        Some((env, var_env, private_env, script)) => (
            Environment::new_declarative(agent, Some(env)),
            var_env,
            private_env,
            script,
        ),
        None => {
            let global = agent.realm(realm).global_env;
            (
                Environment::new_declarative(agent, Some(global)),
                global,
                None,
                id,
            )
        }
    };
    // 18. If strictEval is true, set varEnv to lexEnv.
    if strict_eval {
        var_env = lex_env;
    }
    // 19-27. evalContext.
    agent.contexts.push(ExecutionContext {
        function: None,
        realm,
        script_or_module: Some(ScriptOrModule::Script(script)),
    });
    let result = (|| {
        // 28. Let result be Completion(EvalDeclarationInstantiation(body, varEnv, lexEnv, privateEnv, strictEval)).
        let annex_b = instantiate::eval_declaration_instantiation(
            agent,
            &code,
            var_env,
            lex_env,
            private_env.clone(),
            strict_eval,
        )?;
        // 29. Evaluation of body.
        let mut frame = Frame::new(code.clone(), lex_env, private_env, None);
        frame.var_env = var_env;
        frame.annex_b = annex_b;
        match run_frame(agent, &mut frame)? {
            FrameResult::Return(v) => Ok(v),
            _ => Ok(Value::Undefined),
        }
    })();
    agent.contexts.pop();
    result
}

/// Source text for the parser, which reads UTF-8: each lone surrogate
/// becomes U+FFFD, recorded with its byte offset and code unit so literals
/// keep the code unit itself.
fn source_text(units: &[u16]) -> (String, Vec<(usize, u16)>) {
    let mut text = String::with_capacity(units.len());
    let mut lone = Vec::new();
    for r in char::decode_utf16(units.iter().copied()) {
        match r {
            Ok(c) => text.push(c),
            Err(e) => {
                lone.push((text.len(), e.unpaired_surrogate()));
                text.push(char::REPLACEMENT_CHARACTER);
            }
        }
    }
    (text, lone)
}
