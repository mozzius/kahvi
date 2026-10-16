//! Modules (§16.2): Source Text Module Records with their loading, linking
//! and evaluation (including top-level await), module namespace objects,
//! `import()` and `import.meta`. The host supplies module source text
//! through [`Agent::host_load_module`]; the engine never reads files.

use std::sync::Arc;

use kahvi_parser::ast;

use super::eval::{line_col, register_script};
use super::{Frame, async_function, create_closure, run_frame};
use crate::ScriptOrModule;
use crate::agent::{Agent, ExecutionContext};
use crate::compiler::{self, FunctionCode};
use crate::env::{EnvId, Environment};
use crate::error::{JsError, JsResult};
use crate::function::{builtin_slots, create_builtin_function};
use crate::object::namespace::NamespaceData;
use crate::object::{ObjectKind, Property, PropertyKey};
use crate::ops::promise::{PromiseCapability, new_promise_capability, perform_promise_then};
use crate::ops::{self, call};
use crate::realm::RealmId;
use crate::value::{Args, JsString, Object, Value};

/// What the host returns for a module it loaded.
pub struct LoadedModuleSource {
    /// Identifies the module: within a realm, the same key is the same module.
    pub key: String,
    pub source: String,
}

/// HostLoadImportedModule's loading step. Given the referrer's host-defined
/// name (a script's name, a module's key, or "" for a realm), the specifier
/// and the import attributes: the module's key and source, or an error message.
pub type HostLoadModule =
    fn(&mut Agent, &str, &str, &[(String, String)]) -> Result<LoadedModuleSource, String>;

/// A ModuleRequest Record: [[Specifier]] and [[Attributes]] (sorted by key).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ModuleRequest {
    pub specifier: JsString,
    pub attributes: Vec<(JsString, JsString)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModuleStatus {
    New,
    Unlinked,
    Linking,
    Linked,
    Evaluating,
    EvaluatingAsync,
    Evaluated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AsyncOrder {
    Unset,
    Order(u64),
    Done,
}

/// An ImportEntry's [[ImportName]].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ImportName {
    Name(JsString),
    NamespaceObject,
}

#[derive(Clone, Copy, Debug)]
struct ImportEntry {
    request: usize,
    import_name: ImportName,
    local_name: JsString,
}

/// An ExportEntry's [[ImportName]].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExportImport {
    None,
    Name(JsString),
    All,
    AllButDefault,
}

#[derive(Clone, Copy, Debug)]
struct ExportEntry {
    export_name: Option<JsString>,
    request: Option<usize>,
    import_name: ExportImport,
    local_name: Option<JsString>,
}

/// A Source Text Module Record (§16.2.1.6).
pub struct ModuleRecord {
    pub realm: RealmId,
    pub environment: Option<EnvId>,
    pub namespace: Option<Object>,
    /// [[HostDefined]]: the host's key for this module.
    pub host_defined: Box<str>,
    pub status: ModuleStatus,
    pub evaluation_error: Option<Value>,
    dfs_index: usize,
    dfs_ancestor_index: usize,
    pub requested_modules: Vec<ModuleRequest>,
    /// [[LoadedModules]]
    loaded_modules: Vec<(ModuleRequest, u32)>,
    cycle_root: Option<u32>,
    has_tla: bool,
    async_evaluation_order: AsyncOrder,
    top_level_capability: Option<PromiseCapability>,
    async_parent_modules: Vec<u32>,
    pending_async_dependencies: usize,
    code: Arc<FunctionCode>,
    import_meta: Option<Object>,
    /// For a JSON module (a default-export Synthetic Module Record): its value.
    synthetic_default: Option<Value>,
    import_entries: Vec<ImportEntry>,
    local_export_entries: Vec<ExportEntry>,
    indirect_export_entries: Vec<ExportEntry>,
    star_export_entries: Vec<ExportEntry>,
}

impl std::fmt::Debug for ModuleRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleRecord")
            .field("host_defined", &self.host_defined)
            .field("status", &self.status)
            .finish_non_exhaustive()
    }
}

fn m(agent: &Agent, id: u32) -> &ModuleRecord {
    &agent.modules[id as usize]
}

fn m_mut(agent: &mut Agent, id: u32) -> &mut ModuleRecord {
    &mut agent.modules[id as usize]
}

fn promise_object(p: Value) -> Object {
    match p {
        Value::Object(o) => o,
        _ => unreachable!("a promise capability's promise is an object"),
    }
}

fn throw_value(e: JsError) -> JsResult<Value> {
    match e {
        JsError::Throw(v) => Ok(v),
        other => Err(other),
    }
}

// ---- ParseModule -------------------------------------------------------------

fn export_name(agent: &mut Agent, n: &ast::ModuleExportName) -> JsString {
    match n {
        ast::ModuleExportName::Ident(s) => agent.string(s),
        ast::ModuleExportName::String(u) => agent.intern(u),
    }
}

fn request_index(
    agent: &mut Agent,
    requests: &mut Vec<ModuleRequest>,
    source: &[u16],
    attrs: &[ast::ImportAttribute],
) -> usize {
    let specifier = agent.intern(source);
    let mut attributes: Vec<(JsString, JsString)> = attrs
        .iter()
        .map(|a| (agent.intern(&a.key), agent.intern(&a.value)))
        .collect();
    attributes.sort_by(|a, b| agent.str(a.0).cmp(agent.str(b.0)));
    let request = ModuleRequest {
        specifier,
        attributes,
    };
    // ModuleRequests: duplicates (ModuleRequestsEqual) are not added again.
    match requests.iter().position(|r| *r == request) {
        Some(i) => i,
        None => {
            requests.push(request);
            requests.len() - 1
        }
    }
}

fn declared_names(agent: &mut Agent, s: &ast::Stmt) -> Vec<JsString> {
    let names: Vec<String> = match s {
        ast::Stmt::Var(d) => d
            .declarations
            .iter()
            .flat_map(|decl| {
                kahvi_parser::early::bound_names(&decl.target)
                    .into_iter()
                    .map(str::to_string)
            })
            .collect(),
        ast::Stmt::Function(f) => f.id.iter().map(|i| i.name.clone()).collect(),
        ast::Stmt::Class(c) => c.id.iter().map(|i| i.name.clone()).collect(),
        _ => Vec::new(),
    };
    names.iter().map(|n| agent.string(n)).collect()
}

/// ParseModule(sourceText, realm, hostDefined) (§16.2.1.7.1)
pub fn parse_module(agent: &mut Agent, source: &str, realm: RealmId, key: &str) -> JsResult<u32> {
    // 1. Let body be ParseText(sourceText, Module).
    // 2. If body is a List of errors, return body.
    let cache_key = crate::code_cache::key(source, true);
    let cached = cache_key.as_ref().and_then(crate::code_cache::lookup);
    let (body, preparse) = match kahvi_parser::parse_module_preparsed(source, true, cached.clone())
    {
        Ok(parsed) => parsed,
        Err(e) => {
            let (line, col) = line_col(source, e.pos);
            let msg = format!("{} ({}:{}:{})", e.message, key, line, col);
            agent.contexts.push(ExecutionContext {
                function: None,
                realm,
                script_or_module: None,
            });
            let err = agent.throw_syntax_error(&msg);
            agent.contexts.pop();
            return Err(err);
        }
    };
    let id = agent.modules.len() as u32;
    // 3. Let requestedModules be the ModuleRequests of body.
    let mut requests: Vec<ModuleRequest> = Vec::new();
    // 4. Let importEntries be the ImportEntries of body.
    let mut import_entries = Vec::new();
    // ExportEntries of body.
    let mut export_entries: Vec<ExportEntry> = Vec::new();
    let default = agent.string("default");
    let star_default = agent.string("*default*");
    for item in &body.body {
        match item {
            ast::ModuleItem::Import(d) => {
                let request = request_index(agent, &mut requests, &d.source, &d.attributes);
                if let Some(local) = &d.default {
                    let local_name = agent.string(&local.name);
                    import_entries.push(ImportEntry {
                        request,
                        import_name: ImportName::Name(default),
                        local_name,
                    });
                }
                if let Some(local) = &d.namespace {
                    let local_name = agent.string(&local.name);
                    import_entries.push(ImportEntry {
                        request,
                        import_name: ImportName::NamespaceObject,
                        local_name,
                    });
                }
                for spec in &d.named {
                    let imported = export_name(agent, &spec.imported);
                    let local_name = agent.string(&spec.local.name);
                    import_entries.push(ImportEntry {
                        request,
                        import_name: ImportName::Name(imported),
                        local_name,
                    });
                }
            }
            ast::ModuleItem::Export(e) => match e {
                ast::ExportDecl::Named {
                    specifiers,
                    source: None,
                    ..
                } => {
                    for spec in specifiers {
                        let local = export_name(agent, &spec.local);
                        let exported = export_name(agent, &spec.exported);
                        export_entries.push(ExportEntry {
                            export_name: Some(exported),
                            request: None,
                            import_name: ExportImport::None,
                            local_name: Some(local),
                        });
                    }
                }
                ast::ExportDecl::Named {
                    specifiers,
                    source: Some(src),
                    attributes,
                    ..
                } => {
                    let request = request_index(agent, &mut requests, src, attributes);
                    for spec in specifiers {
                        let imported = export_name(agent, &spec.local);
                        let exported = export_name(agent, &spec.exported);
                        export_entries.push(ExportEntry {
                            export_name: Some(exported),
                            request: Some(request),
                            import_name: ExportImport::Name(imported),
                            local_name: None,
                        });
                    }
                }
                ast::ExportDecl::All {
                    alias,
                    source,
                    attributes,
                    ..
                } => {
                    let request = request_index(agent, &mut requests, source, attributes);
                    let (export, import_name) = match alias {
                        Some(a) => (Some(export_name(agent, a)), ExportImport::All),
                        None => (None, ExportImport::AllButDefault),
                    };
                    export_entries.push(ExportEntry {
                        export_name: export,
                        request: Some(request),
                        import_name,
                        local_name: None,
                    });
                }
                ast::ExportDecl::Decl(s) => {
                    for n in declared_names(agent, s) {
                        export_entries.push(ExportEntry {
                            export_name: Some(n),
                            request: None,
                            import_name: ExportImport::None,
                            local_name: Some(n),
                        });
                    }
                }
                ast::ExportDecl::DefaultDecl(s) => {
                    let local = declared_names(agent, s)
                        .first()
                        .copied()
                        .unwrap_or(star_default);
                    export_entries.push(ExportEntry {
                        export_name: Some(default),
                        request: None,
                        import_name: ExportImport::None,
                        local_name: Some(local),
                    });
                }
                ast::ExportDecl::DefaultExpr(..) => export_entries.push(ExportEntry {
                    export_name: Some(default),
                    request: None,
                    import_name: ExportImport::None,
                    local_name: Some(star_default),
                }),
            },
            ast::ModuleItem::Stmt(_) => {}
        }
    }
    // 5. Let importedBoundNames be ImportedLocalNames(importEntries).
    // 6-10. Sort the export entries into local, indirect and star exports.
    let mut local_export_entries = Vec::new();
    let mut indirect_export_entries = Vec::new();
    let mut star_export_entries = Vec::new();
    for ee in export_entries {
        match ee.request {
            None => {
                let local = ee.local_name.expect("a local export has a local name");
                match import_entries.iter().find(|ie| ie.local_name == local) {
                    // i. If importedBoundNames does not contain ee.[[LocalName]], append ee to localExportEntries.
                    None => local_export_entries.push(ee),
                    // ii. Else, rewrite it as a re-export directly from the imported module.
                    Some(ie) => indirect_export_entries.push(ExportEntry {
                        export_name: ee.export_name,
                        request: Some(ie.request),
                        import_name: match ie.import_name {
                            ImportName::NamespaceObject => ExportImport::All,
                            ImportName::Name(n) => ExportImport::Name(n),
                        },
                        local_name: None,
                    }),
                }
            }
            Some(_) if ee.import_name == ExportImport::AllButDefault => {
                star_export_entries.push(ee)
            }
            Some(_) => indirect_export_entries.push(ee),
        }
    }
    let preparse = cached.unwrap_or_else(|| {
        let p = Arc::new(preparse);
        if let Some(key) = &cache_key {
            crate::code_cache::store(key, &p);
        }
        p
    });
    let source = crate::code_cache::shared_source(cache_key.as_ref(), source);
    let script = register_script(agent, realm, source.clone(), key, false);
    agent.scripts[script as usize].module = Some(id);
    agent.scripts[script as usize].preparse = Some(preparse);
    agent.scripts[script as usize].cache_key = cache_key;
    let code = compiler::compile_module(agent, &body, source, script);
    // 11. Let async be body Contains await.
    let has_tla = code.is_async;
    // 12. Return Source Text Module Record { ... }.
    agent.modules.push(ModuleRecord {
        realm,
        environment: None,
        namespace: None,
        host_defined: key.into(),
        status: ModuleStatus::New,
        evaluation_error: None,
        dfs_index: 0,
        dfs_ancestor_index: 0,
        requested_modules: requests,
        loaded_modules: Vec::new(),
        cycle_root: None,
        has_tla,
        async_evaluation_order: AsyncOrder::Unset,
        top_level_capability: None,
        async_parent_modules: Vec::new(),
        pending_async_dependencies: 0,
        code,
        import_meta: None,
        synthetic_default: None,
        import_entries,
        local_export_entries,
        indirect_export_entries,
        star_export_entries,
    });
    Ok(id)
}

/// ParseJSONModule(source) (§16.2.1.8.4) and CreateDefaultExportSyntheticModule(json).
fn parse_json_module(agent: &mut Agent, source: &str, realm: RealmId, key: &str) -> JsResult<u32> {
    // 1. Let json be ? Call(%JSON.parse%, undefined, « source »).
    let parse = agent.intrinsic_in(realm, "%JSON.parse%");
    let text = agent.string(source);
    let json = call(
        agent,
        Value::Object(parse),
        Value::Undefined,
        &[Value::String(text)],
    )?;
    // 2. Return CreateDefaultExportSyntheticModule(json).
    let id = parse_module(agent, "", realm, key)?;
    let default = agent.string("default");
    let rec = m_mut(agent, id);
    rec.synthetic_default = Some(json);
    rec.local_export_entries.push(ExportEntry {
        export_name: Some(default),
        request: None,
        import_name: ExportImport::None,
        local_name: Some(default),
    });
    Ok(id)
}

// ---- Loading -----------------------------------------------------------------

/// A Script Record, Cyclic Module Record or Realm Record that imports.
#[derive(Clone, Copy, Debug)]
pub enum Referrer {
    Script(u32),
    Module(u32),
    Realm(RealmId),
}

/// The GraphLoadingState Record of LoadRequestedModules (§16.2.1.5.1).
struct GraphLoadingState {
    capability: PromiseCapability,
    is_loading: bool,
    pending: usize,
    visited: Vec<u32>,
}

enum Payload<'a> {
    Graph(&'a mut GraphLoadingState),
    Dynamic(PromiseCapability),
}

/// AllImportAttributesSupported(attributes): the host supports only `type`.
fn all_import_attributes_supported(agent: &Agent, attributes: &[(JsString, JsString)]) -> bool {
    attributes
        .iter()
        .all(|(k, _)| agent.str(*k) == [b't' as u16, b'y' as u16, b'p' as u16, b'e' as u16])
}

/// LoadRequestedModules([hostDefined]) (§16.2.1.5.1): the promise it returns.
pub fn load_requested_modules(agent: &mut Agent, module: u32) -> JsResult<Value> {
    // 2. Let pc be ! NewPromiseCapability(%Promise%).
    let ctor = agent.intrinsic_c("%Promise%");
    let capability = new_promise_capability(agent, Value::Object(ctor))?;
    // 3. Let state be the GraphLoadingState Record { [[IsLoading]]: true, [[PendingModulesCount]]: 1, ... }.
    let mut state = GraphLoadingState {
        capability,
        is_loading: true,
        pending: 1,
        visited: Vec::new(),
    };
    // 4. Perform InnerModuleLoading(state, module).
    inner_module_loading(agent, &mut state, module)?;
    // 5. Return pc.[[Promise]].
    Ok(capability.promise)
}

/// InnerModuleLoading(state, module) (§16.2.1.5.1.1)
fn inner_module_loading(
    agent: &mut Agent,
    state: &mut GraphLoadingState,
    module: u32,
) -> JsResult<()> {
    // 2. If module is a Cyclic Module Record, module.[[Status]] is new, and state.[[Visited]] does not contain module, then
    if m(agent, module).status == ModuleStatus::New && !state.visited.contains(&module) {
        // a. Append module to state.[[Visited]].
        state.visited.push(module);
        // b-c. Set state.[[PendingModulesCount]] to state.[[PendingModulesCount]] + requestedModulesCount.
        let requests = m(agent, module).requested_modules.clone();
        state.pending += requests.len();
        // d. For each ModuleRequest Record request of module.[[RequestedModules]], do
        for request in requests {
            if !all_import_attributes_supported(agent, &request.attributes) {
                // i. If AllImportAttributesSupported(request.[[Attributes]]) is false, then
                //    ContinueModuleLoading(state, ThrowCompletion(a new SyntaxError object)).
                let err = agent.throw_syntax_error("unsupported import attribute");
                continue_module_loading(agent, state, Err(err))?;
            } else if let Some(loaded) = get_loaded_module(agent, module, &request) {
                // ii. Else if module.[[LoadedModules]] contains a record for request, InnerModuleLoading(state, it).
                inner_module_loading(agent, state, loaded)?;
            } else {
                // iii. Else, perform HostLoadImportedModule(module, request, state.[[HostDefined]], state).
                host_load_imported_module(
                    agent,
                    Referrer::Module(module),
                    request,
                    Payload::Graph(state),
                )?;
            }
            // iv. If state.[[IsLoading]] is false, return unused.
            if !state.is_loading {
                return Ok(());
            }
        }
    }
    // 3-4. Set state.[[PendingModulesCount]] to state.[[PendingModulesCount]] - 1.
    state.pending -= 1;
    // 5. If state.[[PendingModulesCount]] = 0, then
    if state.pending == 0 {
        // a. Set state.[[IsLoading]] to false.
        state.is_loading = false;
        // b. For each Cyclic Module Record loaded of state.[[Visited]], if its status is new, set it to unlinked.
        for &v in &state.visited {
            if m(agent, v).status == ModuleStatus::New {
                m_mut(agent, v).status = ModuleStatus::Unlinked;
            }
        }
        // c. Perform ! Call(state.[[PromiseCapability]].[[Resolve]], undefined, « undefined »).
        call(
            agent,
            state.capability.resolve,
            Value::Undefined,
            &[Value::Undefined],
        )?;
    }
    Ok(())
}

/// ContinueModuleLoading(state, moduleCompletion) (§16.2.1.5.1.2)
fn continue_module_loading(
    agent: &mut Agent,
    state: &mut GraphLoadingState,
    result: JsResult<u32>,
) -> JsResult<()> {
    // 1. If state.[[IsLoading]] is false, return unused.
    if !state.is_loading {
        return Ok(());
    }
    match result {
        // 2. If moduleCompletion is a normal completion, perform InnerModuleLoading(state, moduleCompletion.[[Value]]).
        Ok(module) => inner_module_loading(agent, state, module),
        // 3. Else, set IsLoading to false and reject the promise.
        Err(e) => {
            state.is_loading = false;
            let v = throw_value(e)?;
            call(agent, state.capability.reject, Value::Undefined, &[v])?;
            Ok(())
        }
    }
}

fn get_loaded_module(agent: &Agent, referrer: u32, request: &ModuleRequest) -> Option<u32> {
    m(agent, referrer)
        .loaded_modules
        .iter()
        .find(|(r, _)| r == request)
        .map(|(_, id)| *id)
}

/// GetImportedModule(referrer, request) (§16.2.1.10)
fn get_imported_module(agent: &Agent, referrer: u32, request: &ModuleRequest) -> u32 {
    get_loaded_module(agent, referrer, request).expect("the requested module was loaded")
}

/// The realm and host-defined name of a referrer.
fn referrer_info(agent: &Agent, referrer: Referrer) -> (RealmId, String) {
    match referrer {
        Referrer::Script(s) => {
            let r = &agent.scripts[s as usize];
            (r.realm, r.name.clone())
        }
        Referrer::Module(id) => (m(agent, id).realm, m(agent, id).host_defined.to_string()),
        Referrer::Realm(realm) => (realm, String::new()),
    }
}

/// HostLoadImportedModule(referrer, moduleRequest, hostDefined, payload) (§16.2.1.10)
fn host_load_imported_module(
    agent: &mut Agent,
    referrer: Referrer,
    request: ModuleRequest,
    payload: Payload,
) -> JsResult<()> {
    let result = load_via_host(agent, referrer, &request);
    finish_loading_imported_module(agent, referrer, request, payload, result)
}

fn load_via_host(agent: &mut Agent, referrer: Referrer, request: &ModuleRequest) -> JsResult<u32> {
    let (realm, referrer_name) = referrer_info(agent, referrer);
    let Some(hook) = agent.host_load_module else {
        return Err(agent.throw_type_error("this host cannot load modules"));
    };
    let specifier = agent.str_lossy(request.specifier).to_string();
    let attributes: Vec<(String, String)> = request
        .attributes
        .iter()
        .map(|(k, v)| {
            (
                agent.str_lossy(*k).to_string(),
                agent.str_lossy(*v).to_string(),
            )
        })
        .collect();
    let is_json = attributes.iter().any(|(k, v)| k == "type" && v == "json");
    match hook(agent, &referrer_name, &specifier, &attributes) {
        Ok(loaded) => {
            // A JSON module and a JavaScript module of the same file are different modules.
            let key: Box<str> = if is_json {
                format!("{}\u{0}json", loaded.key).into()
            } else {
                loaded.key.into()
            };
            if let Some(&id) = agent.module_map.get(&(realm, key.clone())) {
                return Ok(id);
            }
            let id = if is_json {
                parse_json_module(agent, &loaded.source, realm, &key)?
            } else {
                parse_module(agent, &loaded.source, realm, &key)?
            };
            agent.module_map.insert((realm, key), id);
            Ok(id)
        }
        Err(msg) => Err(agent.throw_type_error(&msg)),
    }
}

/// FinishLoadingImportedModule(referrer, moduleRequest, payload, result) (§16.2.1.11)
fn finish_loading_imported_module(
    agent: &mut Agent,
    referrer: Referrer,
    request: ModuleRequest,
    payload: Payload,
    result: JsResult<u32>,
) -> JsResult<()> {
    // 1. If result is a normal completion, then
    if let (Ok(module), Referrer::Module(r)) = (&result, referrer) {
        // a. If referrer.[[LoadedModules]] contains a LoadedModuleRequest Record record such that
        //    ModuleRequestsEqual(record, moduleRequest) is true, then assert record.[[Module]] is result.[[Value]].
        // b. Else, append the LoadedModuleRequest Record to referrer.[[LoadedModules]].
        if get_loaded_module(agent, r, &request).is_none() {
            m_mut(agent, r).loaded_modules.push((request, *module));
        }
    }
    match payload {
        // 2. If payload is a GraphLoadingState Record, perform ContinueModuleLoading(payload, result).
        Payload::Graph(state) => continue_module_loading(agent, state, result),
        // 3. Else, perform ContinueDynamicImport(payload, result).
        Payload::Dynamic(capability) => continue_dynamic_import(agent, capability, result),
    }
}

// ---- Linking -----------------------------------------------------------------

/// Link() (§16.2.1.6.1.1)
pub fn link(agent: &mut Agent, module: u32) -> JsResult<()> {
    // 2. Let stack be a new empty List.
    let mut stack = Vec::new();
    // 3. Let result be Completion(InnerModuleLinking(module, stack, 0)).
    if let Err(e) = inner_module_linking(agent, module, &mut stack, 0) {
        // 4. If result is an abrupt completion, then for each m of stack, set its status to unlinked.
        for s in stack {
            m_mut(agent, s).status = ModuleStatus::Unlinked;
        }
        return Err(e);
    }
    Ok(())
}

/// InnerModuleLinking(module, stack, index) (§16.2.1.6.1.1.1)
fn inner_module_linking(
    agent: &mut Agent,
    module: u32,
    stack: &mut Vec<u32>,
    mut index: usize,
) -> JsResult<usize> {
    // 2. If module.[[Status]] is one of linking, linked, evaluating-async, or evaluated, return index.
    if matches!(
        m(agent, module).status,
        ModuleStatus::Linking
            | ModuleStatus::Linked
            | ModuleStatus::EvaluatingAsync
            | ModuleStatus::Evaluated
    ) {
        return Ok(index);
    }
    // 3-8. Set status to linking, the DFS indices, and push module.
    let rec = m_mut(agent, module);
    rec.status = ModuleStatus::Linking;
    rec.dfs_index = index;
    rec.dfs_ancestor_index = index;
    index += 1;
    stack.push(module);
    // 9. For each ModuleRequest Record request of module.[[RequestedModules]], do
    for request in m(agent, module).requested_modules.clone() {
        // a. Let requiredModule be GetImportedModule(module, request).
        let required = get_imported_module(agent, module, &request);
        // b. Set index to ? InnerModuleLinking(requiredModule, stack, index).
        index = inner_module_linking(agent, required, stack, index)?;
        // c. If requiredModule.[[Status]] is linking, update module's [[DFSAncestorIndex]].
        if m(agent, required).status == ModuleStatus::Linking {
            let a = m(agent, required).dfs_ancestor_index;
            let rec = m_mut(agent, module);
            rec.dfs_ancestor_index = rec.dfs_ancestor_index.min(a);
        }
    }
    // 10. Perform ? module.InitializeEnvironment().
    initialize_environment(agent, module)?;
    // 13. If module.[[DFSAncestorIndex]] = module.[[DFSIndex]], then pop the strongly connected component.
    if m(agent, module).dfs_ancestor_index == m(agent, module).dfs_index {
        loop {
            let r = stack.pop().expect("module is on the stack");
            m_mut(agent, r).status = ModuleStatus::Linked;
            if r == module {
                break;
            }
        }
    }
    Ok(index)
}

/// The [[BindingName]] of a ResolvedBinding Record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BindingName {
    Name(JsString),
    Namespace,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Resolution {
    Null,
    Ambiguous,
    Resolved(u32, BindingName),
}

/// GetExportedNames([exportStarSet]) (§16.2.1.6.2)
fn get_exported_names(agent: &Agent, module: u32, export_star_set: &mut Vec<u32>) -> Vec<JsString> {
    // 2. If exportStarSet contains module, then return an empty List (a circular import request).
    if export_star_set.contains(&module) {
        return Vec::new();
    }
    // 3. Append module to exportStarSet.
    export_star_set.push(module);
    let rec = m(agent, module);
    // 4-6. The local and indirect export names.
    let mut names: Vec<JsString> = rec
        .local_export_entries
        .iter()
        .chain(&rec.indirect_export_entries)
        .filter_map(|e| e.export_name)
        .collect();
    // 7. For each ExportEntry Record e of module.[[StarExportEntries]], do
    for e in rec.star_export_entries.clone() {
        let request = &rec.requested_modules[e.request.expect("star exports have a request")];
        let requested = get_imported_module(agent, module, request);
        for n in get_exported_names(agent, requested, export_star_set) {
            // If n is not "default" and exportedNames does not contain n, append n.
            if agent.str(n) != "default".encode_utf16().collect::<Vec<u16>>().as_slice()
                && !names.contains(&n)
            {
                names.push(n);
            }
        }
    }
    names
}

/// ResolveExport(exportName [, resolveSet]) (§16.2.1.6.3)
fn resolve_export(
    agent: &Agent,
    module: u32,
    name: JsString,
    resolve_set: &mut Vec<(u32, JsString)>,
) -> Resolution {
    // 2. For each Record r of resolveSet: if same module and export name, return null (a circular import request).
    if resolve_set.contains(&(module, name)) {
        return Resolution::Null;
    }
    // 3. Append the Record { [[Module]]: module, [[ExportName]]: exportName } to resolveSet.
    resolve_set.push((module, name));
    let rec = m(agent, module);
    // 4. For each ExportEntry Record e of module.[[LocalExportEntries]], do
    for e in &rec.local_export_entries {
        if e.export_name == Some(name) {
            return Resolution::Resolved(
                module,
                BindingName::Name(e.local_name.expect("local name")),
            );
        }
    }
    // 5. For each ExportEntry Record e of module.[[IndirectExportEntries]], do
    for e in &rec.indirect_export_entries {
        if e.export_name == Some(name) {
            let request =
                &rec.requested_modules[e.request.expect("indirect exports have a request")];
            let imported = get_imported_module(agent, module, request);
            return match e.import_name {
                // b.i. If e.[[ImportName]] is all, return ResolvedBinding { importedModule, namespace }.
                ExportImport::All => Resolution::Resolved(imported, BindingName::Namespace),
                ExportImport::Name(n) => resolve_export(agent, imported, n, resolve_set),
                _ => unreachable!("indirect exports import a name or all"),
            };
        }
    }
    // 6. If exportName is "default", return null.
    if agent.str(name) == "default".encode_utf16().collect::<Vec<u16>>().as_slice() {
        return Resolution::Null;
    }
    // 7-8. The star exports, which must agree.
    let mut star = Resolution::Null;
    for e in &rec.star_export_entries {
        let request = &rec.requested_modules[e.request.expect("star exports have a request")];
        let imported = get_imported_module(agent, module, request);
        let resolution = resolve_export(agent, imported, name, resolve_set);
        match resolution {
            Resolution::Ambiguous => return Resolution::Ambiguous,
            Resolution::Null => {}
            Resolution::Resolved(rm, rb) => match star {
                Resolution::Null => star = resolution,
                Resolution::Resolved(sm, sb) => {
                    if rm != sm || rb != sb {
                        return Resolution::Ambiguous;
                    }
                }
                Resolution::Ambiguous => unreachable!(),
            },
        }
    }
    // 9. Return starResolution.
    star
}

/// InitializeEnvironment() (§16.2.1.6.4)
fn initialize_environment(agent: &mut Agent, module: u32) -> JsResult<()> {
    // 1. For each ExportEntry Record e of module.[[IndirectExportEntries]], do
    for e in m(agent, module).indirect_export_entries.clone() {
        let name = e.export_name.expect("indirect exports have a name");
        // b-c. If resolution is either null or ambiguous, throw a SyntaxError exception.
        if !matches!(
            resolve_export(agent, module, name, &mut Vec::new()),
            Resolution::Resolved(..)
        ) {
            let msg = format!("cannot resolve export '{}'", agent.str_lossy(name));
            return Err(agent.throw_syntax_error(&msg));
        }
    }
    // 3-5. Let env be NewModuleEnvironment(realm.[[GlobalEnv]]); set module.[[Environment]] to env.
    let realm = m(agent, module).realm;
    let global_env = agent.realm(realm).global_env;
    let env = Environment::new_module(agent, global_env);
    m_mut(agent, module).environment = Some(env);
    // A Synthetic Module Record's Link(): a binding per export name, initialized to undefined.
    if m(agent, module).synthetic_default.is_some() {
        let default = agent.string("default");
        env.create_mutable_binding(agent, default, false)?;
        return env.initialize_binding(agent, default, Value::Undefined);
    }
    // 7. For each ImportEntry Record in of module.[[ImportEntries]], do
    for ie in m(agent, module).import_entries.clone() {
        let request = m(agent, module).requested_modules[ie.request].clone();
        let imported = get_imported_module(agent, module, &request);
        match ie.import_name {
            // b. If in.[[ImportName]] is namespace-object, bind the namespace.
            ImportName::NamespaceObject => {
                let namespace = get_module_namespace(agent, imported)?;
                env.create_immutable_binding(agent, ie.local_name, true)?;
                env.initialize_binding(agent, ie.local_name, Value::Object(namespace))?;
            }
            ImportName::Name(n) => match resolve_export(agent, imported, n, &mut Vec::new()) {
                // ii. If resolution is either null or ambiguous, throw a SyntaxError exception.
                Resolution::Null | Resolution::Ambiguous => {
                    let msg = format!(
                        "the requested module does not provide an export named '{}'",
                        agent.str_lossy(n)
                    );
                    return Err(agent.throw_syntax_error(&msg));
                }
                // iii. If resolution.[[BindingName]] is namespace, bind that module's namespace.
                Resolution::Resolved(target, BindingName::Namespace) => {
                    let namespace = get_module_namespace(agent, target)?;
                    env.create_immutable_binding(agent, ie.local_name, true)?;
                    env.initialize_binding(agent, ie.local_name, Value::Object(namespace))?;
                }
                // iv. Else, perform env.CreateImportBinding(in.[[LocalName]], resolution.[[Module]], resolution.[[BindingName]]).
                Resolution::Resolved(target, BindingName::Name(b)) => {
                    env.create_import_binding(agent, ie.local_name, target, b);
                }
            },
        }
    }
    // 8-16. moduleContext, as the running execution context while declarations are instantiated.
    agent.contexts.push(ExecutionContext {
        function: None,
        realm,
        script_or_module: Some(ScriptOrModule::Module(module)),
    });
    let result = (|| {
        let code = m(agent, module).code.clone();
        // 21-22. Var declarations: mutable bindings initialized to undefined.
        let mut declared: Vec<JsString> = Vec::new();
        for &name in &code.decls().var_names {
            if !declared.contains(&name) {
                env.create_mutable_binding(agent, name, false)?;
                env.initialize_binding(agent, name, Value::Undefined)?;
                declared.push(name);
            }
        }
        // 23-24. Lexical declarations: const are immutable, the rest mutable.
        for &(name, is_const) in &code.decls().lexical {
            if is_const {
                env.create_immutable_binding(agent, name, true)?;
            } else {
                env.create_mutable_binding(agent, name, false)?;
            }
        }
        // Function declarations: created and initialized now (InstantiateFunctionObject).
        let star_default = agent.string("*default*");
        for &(name, index) in &code.decls().functions {
            env.create_mutable_binding(agent, name, false)?;
            let fname = (name == star_default).then(|| agent.key("default"));
            let fo = create_closure(
                agent,
                env,
                None,
                code.functions[index as usize].clone(),
                fname,
            )?;
            env.initialize_binding(agent, name, Value::Object(fo))?;
        }
        Ok(())
    })();
    // 25. Remove moduleContext from the execution context stack.
    agent.contexts.pop();
    result
}

// ---- Evaluation ----------------------------------------------------------------

/// Evaluate() (§16.2.1.6.1.3): the promise for the module's evaluation.
pub fn evaluate(agent: &mut Agent, module: u32) -> JsResult<Value> {
    let mut module = module;
    // 3. If module.[[Status]] is either evaluating-async or evaluated, then
    if matches!(
        m(agent, module).status,
        ModuleStatus::EvaluatingAsync | ModuleStatus::Evaluated
    ) {
        // a. If module.[[CycleRoot]] is not empty, set module to module.[[CycleRoot]].
        if let Some(root) = m(agent, module).cycle_root {
            module = root;
        }
    }
    // 4. If module.[[TopLevelCapability]] is not empty, return its promise.
    if let Some(cap) = m(agent, module).top_level_capability {
        return Ok(cap.promise);
    }
    // 5-7. Let stack be empty; promiseCapability be ! NewPromiseCapability(%Promise%).
    let mut stack = Vec::new();
    let ctor = agent.intrinsic_c("%Promise%");
    let capability = new_promise_capability(agent, Value::Object(ctor))?;
    m_mut(agent, module).top_level_capability = Some(capability);
    // 8. Let result be Completion(InnerModuleEvaluation(module, stack, 0)).
    match inner_module_evaluation(agent, module, &mut stack, 0) {
        // 9. If result is an abrupt completion, then
        Err(JsError::Throw(e)) => {
            // a. For each requiredModule of stack: status evaluated, [[EvaluationError]] result.
            for s in stack {
                let rec = m_mut(agent, s);
                rec.status = ModuleStatus::Evaluated;
                rec.evaluation_error = Some(e);
            }
            // d. Perform ! Call(promiseCapability.[[Reject]], undefined, « result.[[Value]] »).
            call(agent, capability.reject, Value::Undefined, &[e])?;
        }
        Err(e) => return Err(e),
        // 10. Else, if module.[[Status]] is evaluated, resolve the promise.
        Ok(_) => {
            if m(agent, module).status == ModuleStatus::Evaluated {
                call(
                    agent,
                    capability.resolve,
                    Value::Undefined,
                    &[Value::Undefined],
                )?;
            }
        }
    }
    // 11. Return promiseCapability.[[Promise]].
    Ok(capability.promise)
}

/// InnerModuleEvaluation(module, stack, index) (§16.2.1.6.1.3.1)
fn inner_module_evaluation(
    agent: &mut Agent,
    module: u32,
    stack: &mut Vec<u32>,
    mut index: usize,
) -> JsResult<usize> {
    // 2. If module.[[Status]] is either evaluating-async or evaluated, then
    if matches!(
        m(agent, module).status,
        ModuleStatus::EvaluatingAsync | ModuleStatus::Evaluated
    ) {
        // a-b. Return index, or ? module.[[EvaluationError]].
        return match m(agent, module).evaluation_error {
            None => Ok(index),
            Some(e) => Err(JsError::Throw(e)),
        };
    }
    // 3. If module.[[Status]] is evaluating, return index.
    if m(agent, module).status == ModuleStatus::Evaluating {
        return Ok(index);
    }
    // 5-10. Mark module as evaluating and push it.
    let rec = m_mut(agent, module);
    rec.status = ModuleStatus::Evaluating;
    let module_index = index;
    rec.dfs_index = index;
    rec.dfs_ancestor_index = index;
    rec.pending_async_dependencies = 0;
    index += 1;
    stack.push(module);
    // 11. For each ModuleRequest Record request of module.[[RequestedModules]], do
    for request in m(agent, module).requested_modules.clone() {
        // a. Let requiredModule be GetImportedModule(module, request).
        let mut required = get_imported_module(agent, module, &request);
        // b. Set index to ? InnerModuleEvaluation(requiredModule, stack, index).
        index = inner_module_evaluation(agent, required, stack, index)?;
        // c.iii. If requiredModule.[[Status]] is evaluating, then update module's [[DFSAncestorIndex]].
        if m(agent, required).status == ModuleStatus::Evaluating {
            let a = m(agent, required).dfs_ancestor_index;
            let rec = m_mut(agent, module);
            rec.dfs_ancestor_index = rec.dfs_ancestor_index.min(a);
        } else {
            // iv. Else, set requiredModule to requiredModule.[[CycleRoot]]; throw its [[EvaluationError]] if any.
            required = m(agent, required)
                .cycle_root
                .expect("an evaluated module has a cycle root");
            if let Some(e) = m(agent, required).evaluation_error {
                return Err(JsError::Throw(e));
            }
        }
        // v. If requiredModule.[[AsyncEvaluationOrder]] is an integer, then
        if let AsyncOrder::Order(_) = m(agent, required).async_evaluation_order {
            m_mut(agent, module).pending_async_dependencies += 1;
            m_mut(agent, required).async_parent_modules.push(module);
        }
    }
    // 12. If module.[[HasTLA]] is true or module.[[PendingAsyncDependencies]] > 0, then
    if m(agent, module).has_tla || m(agent, module).pending_async_dependencies > 0 {
        // b. Set module.[[AsyncEvaluationOrder]] to IncrementModuleAsyncEvaluationCount().
        agent.module_async_evaluation_count += 1;
        let order = agent.module_async_evaluation_count;
        m_mut(agent, module).async_evaluation_order = AsyncOrder::Order(order);
        // c. If module.[[PendingAsyncDependencies]] = 0, perform ExecuteAsyncModule(module).
        if m(agent, module).pending_async_dependencies == 0 {
            execute_async_module(agent, module)?;
        }
    } else {
        // 13. Else, perform ? module.ExecuteModule().
        execute_module(agent, module, None)?;
    }
    // 16. If module.[[DFSAncestorIndex]] = moduleIndex, then pop the component.
    if m(agent, module).dfs_ancestor_index == module_index {
        loop {
            let r = stack.pop().expect("module is on the stack");
            let rec = m_mut(agent, r);
            rec.status = if rec.async_evaluation_order == AsyncOrder::Unset {
                ModuleStatus::Evaluated
            } else {
                ModuleStatus::EvaluatingAsync
            };
            rec.cycle_root = Some(module);
            if r == module {
                break;
            }
        }
    }
    // 17. Return index.
    Ok(index)
}

/// ExecuteModule([capability]) (§16.2.1.6.5)
fn execute_module(
    agent: &mut Agent,
    module: u32,
    capability: Option<PromiseCapability>,
) -> JsResult<()> {
    // A Synthetic Module Record's evaluation steps: SetSyntheticModuleExport(module, "default", value).
    if let Some(value) = m(agent, module).synthetic_default {
        let env = m(agent, module)
            .environment
            .expect("a linked module has an environment");
        let default = agent.string("default");
        return env.set_mutable_binding(agent, default, value, true);
    }
    let rec = m(agent, module);
    let (realm, env, code) = (
        rec.realm,
        rec.environment.expect("a linked module has an environment"),
        rec.code.clone(),
    );
    let frame = Frame::new(code, env, None, None);
    match capability {
        // 3. Else, perform AsyncBlockStart(capability, module.[[ECMAScriptCode]], moduleContext).
        Some(cap) => async_function::start_async_module(agent, module, frame, cap),
        None => {
            // 1-2, 4. Push moduleContext and evaluate the module's code.
            let mut frame = frame;
            agent.contexts.push(ExecutionContext {
                function: None,
                realm,
                script_or_module: Some(ScriptOrModule::Module(module)),
            });
            let r = run_frame(agent, &mut frame);
            agent.contexts.pop();
            r.map(|_| ())
        }
    }
}

fn module_closure(
    agent: &mut Agent,
    f: crate::function::NativeFn,
    module: u32,
    length: u32,
) -> Value {
    let empty = PropertyKey::String(agent.names.empty);
    Value::Object(create_builtin_function(
        agent,
        f,
        length,
        empty,
        vec![Value::Number(module as f64)],
        None,
        None,
        None,
        false,
    ))
}

fn module_of_active(agent: &Agent) -> u32 {
    let f = agent
        .active_function()
        .expect("a module closure is running");
    match builtin_slots(agent, f).first() {
        Some(Value::Number(n)) => *n as u32,
        _ => unreachable!("module closures carry their module"),
    }
}

/// ExecuteAsyncModule(module) (§16.2.1.6.1.3.2)
fn execute_async_module(agent: &mut Agent, module: u32) -> JsResult<()> {
    // 3. Let capability be ! NewPromiseCapability(%Promise%).
    let ctor = agent.intrinsic_c("%Promise%");
    let capability = new_promise_capability(agent, Value::Object(ctor))?;
    // 4-7. onFulfilled and onRejected run AsyncModuleExecutionFulfilled/Rejected.
    let on_fulfilled = module_closure(agent, async_fulfilled_closure, module, 0);
    let on_rejected = module_closure(agent, async_rejected_closure, module, 0);
    // 8. Perform PerformPromiseThen(capability.[[Promise]], onFulfilled, onRejected).
    perform_promise_then(
        agent,
        promise_object(capability.promise),
        on_fulfilled,
        on_rejected,
        None,
    );
    // 9. Perform ! module.ExecuteModule(capability).
    execute_module(agent, module, Some(capability))
}

fn async_fulfilled_closure(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let module = module_of_active(agent);
    async_module_execution_fulfilled(agent, module)?;
    Ok(Value::Undefined)
}

fn async_rejected_closure(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let module = module_of_active(agent);
    async_module_execution_rejected(agent, module, args.get_or_undefined(0))?;
    Ok(Value::Undefined)
}

/// GatherAvailableAncestors(module, execList) (§16.2.1.6.1.3.3)
fn gather_available_ancestors(agent: &mut Agent, module: u32, exec_list: &mut Vec<u32>) {
    for ancestor in m(agent, module).async_parent_modules.clone() {
        let root = m(agent, ancestor).cycle_root.expect("cycle root");
        if !exec_list.contains(&ancestor) && m(agent, root).evaluation_error.is_none() {
            let rec = m_mut(agent, ancestor);
            rec.pending_async_dependencies -= 1;
            if rec.pending_async_dependencies == 0 {
                exec_list.push(ancestor);
                if !m(agent, ancestor).has_tla {
                    gather_available_ancestors(agent, ancestor, exec_list);
                }
            }
        }
    }
}

/// AsyncModuleExecutionFulfilled(module) (§16.2.1.6.1.3.4)
fn async_module_execution_fulfilled(agent: &mut Agent, module: u32) -> JsResult<()> {
    // 1. If module.[[Status]] is evaluated, return.
    if m(agent, module).status == ModuleStatus::Evaluated {
        return Ok(());
    }
    // 5-6. Set [[AsyncEvaluationOrder]] to done and [[Status]] to evaluated.
    let rec = m_mut(agent, module);
    rec.async_evaluation_order = AsyncOrder::Done;
    rec.status = ModuleStatus::Evaluated;
    // 7. If module.[[TopLevelCapability]] is not empty, resolve it.
    if let Some(cap) = m(agent, module).top_level_capability {
        call(agent, cap.resolve, Value::Undefined, &[Value::Undefined])?;
    }
    // 8-11. Gather the ancestors that can now run, sorted by [[AsyncEvaluationOrder]].
    let mut exec_list = Vec::new();
    gather_available_ancestors(agent, module, &mut exec_list);
    exec_list.sort_by_key(|&a| match m(agent, a).async_evaluation_order {
        AsyncOrder::Order(n) => n,
        _ => u64::MAX,
    });
    // 12. For each Cyclic Module Record ancestorModule of sortedExecList, do
    for ancestor in exec_list {
        if m(agent, ancestor).status == ModuleStatus::Evaluated {
            // a. It failed already.
            continue;
        }
        if m(agent, ancestor).has_tla {
            // b. Perform ExecuteAsyncModule(ancestorModule).
            execute_async_module(agent, ancestor)?;
        } else {
            // c. Let result be Completion(ancestorModule.ExecuteModule()).
            match execute_module(agent, ancestor, None) {
                Err(JsError::Throw(e)) => async_module_execution_rejected(agent, ancestor, e)?,
                Err(e) => return Err(e),
                Ok(()) => {
                    let rec = m_mut(agent, ancestor);
                    rec.async_evaluation_order = AsyncOrder::Done;
                    rec.status = ModuleStatus::Evaluated;
                    if let Some(cap) = m(agent, ancestor).top_level_capability {
                        call(agent, cap.resolve, Value::Undefined, &[Value::Undefined])?;
                    }
                }
            }
        }
    }
    Ok(())
}

/// AsyncModuleExecutionRejected(module, error) (§16.2.1.6.1.3.5)
fn async_module_execution_rejected(agent: &mut Agent, module: u32, error: Value) -> JsResult<()> {
    // 1. If module.[[Status]] is evaluated, return.
    if m(agent, module).status == ModuleStatus::Evaluated {
        return Ok(());
    }
    // 5-7. Record the error.
    let rec = m_mut(agent, module);
    rec.evaluation_error = Some(error);
    rec.status = ModuleStatus::Evaluated;
    rec.async_evaluation_order = AsyncOrder::Done;
    // 9. If module.[[TopLevelCapability]] is not empty, reject it.
    if let Some(cap) = m(agent, module).top_level_capability {
        call(agent, cap.reject, Value::Undefined, &[error])?;
    }
    // 10. For each ancestorModule of module.[[AsyncParentModules]], reject it too.
    for ancestor in m(agent, module).async_parent_modules.clone() {
        async_module_execution_rejected(agent, ancestor, error)?;
    }
    Ok(())
}

// ---- Namespaces ----------------------------------------------------------------

/// GetModuleNamespace(module) (§16.2.1.12)
pub fn get_module_namespace(agent: &mut Agent, module: u32) -> JsResult<Object> {
    // 3. Let namespace be module.[[Namespace]].
    if let Some(ns) = m(agent, module).namespace {
        return Ok(ns);
    }
    // 4.a. Let exportedNames be module.GetExportedNames().
    let exported = get_exported_names(agent, module, &mut Vec::new());
    // b-c. The names that resolve unambiguously.
    let mut names: Vec<JsString> = exported
        .into_iter()
        .filter(|&n| {
            matches!(
                resolve_export(agent, module, n, &mut Vec::new()),
                Resolution::Resolved(..)
            )
        })
        .collect();
    // d. Set namespace to ModuleNamespaceCreate(module, unambiguousNames).
    names.sort_by(|&a, &b| agent.str(a).cmp(agent.str(b)));
    names.dedup();
    let ns = agent.object_with_kind(
        None,
        ObjectKind::ModuleNamespace(Box::new(NamespaceData {
            module,
            exports: names,
        })),
    );
    // ModuleNamespaceCreate step 9: the %Symbol.toStringTag% property.
    let tag = agent.string("Module");
    let key = PropertyKey::Symbol(agent.symbols_wk.to_string_tag);
    agent.obj_mut(ns).properties.insert(
        key,
        Property::Data {
            value: Value::String(tag),
            writable: false,
            enumerable: false,
            configurable: false,
        },
    );
    m_mut(agent, module).namespace = Some(ns);
    Ok(ns)
}

/// A module namespace's [[Get]] of an export name (§10.4.6.8 steps 4-12).
pub fn module_namespace_get(agent: &mut Agent, module: u32, name: JsString) -> JsResult<Value> {
    // 6. Let binding be m.ResolveExport(P).
    let Resolution::Resolved(target, binding) =
        resolve_export(agent, module, name, &mut Vec::new())
    else {
        unreachable!("namespace exports resolve");
    };
    match binding {
        // 10. If binding.[[BindingName]] is namespace, return GetModuleNamespace(targetModule).
        BindingName::Namespace => Ok(Value::Object(get_module_namespace(agent, target)?)),
        BindingName::Name(n) => {
            // 11. Let targetEnv be targetModule.[[Environment]]; if empty, throw a ReferenceError exception.
            let Some(env) = m(agent, target).environment else {
                let msg = format!("{} is not initialized", agent.str_lossy(name));
                return Err(agent.throw_reference_error(&msg));
            };
            // 12. Return ? targetEnv.GetBindingValue(binding.[[BindingName]], true).
            env.get_binding_value(agent, n, true)
        }
    }
}

// ---- import() and import.meta ----------------------------------------------------

/// GetActiveScriptOrModule() (§9.4.1)
pub fn get_active_script_or_module(agent: &Agent) -> Option<ScriptOrModule> {
    agent.contexts.iter().rev().find_map(|c| c.script_or_module)
}

/// The import.meta of the running module (§13.3.12.1).
pub fn import_meta(agent: &mut Agent) -> JsResult<Value> {
    // 1-2. Let module be GetActiveScriptOrModule(); it is a Source Text Module Record.
    let Some(ScriptOrModule::Module(module)) = get_active_script_or_module(agent) else {
        return Err(agent.throw_syntax_error("import.meta outside a module"));
    };
    // 3-4. If module.[[ImportMeta]] is empty, create it and let the host add properties.
    if let Some(meta) = m(agent, module).import_meta {
        return Ok(Value::Object(meta));
    }
    let meta = agent.ordinary_object_create(None);
    if let Some(hook) = agent.host_import_meta {
        hook(agent, module, meta);
    }
    m_mut(agent, module).import_meta = Some(meta);
    Ok(Value::Object(meta))
}

/// EvaluateImportCall(specifier, options) (§13.3.10.2), after evaluating both expressions.
pub fn evaluate_import_call(
    agent: &mut Agent,
    specifier: Value,
    options: Value,
) -> JsResult<Value> {
    // 1-2. Let referrer be GetActiveScriptOrModule(), or the current Realm Record.
    let referrer = match get_active_script_or_module(agent) {
        Some(ScriptOrModule::Script(s)) => Referrer::Script(s),
        Some(ScriptOrModule::Module(id)) => Referrer::Module(id),
        None => Referrer::Realm(agent.current_realm()),
    };
    // 7. Let promiseCapability be ! NewPromiseCapability(%Promise%).
    let ctor = agent.intrinsic_c("%Promise%");
    let capability = new_promise_capability(agent, Value::Object(ctor))?;
    let reject = |agent: &mut Agent, e: JsError| -> JsResult<Value> {
        let v = throw_value(e)?;
        call(agent, capability.reject, Value::Undefined, &[v])?;
        Ok(capability.promise)
    };
    // 8-9. Let specifierString be Completion(ToString(specifier)); IfAbruptRejectPromise.
    let specifier = match ops::to_string(agent, specifier) {
        Ok(s) => s,
        Err(e) => return reject(agent, e),
    };
    // 10-11. The import attributes from options.with.
    let mut attributes: Vec<(JsString, JsString)> = Vec::new();
    if !options.is_undefined() {
        let Value::Object(opts) = options else {
            let e = agent.throw_type_error("import() options must be an object");
            return reject(agent, e);
        };
        let key = agent.key("with");
        let with = match ops::get(agent, opts, key) {
            Ok(v) => v,
            Err(e) => return reject(agent, e),
        };
        if !with.is_undefined() {
            let Value::Object(with) = with else {
                let e = agent.throw_type_error("import() options.with must be an object");
                return reject(agent, e);
            };
            let entries =
                match ops::enumerable_own_properties(agent, with, ops::EnumerableKind::KeyValue) {
                    Ok(e) => e,
                    Err(e) => return reject(agent, e),
                };
            for entry in entries {
                let Value::Object(pair) = entry else { continue };
                let k = ops::get(agent, pair, PropertyKey::Index(0))?;
                let v = ops::get(agent, pair, PropertyKey::Index(1))?;
                let (Value::String(k), Value::String(v)) = (k, v) else {
                    let e = agent.throw_type_error("import attribute values must be strings");
                    return reject(agent, e);
                };
                attributes.push((k, v));
            }
        }
        // d. If AllImportAttributesSupported(attributes) is false, reject with a SyntaxError.
        if !all_import_attributes_supported(agent, &attributes) {
            let e = agent.throw_syntax_error("unsupported import attribute");
            return reject(agent, e);
        }
        // e. Sort attributes by the code units of their [[Key]] fields.
        attributes.sort_by(|a, b| agent.str(a.0).cmp(agent.str(b.0)));
    }
    // 12. Let moduleRequest be a new ModuleRequest Record.
    let request = ModuleRequest {
        specifier,
        attributes,
    };
    // 13. Perform HostLoadImportedModule(referrer, moduleRequest, empty, promiseCapability).
    host_load_imported_module(agent, referrer, request, Payload::Dynamic(capability))?;
    // 14. Return promiseCapability.[[Promise]].
    Ok(capability.promise)
}

/// ContinueDynamicImport(promiseCapability, moduleCompletion) (§16.2.1.12)
fn continue_dynamic_import(
    agent: &mut Agent,
    capability: PromiseCapability,
    result: JsResult<u32>,
) -> JsResult<()> {
    // 1. If moduleCompletion is an abrupt completion, reject and return.
    let module = match result {
        Ok(m) => m,
        Err(e) => {
            let v = throw_value(e)?;
            call(agent, capability.reject, Value::Undefined, &[v])?;
            return Ok(());
        }
    };
    // 3. Let loadPromise be module.LoadRequestedModules().
    let load_promise = load_requested_modules(agent, module)?;
    // 4-5. onRejected rejects promiseCapability.
    let empty = PropertyKey::String(agent.names.empty);
    let caps = vec![
        Value::Number(module as f64),
        capability.promise,
        capability.resolve,
        capability.reject,
    ];
    let on_rejected = create_builtin_function(
        agent,
        dynamic_import_rejected,
        1,
        empty,
        caps.clone(),
        None,
        None,
        None,
        false,
    );
    // 6-7. linkAndEvaluate links and evaluates the module.
    let link_and_evaluate = create_builtin_function(
        agent,
        dynamic_import_link_and_evaluate,
        0,
        empty,
        caps,
        None,
        None,
        None,
        false,
    );
    // 8. Perform PerformPromiseThen(loadPromise, linkAndEvaluate, onRejected).
    perform_promise_then(
        agent,
        promise_object(load_promise),
        Value::Object(link_and_evaluate),
        Value::Object(on_rejected),
        None,
    );
    Ok(())
}

fn dynamic_import_slots(agent: &Agent) -> (u32, PromiseCapability) {
    let f = agent
        .active_function()
        .expect("an import() closure is running");
    let slots = builtin_slots(agent, f);
    let module = match slots[0] {
        Value::Number(n) => n as u32,
        _ => unreachable!("import() closures carry their module"),
    };
    (
        module,
        PromiseCapability {
            promise: slots[1],
            resolve: slots[2],
            reject: slots[3],
        },
    )
}

fn dynamic_import_rejected(
    agent: &mut Agent,
    _this: Value,
    args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let (_, capability) = dynamic_import_slots(agent);
    // a. Perform ! Call(promiseCapability.[[Reject]], undefined, « reason »).
    call(
        agent,
        capability.reject,
        Value::Undefined,
        &[args.get_or_undefined(0)],
    )?;
    Ok(Value::Undefined)
}

fn dynamic_import_link_and_evaluate(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let (module, capability) = dynamic_import_slots(agent);
    // a-b. Let link be Completion(module.Link()); if abrupt, reject and return.
    if let Err(e) = link(agent, module) {
        let v = throw_value(e)?;
        call(agent, capability.reject, Value::Undefined, &[v])?;
        return Ok(Value::Undefined);
    }
    // c. Let evaluatePromise be module.Evaluate().
    let evaluate_promise = evaluate(agent, module)?;
    // d-e. onFulfilled resolves with the module namespace.
    let f = agent.active_function().expect("running closure");
    let slots = builtin_slots(agent, f).to_vec();
    let empty = PropertyKey::String(agent.names.empty);
    let on_fulfilled = create_builtin_function(
        agent,
        dynamic_import_fulfilled,
        0,
        empty,
        slots.clone(),
        None,
        None,
        None,
        false,
    );
    let on_rejected = create_builtin_function(
        agent,
        dynamic_import_rejected,
        1,
        empty,
        slots,
        None,
        None,
        None,
        false,
    );
    // f. Perform PerformPromiseThen(evaluatePromise, onFulfilled, onRejected).
    perform_promise_then(
        agent,
        promise_object(evaluate_promise),
        Value::Object(on_fulfilled),
        Value::Object(on_rejected),
        None,
    );
    Ok(Value::Undefined)
}

fn dynamic_import_fulfilled(
    agent: &mut Agent,
    _this: Value,
    _args: &[Value],
    _nt: Option<Object>,
) -> JsResult<Value> {
    let (module, capability) = dynamic_import_slots(agent);
    // i. Let namespace be GetModuleNamespace(module).
    let ns = get_module_namespace(agent, module)?;
    // ii. Perform ! Call(promiseCapability.[[Resolve]], undefined, « namespace »).
    call(
        agent,
        capability.resolve,
        Value::Undefined,
        &[Value::Object(ns)],
    )?;
    Ok(Value::Undefined)
}

/// Loads, links and evaluates a module the host names directly (an entry
/// point): HostLoadImportedModule from the realm, then Link and Evaluate.
/// Returns the evaluation promise; loading and linking errors are thrown.
pub fn run_module(agent: &mut Agent, realm: RealmId, key: &str, source: &str) -> JsResult<Value> {
    let module = match agent.module_map.get(&(realm, key.into())) {
        Some(&id) => id,
        None => {
            let id = parse_module(agent, source, realm, key)?;
            agent.module_map.insert((realm, key.into()), id);
            id
        }
    };
    let load = load_requested_modules(agent, module)?;
    // The host loads synchronously, so the load promise has settled.
    if let Value::Object(p) = load
        && let Some(Err(reason)) = crate::ops::promise::promise_result(agent, p)
    {
        return Err(JsError::Throw(reason));
    }
    link(agent, module)?;
    evaluate(agent, module)
}

// ---- garbage collection ----------------------------------------------------

use crate::gc::{Trace, Tracer};

impl Trace for ModuleRequest {
    fn trace(&self, t: &mut Tracer) {
        t.string(self.specifier);
        self.attributes.trace(t);
    }
}

impl Trace for ImportEntry {
    fn trace(&self, t: &mut Tracer) {
        if let ImportName::Name(n) = self.import_name {
            t.string(n);
        }
        t.string(self.local_name);
    }
}

impl Trace for ExportEntry {
    fn trace(&self, t: &mut Tracer) {
        self.export_name.trace(t);
        if let ExportImport::Name(n) = self.import_name {
            t.string(n);
        }
        self.local_name.trace(t);
    }
}

impl Trace for ModuleRecord {
    fn trace(&self, t: &mut Tracer) {
        t.code(&self.code);
        self.environment.trace(t);
        self.namespace.trace(t);
        self.evaluation_error.trace(t);
        self.requested_modules.trace(t);
        for (r, _) in &self.loaded_modules {
            r.trace(t);
        }
        self.top_level_capability.trace(t);
        self.import_meta.trace(t);
        self.synthetic_default.trace(t);
        self.import_entries.trace(t);
        self.local_export_entries.trace(t);
        self.indirect_export_entries.trace(t);
        self.star_export_entries.trace(t);
    }
}
