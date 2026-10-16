//! Declaration instantiation: FunctionDeclarationInstantiation (§10.2.11),
//! BlockDeclarationInstantiation (§14.2.3), GlobalDeclarationInstantiation
//! (§16.1.7) and EvalDeclarationInstantiation (§19.2.1.3), plus the global
//! environment record methods they use (§9.1.1.4).

use super::Frame;
use super::call::create_closure;
use crate::agent::Agent;
use crate::compiler::{FunctionCode, ScopeInfo};
use crate::env::{EnvId, Environment};
use crate::error::JsResult;
use crate::function::{ThisMode, create_mapped_arguments_object, create_unmapped_arguments_object};
use crate::object::PropertyDescriptor;
use crate::ops;
use crate::value::{JsString, Object, Value};

/// FunctionDeclarationInstantiation steps 1-23 (before the parameters are
/// bound by the function's own code).
pub(super) fn function_declaration_instantiation(
    agent: &mut Agent,
    frame: &mut Frame,
    func: Object,
    args: &[Value],
) -> JsResult<()> {
    let code = frame.code.clone();
    let callee_env = frame.env;
    let strict = code.strict;
    let params = &code.params;
    let arguments = agent.names.arguments;
    // 12-14. funcNames: the var-scoped function declarations (code.decls.functions).
    // 15-18. argumentsObjectNeeded.
    let this_mode = super::this_mode_of(agent, func).unwrap_or(ThisMode::Strict);
    // (An arguments object nothing can observe is skipped.)
    let shadowed = !params.has_expressions
        && (code.decls().functions.iter().any(|(n, _)| *n == arguments)
            || code.decls().lexical.iter().any(|(n, _)| *n == arguments));
    let arguments_needed = (code.uses_arguments || code.has_direct_eval)
        && this_mode != ThisMode::Lexical
        && !params.names.contains(&arguments)
        && !shadowed;
    if code.locals.is_some() {
        // The bindings are frame locals: callee_env gets none (and with
        // no parameter expressions it is env).
        frame.pending.env_record = Some(callee_env);
        frame.pending.arguments_bound = false;
        return Ok(());
    }
    // 19-20. The environment for the parameters.
    let env_record = if strict || !params.has_expressions {
        callee_env
    } else {
        // 20.c Let env be NewDeclarativeEnvironment(calleeEnv).
        let env = Environment::new_declarative(agent, Some(callee_env));
        frame.env = env;
        env
    };
    // (Room for the bindings steps 21-34 create here, so the list grows once.)
    if let Environment::Declarative { bindings, .. } = agent.env_mut(env_record) {
        let d = code.decls();
        let mut n = params.names.len() + 1;
        if !params.has_expressions {
            n += d.var_names.len() + d.functions.len();
            if strict {
                n += d.lexical.len();
            }
        }
        bindings.reserve(n);
    }
    // 21. For each String paramName of parameterNames, do
    for &name in &params.names {
        // a. Let alreadyDeclared be ! env.HasBinding(paramName).
        //    (env is new, so only a duplicate name is already declared.)
        if !params.has_duplicates {
            // c.i Perform ! env.CreateMutableBinding(paramName, false).
            env_record.create_new_mutable_binding(agent, name, None);
            continue;
        }
        if !env_record.has_binding(agent, name)? {
            // c.i Perform ! env.CreateMutableBinding(paramName, false).
            env_record.create_mutable_binding(agent, name, false)?;
            // c.ii If hasDuplicates is true, then perform ! env.InitializeBinding(paramName, undefined).
            if params.has_duplicates {
                env_record.initialize_binding(agent, name, Value::Undefined)?;
            }
        }
    }
    // 22. If argumentsObjectNeeded is true, then
    if arguments_needed {
        // a-b. Mapped arguments only for sloppy functions with simple parameter lists.
        let ao = if strict || !params.simple {
            create_unmapped_arguments_object(agent, args)?
        } else {
            create_mapped_arguments_object(agent, func, &params.names, args, env_record)?
        };
        // c-d. Strict: an immutable binding; otherwise mutable.
        if strict {
            env_record.create_immutable_binding(agent, arguments, false)?;
        } else {
            env_record.create_mutable_binding(agent, arguments, false)?;
        }
        // e. Perform ! env.InitializeBinding("arguments", ao).
        env_record.initialize_binding(agent, arguments, Value::Object(ao))?;
        // f. Let parameterBindings be the list-concatenation of parameterNames and « "arguments" ».
    }
    frame.pending.env_record = Some(env_record);
    frame.pending.arguments_bound = arguments_needed;
    Ok(())
}

/// FunctionDeclarationInstantiation steps 27-36, after the parameters are
/// bound (the `InstantiateFunctionBody` instruction).
pub(super) fn function_body_instantiation(agent: &mut Agent, frame: &mut Frame) -> JsResult<()> {
    let code = frame.code.clone();
    let Some(env_record) = frame.pending.env_record else {
        // Not function code (a default constructor run directly): nothing to do.
        return Ok(());
    };
    if code.locals.is_some() {
        // 27. The var bindings, and 34. the lexical ones, are frame locals
        //     (the frame starts vars at undefined); there are no functions
        //     to initialize.
        frame.var_env = env_record;
        frame.env = env_record;
        frame.pending.env_record = None;
        return Ok(());
    }
    let arguments = agent.names.arguments;
    let arguments_bound = std::mem::take(&mut frame.pending.arguments_bound);
    let is_param_binding =
        |n: JsString| code.params.names.contains(&n) || (arguments_bound && n == arguments);
    let decls = code.decls();
    let is_func_name = |n: JsString| decls.functions.iter().any(|&(f, _)| f == n);
    // VarDeclaredNames includes the function declarations' names.
    let var_names = decls.var_names.iter().copied().chain(
        decls
            .functions
            .iter()
            .map(|&(n, _)| n)
            .filter(|n| !decls.var_names.contains(n)),
    );
    // instantiatedVarNames: the names varEnv has bindings for. With one
    // environment for parameters and vars, that starts as parameterBindings.
    let var_env = if !code.params.has_expressions {
        // 27. NOTE: Only a single Environment Record is needed for the parameters and top-level vars.
        // b. Let instantiatedVarNames be a copy of the List parameterBindings.
        // c. For each element n of varNames, do
        for n in var_names {
            // i. If instantiatedVarNames does not contain n, then
            if !env_record.has_binding(agent, n)? {
                // 1-3. Append n; ! env.CreateMutableBinding(n, false); ! env.InitializeBinding(n, undefined).
                env_record.create_new_mutable_binding(agent, n, Some(Value::Undefined));
            }
        }
        env_record
    } else {
        // 28. A separate Environment Record for the body's var declarations.
        let var_env = Environment::new_declarative(agent, Some(env_record));
        for n in var_names {
            if !var_env.has_binding(agent, n)? {
                var_env.create_mutable_binding(agent, n, false)?;
                // A var with the same name as a parameter starts with the parameter's value.
                let initial = if !is_param_binding(n) || is_func_name(n) {
                    Value::Undefined
                } else {
                    env_record.get_binding_value(agent, n, false)?
                };
                var_env.initialize_binding(agent, n, initial)?;
            }
        }
        var_env
    };
    frame.var_env = var_env;
    // 30.a If the host supports Block-Level Function Declarations Web Legacy
    //      Compatibility Semantics (code.decls.annex_b_functions is empty for strict code), then
    //   i. For each FunctionDeclaration f directly contained in the StatementList of a Block,
    //      CaseClause, or DefaultClause, such that replacing it with a `var F` would not produce
    //      any Early Errors (the compiler's static check), do
    for &name in &decls.annex_b_functions {
        // 2. ... and F is not an element of parameterNames (the formals, and "arguments"
        //    when there is an arguments object), then
        if is_param_binding(name) {
            continue;
        }
        // b. If instantiatedVarNames does not contain F and F is not "arguments", then
        if !var_env.has_binding(agent, name)? && name != arguments {
            // i. Perform ! varEnv.CreateMutableBinding(F, false).
            var_env.create_mutable_binding(agent, name, false)?;
            // ii. Perform ! varEnv.InitializeBinding(F, undefined).
            var_env.initialize_binding(agent, name, Value::Undefined)?;
            // iii. Append F to instantiatedVarNames (varEnv now has its binding).
        }
        // c. When the FunctionDeclaration f is evaluated, set varEnv's F (Op::AnnexBFunction).
        frame.annex_b.push(name);
    }
    // 30-31. Non-strict functions put top-level lexical declarations in their own environment.
    //     (That environment only matters to a direct eval's conflict check
    //     when it has bindings, so with no lexical declarations it is varEnv.)
    let lex_env = if code.strict || decls.lexical.is_empty() {
        var_env
    } else {
        Environment::new_declarative(agent, Some(var_env))
    };
    frame.env = lex_env;
    // 34. Lexical declarations.
    for &(name, is_const) in &decls.lexical {
        if is_const {
            lex_env.create_immutable_binding(agent, name, true)?;
        } else {
            lex_env.create_mutable_binding(agent, name, false)?;
        }
    }
    // 36. For each Parse Node f of functionsToInitialize, do
    let private_env = frame.private_env.clone();
    for &(name, index) in &decls.functions {
        let fo = create_closure(
            agent,
            lex_env,
            private_env.clone(),
            code.functions[index as usize].clone(),
            None,
        )?;
        // c. Perform ! varEnv.SetMutableBinding(fn, fo, false).
        var_env.set_mutable_binding(agent, name, Value::Object(fo), false)?;
    }
    frame.pending.env_record = None;
    Ok(())
}

/// BlockDeclarationInstantiation(code, env) (§14.2.3)
pub(super) fn block_declaration_instantiation(
    agent: &mut Agent,
    frame: &Frame,
    code: &FunctionCode,
    scope: &ScopeInfo,
    env: EnvId,
) -> JsResult<()> {
    // 3.a-b Lexical declarations: const is immutable, everything else mutable.
    for &(name, is_const) in &scope.lexical {
        if is_const {
            env.create_immutable_binding(agent, name, true)?;
        } else {
            env.create_mutable_binding(agent, name, false)?;
        }
    }
    // 3.b.ii Function declarations: InstantiateFunctionObject and initialize.
    for &(name, index) in &scope.functions {
        if !env.has_binding(agent, name)? {
            env.create_mutable_binding(agent, name, false)?;
        }
        let fo = create_closure(
            agent,
            env,
            frame.private_env.clone(),
            code.functions[index as usize].clone(),
            None,
        )?;
        env.initialize_binding(agent, name, Value::Object(fo))?;
    }
    Ok(())
}

// ---- the global environment record --------------------------------------------------------

fn global_parts(agent: &Agent, env: EnvId) -> Option<(EnvId, EnvId, Object)> {
    match agent.env(env) {
        Environment::Global {
            object_record,
            declarative_record,
            ..
        } => match agent.env(*object_record) {
            Environment::Object { binding_object, .. } => {
                Some((*object_record, *declarative_record, *binding_object))
            }
            _ => None,
        },
        _ => None,
    }
}

pub(super) fn is_global(agent: &Agent, env: EnvId) -> bool {
    matches!(agent.env(env), Environment::Global { .. })
}

/// HasLexicalDeclaration(N)
fn has_lexical_declaration(agent: &mut Agent, env: EnvId, name: JsString) -> JsResult<bool> {
    let (_, dcl, _) = global_parts(agent, env).expect("global environment");
    dcl.has_binding(agent, name)
}

/// HasRestrictedGlobalProperty(N)
fn has_restricted_global_property(agent: &mut Agent, env: EnvId, name: JsString) -> JsResult<bool> {
    let (_, _, global) = global_parts(agent, env).expect("global environment");
    let key = agent.key_from_string(name);
    let existing = global.internal_get_own_property(agent, key)?;
    Ok(matches!(existing, Some(d) if d.configurable != Some(true)))
}

/// CanDeclareGlobalVar(N)
fn can_declare_global_var(agent: &mut Agent, env: EnvId, name: JsString) -> JsResult<bool> {
    let (_, _, global) = global_parts(agent, env).expect("global environment");
    let key = agent.key_from_string(name);
    if ops::has_own_property(agent, global, key)? {
        return Ok(true);
    }
    ops::is_extensible(agent, global)
}

/// CanDeclareGlobalFunction(N)
fn can_declare_global_function(agent: &mut Agent, env: EnvId, name: JsString) -> JsResult<bool> {
    let (_, _, global) = global_parts(agent, env).expect("global environment");
    let key = agent.key_from_string(name);
    match global.internal_get_own_property(agent, key)? {
        None => ops::is_extensible(agent, global),
        Some(d) if d.configurable == Some(true) => Ok(true),
        Some(d) => {
            Ok(d.is_data_descriptor() && d.writable == Some(true) && d.enumerable == Some(true))
        }
    }
}

/// CreateGlobalVarBinding(N, D)
fn create_global_var_binding(
    agent: &mut Agent,
    env: EnvId,
    name: JsString,
    deletable: bool,
) -> JsResult<()> {
    let (obj_rec, _, global) = global_parts(agent, env).expect("global environment");
    let key = agent.key_from_string(name);
    let has_property = ops::has_own_property(agent, global, key)?;
    let extensible = ops::is_extensible(agent, global)?;
    if !has_property && extensible {
        obj_rec.create_mutable_binding(agent, name, deletable)?;
        obj_rec.initialize_binding(agent, name, Value::Undefined)?;
    }
    Ok(())
}

/// CreateGlobalFunctionBinding(N, V, D)
fn create_global_function_binding(
    agent: &mut Agent,
    env: EnvId,
    name: JsString,
    value: Value,
    deletable: bool,
) -> JsResult<()> {
    let (_, _, global) = global_parts(agent, env).expect("global environment");
    let key = agent.key_from_string(name);
    let existing = global.internal_get_own_property(agent, key)?;
    let desc = match existing {
        None => PropertyDescriptor::data(value, true, true, deletable),
        Some(d) if d.configurable == Some(true) => {
            PropertyDescriptor::data(value, true, true, deletable)
        }
        Some(_) => PropertyDescriptor {
            value: Some(value),
            ..Default::default()
        },
    };
    ops::define_property_or_throw(agent, global, key, desc)?;
    ops::set(agent, global, key, value, false)?;
    Ok(())
}

fn redeclaration(agent: &mut Agent, name: JsString) -> crate::error::JsError {
    let msg = format!(
        "Identifier '{}' has already been declared",
        agent.str_lossy(name)
    );
    agent.throw_syntax_error(&msg)
}

/// GlobalDeclarationInstantiation(script, env) (§16.1.7). Returns the
/// Annex B.3.2 function names that got a var binding.
pub(crate) fn global_declaration_instantiation(
    agent: &mut Agent,
    code: &FunctionCode,
    env: EnvId,
) -> JsResult<Vec<JsString>> {
    let decls = code.decls();
    let func_names: Vec<JsString> = decls.functions.iter().map(|(n, _)| *n).collect();
    // 3. For each element name of lexNames, do
    for &(name, _) in &decls.lexical {
        // a. If HasLexicalDeclaration(envRecord, name) is true, throw a SyntaxError exception.
        // b-d. If ? HasRestrictedGlobalProperty(envRecord, name) is true, throw a SyntaxError
        //      exception (global var and function bindings are non-configurable, except
        //      those introduced by non-strict direct eval).
        if has_lexical_declaration(agent, env, name)?
            || has_restricted_global_property(agent, env, name)?
        {
            return Err(redeclaration(agent, name));
        }
    }
    // 4. For each element name of varNames, do
    for &name in decls.var_names.iter().chain(func_names.iter()) {
        // a. If env.HasLexicalDeclaration(name) is true, throw a SyntaxError exception.
        if has_lexical_declaration(agent, env, name)? {
            return Err(redeclaration(agent, name));
        }
    }
    // 8. For each functionsToInitialize: fnDefinable = ? env.CanDeclareGlobalFunction(fn).
    for &name in &func_names {
        if !can_declare_global_function(agent, env, name)? {
            let msg = format!("cannot declare global function '{}'", agent.str_lossy(name));
            return Err(agent.throw_type_error(&msg));
        }
    }
    // 10. declaredVarNames.
    let mut declared_var_names: Vec<JsString> = Vec::new();
    for &name in &decls.var_names {
        if func_names.contains(&name) {
            continue;
        }
        if !can_declare_global_var(agent, env, name)? {
            let msg = format!("cannot declare global variable '{}'", agent.str_lossy(name));
            return Err(agent.throw_type_error(&msg));
        }
        if !declared_var_names.contains(&name) {
            declared_var_names.push(name);
        }
    }
    // 12. If the host supports Block-Level Function Declarations Web Legacy Compatibility
    //     Semantics, then (a-b. If strict is false: annex_b_functions is empty otherwise)
    let mut hoisted: Vec<JsString> = Vec::new();
    // i. Let declaredFunctionOrVarNames be the list-concatenation of declaredFunctionNames
    //    and declaredVarNames.
    let mut declared_func_or_var_names: Vec<JsString> = func_names
        .iter()
        .chain(declared_var_names.iter())
        .copied()
        .collect();
    // ii. For each FunctionDeclaration f directly contained in the StatementList of a Block,
    //     CaseClause, or DefaultClause, such that replacing it with a `var F` would not
    //     produce any Early Errors for script (the compiler's static check), do
    for &name in &decls.annex_b_functions {
        // a. If env.HasLexicalDeclaration(F) is false, then
        if has_lexical_declaration(agent, env, name)? {
            continue;
        }
        // i. Let fnDefinable be ? env.CanDeclareGlobalVar(F).
        // ii. If fnDefinable is true, then
        if !can_declare_global_var(agent, env, name)? {
            continue;
        }
        // 2. If declaredFunctionOrVarNames does not contain F, then
        if !declared_func_or_var_names.contains(&name) {
            // i. Perform ? env.CreateGlobalVarBinding(F, false).
            create_global_var_binding(agent, env, name, false)?;
            // ii. Append F to declaredFunctionOrVarNames.
            declared_func_or_var_names.push(name);
        }
        // 3. When the FunctionDeclaration f is evaluated, set the global F (Op::AnnexBFunction).
        hoisted.push(name);
    }
    // 15. Lexical declarations.
    for &(name, is_const) in &decls.lexical {
        if is_const {
            env.create_immutable_binding(agent, name, true)?;
        } else {
            env.create_mutable_binding(agent, name, false)?;
        }
    }
    // 16. For each Parse Node f of functionsToInitialize, do
    for &(name, index) in &decls.functions {
        let fo = create_closure(
            agent,
            env,
            None,
            code.functions[index as usize].clone(),
            None,
        )?;
        create_global_function_binding(agent, env, name, Value::Object(fo), false)?;
    }
    // 17. For each String vn of declaredVarNames, do
    for name in declared_var_names {
        create_global_var_binding(agent, env, name, false)?;
    }
    Ok(hoisted)
}

/// EvalDeclarationInstantiation(body, varEnv, lexEnv, privateEnv, strict)
/// (§19.2.1.3). Returns the Annex B.3.2 function names that got a var binding.
pub(super) fn eval_declaration_instantiation(
    agent: &mut Agent,
    code: &FunctionCode,
    var_env: EnvId,
    lex_env: EnvId,
    private_env: Option<std::sync::Arc<super::PrivateEnv>>,
    strict: bool,
) -> JsResult<Vec<JsString>> {
    let decls = code.decls();
    let func_names: Vec<JsString> = decls.functions.iter().map(|(n, _)| *n).collect();
    let var_names: Vec<JsString> = decls
        .var_names
        .iter()
        .chain(func_names.iter())
        .copied()
        .collect();
    // 3. If strict is false, then
    if !strict {
        // a. If varEnv is a Global Environment Record, then
        if is_global(agent, var_env) {
            for &name in &var_names {
                // i.1 If varEnv.HasLexicalDeclaration(name) is true, throw a SyntaxError exception.
                if has_lexical_declaration(agent, var_env, name)? {
                    return Err(redeclaration(agent, name));
                }
            }
        }
        // b-d. Check the environments between lexEnv and varEnv.
        let mut this_env = lex_env.outer(agent);
        while let Some(e) = this_env {
            if e == var_env {
                break;
            }
            // i. If thisEnv is not an Object Environment Record, then
            if !matches!(agent.env(e), Environment::Object { .. }) {
                for &name in &var_names {
                    // a. If ! thisEnv.HasBinding(name) is true, then
                    //   i. (Annex B.3.4) If thisEnv is not the Environment Record for a
                    //      Catch clause, throw a SyntaxError exception.
                    if e.has_binding(agent, name)? && !Environment::is_catch(agent, e) {
                        return Err(redeclaration(agent, name));
                    }
                }
            }
            this_env = e.outer(agent);
        }
    }
    let global = is_global(agent, var_env);
    // 10. functionsToInitialize: if varEnv is global, check CanDeclareGlobalFunction.
    if global {
        for &name in &func_names {
            if !can_declare_global_function(agent, var_env, name)? {
                let msg = format!("cannot declare global function '{}'", agent.str_lossy(name));
                return Err(agent.throw_type_error(&msg));
            }
        }
    }
    // 11. declaredVarNames.
    let mut declared_var_names: Vec<JsString> = Vec::new();
    for &name in &decls.var_names {
        if func_names.contains(&name) {
            continue;
        }
        if global && !can_declare_global_var(agent, var_env, name)? {
            let msg = format!("cannot declare global variable '{}'", agent.str_lossy(name));
            return Err(agent.throw_type_error(&msg));
        }
        if !declared_var_names.contains(&name) {
            declared_var_names.push(name);
        }
    }
    // 13. If strict is false and the host supports Block-Level Function Declarations Web
    //     Legacy Compatibility Semantics (annex_b_functions is empty for strict code), then
    let mut hoisted: Vec<JsString> = Vec::new();
    // a. Let declaredFunctionOrVarNames be the list-concatenation of declaredFunctionNames
    //    and declaredVarNames.
    let mut declared_func_or_var_names: Vec<JsString> = func_names
        .iter()
        .chain(declared_var_names.iter())
        .copied()
        .collect();
    // b. For each FunctionDeclaration f directly contained in the StatementList of a Block,
    //    CaseClause, or DefaultClause, such that replacing it with a `var F` would not
    //    produce any Early Errors for body (the compiler's static check), do
    for &name in &decls.annex_b_functions {
        // 1. Let bindingExists be false.
        let mut binding_exists = false;
        // 2. Let thisEnv be lexEnv.
        let mut this_env = Some(lex_env);
        // 4. Repeat, while thisEnv is not varEnv,
        while let Some(e) = this_env {
            if e == var_env {
                break;
            }
            // a. If thisEnv is not an Object Environment Record, then
            //   i. If ! thisEnv.HasBinding(F) is true, then
            //     1. (Annex B.3.4) If thisEnv is not the Environment Record for a Catch
            //        clause, let bindingExists be true.
            if !matches!(agent.env(e), Environment::Object { .. })
                && e.has_binding(agent, name)?
                && !Environment::is_catch(agent, e)
            {
                binding_exists = true;
            }
            // b. Set thisEnv to thisEnv.[[OuterEnv]].
            this_env = e.outer(agent);
        }
        // 5. If bindingExists is false and varEnv is a Global Environment Record, then
        let fn_definable = if !binding_exists && global {
            // a. If varEnv.HasLexicalDeclaration(F) is false, then
            //   i. Let fnDefinable be ? varEnv.CanDeclareGlobalVar(F).
            // b. Else, let fnDefinable be false.
            !has_lexical_declaration(agent, var_env, name)?
                && can_declare_global_var(agent, var_env, name)?
        } else {
            // 6. Else, let fnDefinable be true.
            true
        };
        // 7. If bindingExists is false and fnDefinable is true, then
        if binding_exists || !fn_definable {
            continue;
        }
        // a. If declaredFunctionOrVarNames does not contain F, then
        if !declared_func_or_var_names.contains(&name) {
            if global {
                // i. If varEnv is a Global Environment Record, then
                //   1. Perform ? varEnv.CreateGlobalVarBinding(F, true).
                create_global_var_binding(agent, var_env, name, true)?;
            } else if !var_env.has_binding(agent, name)? {
                // ii. Else,
                //   1. Let bindingExists be ! varEnv.HasBinding(F).
                //   2. If bindingExists is false, then
                //     a. Perform ! varEnv.CreateMutableBinding(F, true).
                //     b. Perform ! varEnv.InitializeBinding(F, undefined).
                var_env.create_mutable_binding(agent, name, true)?;
                var_env.initialize_binding(agent, name, Value::Undefined)?;
            }
            // iii. Append F to declaredFunctionOrVarNames.
            declared_func_or_var_names.push(name);
        }
        // b. When the FunctionDeclaration f is evaluated, set varEnv's F (Op::AnnexBFunction).
        hoisted.push(name);
    }
    // 15. Lexical declarations go in lexEnv.
    for &(name, is_const) in &decls.lexical {
        if is_const {
            lex_env.create_immutable_binding(agent, name, true)?;
        } else {
            lex_env.create_mutable_binding(agent, name, false)?;
        }
    }
    // 16. For each Parse Node f of functionsToInitialize, do
    for &(name, index) in &decls.functions {
        let fo = create_closure(
            agent,
            lex_env,
            private_env.clone(),
            code.functions[index as usize].clone(),
            None,
        )?;
        if global {
            create_global_function_binding(agent, var_env, name, Value::Object(fo), true)?;
        } else if !var_env.has_binding(agent, name)? {
            var_env.create_mutable_binding(agent, name, true)?;
            var_env.initialize_binding(agent, name, Value::Object(fo))?;
        } else {
            var_env.set_mutable_binding(agent, name, Value::Object(fo), false)?;
        }
    }
    // 17. For each String vn of declaredVarNames, do
    for name in declared_var_names {
        if global {
            create_global_var_binding(agent, var_env, name, true)?;
        } else if !var_env.has_binding(agent, name)? {
            var_env.create_mutable_binding(agent, name, true)?;
            var_env.initialize_binding(agent, name, Value::Undefined)?;
        }
    }
    Ok(hoisted)
}
