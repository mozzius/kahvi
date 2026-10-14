//! Environment Records (ECMA-262 §9.1). Implemented literally: bindings are
//! looked up by name, which keeps `eval` and `with` simple and correct.

use crate::agent::Agent;
use crate::error::JsResult;
use crate::object::PropertyKey;
use crate::ops;
use crate::value::{JsString, Object, Value};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct EnvId(pub u32);

#[derive(Debug, Clone)]
pub struct Binding {
    /// `None` while uninitialized (the temporal dead zone).
    pub value: Option<Value>,
    pub mutable: bool,
    /// Immutable binding that throws on assignment even in sloppy code.
    pub strict: bool,
    pub deletable: bool,
}

/// A declarative record's bindings: a short list searched linearly (most
/// scopes have a handful of names), with a hash index once it grows.
#[derive(Debug, Clone, Default)]
pub struct Bindings {
    list: Vec<(JsString, Binding)>,
    index: Option<crate::hash::FxHashMap<JsString, usize>>,
}

/// The size at which a scope's bindings get a hash index.
const INDEX_FROM: usize = 16;

impl Bindings {
    fn position(&self, name: &JsString) -> Option<usize> {
        match &self.index {
            Some(ix) => ix.get(name).copied(),
            None => self.list.iter().position(|(n, _)| n == name),
        }
    }

    pub fn get(&self, name: &JsString) -> Option<&Binding> {
        self.position(name).map(|i| &self.list[i].1)
    }

    pub fn get_mut(&mut self, name: &JsString) -> Option<&mut Binding> {
        self.position(name).map(|i| &mut self.list[i].1)
    }

    pub fn contains_key(&self, name: &JsString) -> bool {
        self.position(name).is_some()
    }

    /// Adds or replaces a binding.
    pub fn insert(&mut self, name: JsString, binding: Binding) -> Option<Binding> {
        if let Some(i) = self.position(&name) {
            return Some(std::mem::replace(&mut self.list[i].1, binding));
        }
        self.list.push((name, binding));
        if let Some(ix) = &mut self.index {
            ix.insert(name, self.list.len() - 1);
        } else if self.list.len() >= INDEX_FROM {
            self.index = Some(
                self.list
                    .iter()
                    .enumerate()
                    .map(|(i, (n, _))| (*n, i))
                    .collect(),
            );
        }
        None
    }

    pub fn remove(&mut self, name: &JsString) -> Option<Binding> {
        let i = self.position(name)?;
        let (_, b) = self.list.remove(i);
        if self.index.is_some() {
            self.index = Some(
                self.list
                    .iter()
                    .enumerate()
                    .map(|(i, (n, _))| (*n, i))
                    .collect(),
            );
        }
        Some(b)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThisBindingStatus {
    Lexical,
    Initialized,
    Uninitialized,
}

/// The extra fields of a Function Environment Record (§9.1.1.3).
#[derive(Debug, Clone)]
pub struct FunctionEnvData {
    pub this_value: Value,
    pub this_binding_status: ThisBindingStatus,
    pub function_object: Object,
    pub new_target: Value,
}

#[derive(Debug, Clone)]
pub enum Environment {
    /// Declarative (and function, and module) environment records.
    Declarative {
        bindings: Bindings,
        outer: Option<EnvId>,
        function: Option<Box<FunctionEnvData>>,
        /// Module environments resolve imports through other modules' environments.
        imports: crate::hash::FxHashMap<JsString, (u32, JsString)>,
        is_module: bool,
    },
    /// Object environment records (`with` and the global object part).
    Object {
        binding_object: Object,
        is_with_environment: bool,
        outer: Option<EnvId>,
    },
    /// Global environment record (§9.1.1.4).
    Global {
        object_record: EnvId,
        global_this: Object,
        declarative_record: EnvId,
        var_names: crate::hash::FxHashSet<JsString>,
    },
}

/// A cheap, copyable view of which kind of record an environment is.
#[derive(Clone, Copy)]
enum Kind {
    Declarative,
    Object {
        binding_object: Object,
        is_with: bool,
    },
    Global {
        object_record: EnvId,
        declarative_record: EnvId,
    },
}

impl Environment {
    pub fn new_declarative(agent: &mut Agent, outer: Option<EnvId>) -> EnvId {
        agent.alloc_env(Environment::Declarative {
            bindings: Default::default(),
            outer,
            function: None,
            imports: Default::default(),
            is_module: false,
        })
    }

    /// NewFunctionEnvironment(F, newTarget)
    pub fn new_function(
        agent: &mut Agent,
        f: Object,
        new_target: Value,
        outer: EnvId,
        lexical_this: bool,
    ) -> EnvId {
        agent.alloc_env(Environment::Declarative {
            bindings: Default::default(),
            outer: Some(outer),
            function: Some(Box::new(FunctionEnvData {
                this_value: Value::Undefined,
                this_binding_status: if lexical_this {
                    ThisBindingStatus::Lexical
                } else {
                    ThisBindingStatus::Uninitialized
                },
                function_object: f,
                new_target,
            })),
            imports: Default::default(),
            is_module: false,
        })
    }

    pub fn new_module(agent: &mut Agent, outer: EnvId) -> EnvId {
        agent.alloc_env(Environment::Declarative {
            bindings: Default::default(),
            outer: Some(outer),
            function: None,
            imports: Default::default(),
            is_module: true,
        })
    }

    /// NewObjectEnvironment(O, W, E)
    pub fn new_object(agent: &mut Agent, o: Object, is_with: bool, outer: Option<EnvId>) -> EnvId {
        agent.alloc_env(Environment::Object {
            binding_object: o,
            is_with_environment: is_with,
            outer,
        })
    }

    /// NewGlobalEnvironment(G, thisValue)
    pub fn new_global(agent: &mut Agent, global: Object, this_value: Object) -> EnvId {
        let object_record = Environment::new_object(agent, global, false, None);
        let declarative_record = Environment::new_declarative(agent, None);
        agent.alloc_env(Environment::Global {
            object_record,
            global_this: this_value,
            declarative_record,
            var_names: Default::default(),
        })
    }

    pub fn outer(&self) -> Option<EnvId> {
        match self {
            Environment::Declarative { outer, .. } | Environment::Object { outer, .. } => *outer,
            Environment::Global { .. } => None,
        }
    }
}

impl EnvId {
    fn kind(self, agent: &Agent) -> Kind {
        match agent.env(self) {
            Environment::Declarative { .. } => Kind::Declarative,
            Environment::Object {
                binding_object,
                is_with_environment,
                ..
            } => Kind::Object {
                binding_object: *binding_object,
                is_with: *is_with_environment,
            },
            Environment::Global {
                object_record,
                declarative_record,
                ..
            } => Kind::Global {
                object_record: *object_record,
                declarative_record: *declarative_record,
            },
        }
    }

    fn binding(self, agent: &Agent, name: JsString) -> Option<&Binding> {
        match agent.env(self) {
            Environment::Declarative { bindings, .. } => bindings.get(&name),
            _ => None,
        }
    }

    fn import_of(self, agent: &Agent, name: JsString) -> Option<(u32, JsString)> {
        match agent.env(self) {
            Environment::Declarative { imports, .. } => imports.get(&name).copied(),
            _ => None,
        }
    }

    /// CreateImportBinding(N, M, N2) (§9.1.1.5.5): N reads binding N2 of module M.
    pub fn create_import_binding(
        self,
        agent: &mut Agent,
        name: JsString,
        module: u32,
        target: JsString,
    ) {
        if let Environment::Declarative { imports, .. } = agent.env_mut(self) {
            imports.insert(name, (module, target));
        }
    }

    /// HasBinding(N)
    pub fn has_binding(self, agent: &mut Agent, name: JsString) -> JsResult<bool> {
        match self.kind(agent) {
            Kind::Declarative => {
                Ok(self.binding(agent, name).is_some() || self.import_of(agent, name).is_some())
            }
            Kind::Object {
                binding_object,
                is_with: is_with_environment,
            } => {
                let key = agent.key_from_string(name);
                // 1. Let bindingObject be envRec.[[BindingObject]].
                // 2. Let foundBinding be ? HasProperty(bindingObject, N).
                if !ops::has_property(agent, binding_object, key)? {
                    // 3. If foundBinding is false, return false.
                    return Ok(false);
                }
                // 4. If envRec.[[IsWithEnvironment]] is false, return true.
                if !is_with_environment {
                    return Ok(true);
                }
                // 5. Let unscopables be ? Get(bindingObject, %Symbol.unscopables%).
                let unscopables_key = PropertyKey::Symbol(agent.symbols_wk.unscopables);
                let unscopables = ops::get(agent, binding_object, unscopables_key)?;
                // 6. If unscopables is an Object, then
                if let Value::Object(u) = unscopables {
                    // a. Let blocked be ToBoolean(? Get(unscopables, N)).
                    let value = ops::get(agent, u, key)?;
                    let blocked = ops::to_boolean(agent, value);
                    // b. If blocked is true, return false.
                    if blocked {
                        return Ok(false);
                    }
                }
                // 7. Return true.
                Ok(true)
            }
            Kind::Global {
                object_record,
                declarative_record,
            } => {
                if declarative_record.has_binding(agent, name)? {
                    return Ok(true);
                }
                object_record.has_binding(agent, name)
            }
        }
    }

    fn decl_bindings(self, agent: &mut Agent) -> &mut Bindings {
        match agent.env_mut(self) {
            Environment::Declarative { bindings, .. } => bindings,
            _ => unreachable!("declarative environment expected"),
        }
    }

    /// CreateMutableBinding(N, D) for declarative records.
    pub fn create_mutable_binding(
        self,
        agent: &mut Agent,
        name: JsString,
        deletable: bool,
    ) -> JsResult<()> {
        match self.kind(agent) {
            Kind::Declarative => {
                self.decl_bindings(agent).insert(
                    name,
                    Binding {
                        value: None,
                        mutable: true,
                        strict: false,
                        deletable,
                    },
                );
                Ok(())
            }
            Kind::Object { binding_object, .. } => {
                // DefinePropertyOrThrow(bindingObject, N, { [[Value]]: undefined,
                //   [[Writable]]: true, [[Enumerable]]: true, [[Configurable]]: D }).
                let key = agent.key_from_string(name);
                let desc = crate::object::PropertyDescriptor::data(
                    Value::Undefined,
                    true,
                    true,
                    deletable,
                );
                ops::define_property_or_throw(agent, binding_object, key, desc)
            }
            Kind::Global {
                declarative_record, ..
            } => {
                if declarative_record.has_binding(agent, name)? {
                    let msg = format!("redeclaration of '{}'", agent.str_lossy(name));
                    return Err(agent.throw_type_error(&msg));
                }
                declarative_record.create_mutable_binding(agent, name, deletable)
            }
        }
    }

    /// CreateImmutableBinding(N, S)
    pub fn create_immutable_binding(
        self,
        agent: &mut Agent,
        name: JsString,
        strict: bool,
    ) -> JsResult<()> {
        match self.kind(agent) {
            Kind::Declarative => {
                self.decl_bindings(agent).insert(
                    name,
                    Binding {
                        value: None,
                        mutable: false,
                        strict,
                        deletable: false,
                    },
                );
                Ok(())
            }
            Kind::Global {
                declarative_record, ..
            } => {
                if declarative_record.has_binding(agent, name)? {
                    let msg = format!("redeclaration of '{}'", agent.str_lossy(name));
                    return Err(agent.throw_type_error(&msg));
                }
                declarative_record.create_immutable_binding(agent, name, strict)
            }
            Kind::Object { .. } => unreachable!("object environments have no immutable bindings"),
        }
    }

    /// InitializeBinding(N, V)
    pub fn initialize_binding(
        self,
        agent: &mut Agent,
        name: JsString,
        value: Value,
    ) -> JsResult<()> {
        match self.kind(agent) {
            Kind::Declarative => {
                if let Some(b) = self.decl_bindings(agent).get_mut(&name) {
                    b.value = Some(value);
                }
                Ok(())
            }
            Kind::Object { .. } => self.set_mutable_binding(agent, name, value, false),
            Kind::Global {
                object_record,
                declarative_record,
            } => {
                if declarative_record.has_binding(agent, name)? {
                    declarative_record.initialize_binding(agent, name, value)
                } else {
                    object_record.initialize_binding(agent, name, value)
                }
            }
        }
    }

    /// SetMutableBinding(N, V, S)
    pub fn set_mutable_binding(
        self,
        agent: &mut Agent,
        name: JsString,
        value: Value,
        strict: bool,
    ) -> JsResult<()> {
        match self.kind(agent) {
            Kind::Declarative => {
                if self.import_of(agent, name).is_some() {
                    return Err(agent.throw_type_error("assignment to an imported binding"));
                }
                let Some(binding) = self.binding(agent, name).cloned() else {
                    // 1. If envRec does not have a binding for N, then
                    if strict {
                        let msg = format!("{} is not defined", agent.str_lossy(name));
                        return Err(agent.throw_reference_error(&msg));
                    }
                    self.create_mutable_binding(agent, name, true)?;
                    return self.initialize_binding(agent, name, value);
                };
                let strict = strict || binding.strict;
                if binding.value.is_none() {
                    let msg = format!(
                        "cannot access '{}' before initialization",
                        agent.str_lossy(name)
                    );
                    return Err(agent.throw_reference_error(&msg));
                }
                if binding.mutable {
                    if let Some(b) = self.decl_bindings(agent).get_mut(&name) {
                        b.value = Some(value);
                    }
                } else if strict {
                    let msg = format!("assignment to constant '{}'", agent.str_lossy(name));
                    return Err(agent.throw_type_error(&msg));
                }
                Ok(())
            }
            Kind::Object { binding_object, .. } => {
                let key = agent.key_from_string(name);
                // 1. Let stillExists be ? HasProperty(bindingObject, N).
                let still_exists = ops::has_property(agent, binding_object, key)?;
                // 2. If stillExists is false and S is true, throw a ReferenceError exception.
                if !still_exists && strict {
                    let msg = format!("{} is not defined", agent.str_lossy(name));
                    return Err(agent.throw_reference_error(&msg));
                }
                // 3. Perform ? Set(bindingObject, N, V, S).
                ops::set(agent, binding_object, key, value, strict)?;
                Ok(())
            }
            Kind::Global {
                object_record,
                declarative_record,
            } => {
                if declarative_record.has_binding(agent, name)? {
                    declarative_record.set_mutable_binding(agent, name, value, strict)
                } else {
                    object_record.set_mutable_binding(agent, name, value, strict)
                }
            }
        }
    }

    /// GetBindingValue(N, S)
    pub fn get_binding_value(
        self,
        agent: &mut Agent,
        name: JsString,
        strict: bool,
    ) -> JsResult<Value> {
        match self.kind(agent) {
            Kind::Declarative => {
                // An indirect import binding: the target module's binding, once it has an environment.
                if let Some((module, target)) = self.import_of(agent, name) {
                    let Some(env) = agent.modules[module as usize].environment else {
                        let msg = format!("{} is not initialized", agent.str_lossy(name));
                        return Err(agent.throw_reference_error(&msg));
                    };
                    return env.get_binding_value(agent, target, true);
                }
                match self.binding(agent, name).and_then(|b| b.value) {
                    Some(v) => Ok(v),
                    None => {
                        let msg = format!(
                            "cannot access '{}' before initialization",
                            agent.str_lossy(name)
                        );
                        Err(agent.throw_reference_error(&msg))
                    }
                }
            }
            Kind::Object { binding_object, .. } => {
                let key = agent.key_from_string(name);
                // 1. Let value be ? HasProperty(bindingObject, N).
                let exists = ops::has_property(agent, binding_object, key)?;
                // 2. If value is false, then
                if !exists {
                    // a. If S is false, return undefined; otherwise throw a ReferenceError exception.
                    if !strict {
                        return Ok(Value::Undefined);
                    }
                    let msg = format!("{} is not defined", agent.str_lossy(name));
                    return Err(agent.throw_reference_error(&msg));
                }
                // 3. Return ? Get(bindingObject, N).
                ops::get(agent, binding_object, key)
            }
            Kind::Global {
                object_record,
                declarative_record,
            } => {
                if declarative_record.has_binding(agent, name)? {
                    declarative_record.get_binding_value(agent, name, strict)
                } else {
                    object_record.get_binding_value(agent, name, strict)
                }
            }
        }
    }

    /// DeleteBinding(N)
    pub fn delete_binding(self, agent: &mut Agent, name: JsString) -> JsResult<bool> {
        match self.kind(agent) {
            Kind::Declarative => match self.binding(agent, name) {
                Some(b) if !b.deletable => Ok(false),
                _ => {
                    self.decl_bindings(agent).remove(&name);
                    Ok(true)
                }
            },
            Kind::Object { binding_object, .. } => {
                let key = agent.key_from_string(name);
                binding_object.internal_delete(agent, key)
            }
            Kind::Global {
                object_record,
                declarative_record,
            } => {
                if declarative_record.has_binding(agent, name)? {
                    return declarative_record.delete_binding(agent, name);
                }
                let global = agent.global_object_of_env(self);
                let key = agent.key_from_string(name);
                if ops::has_own_property(agent, global, key)? {
                    return object_record.delete_binding(agent, name);
                }
                Ok(true)
            }
        }
    }

    /// HasThisBinding()
    pub fn has_this_binding(self, agent: &Agent) -> bool {
        match agent.env(self) {
            Environment::Declarative {
                function: Some(f), ..
            } => f.this_binding_status != ThisBindingStatus::Lexical,
            Environment::Declarative { is_module, .. } => *is_module,
            Environment::Object { .. } => false,
            Environment::Global { .. } => true,
        }
    }

    /// WithBaseObject()
    pub fn with_base_object(self, agent: &Agent) -> Value {
        match agent.env(self) {
            Environment::Object {
                binding_object,
                is_with_environment: true,
                ..
            } => Value::Object(*binding_object),
            _ => Value::Undefined,
        }
    }

    /// GetThisBinding()
    pub fn get_this_binding(self, agent: &mut Agent) -> JsResult<Value> {
        match agent.env(self) {
            Environment::Declarative {
                function: Some(f), ..
            } => {
                if f.this_binding_status == ThisBindingStatus::Uninitialized {
                    return Err(agent
                        .throw_reference_error("'this' is not initialized; call super() first"));
                }
                Ok(f.this_value)
            }
            Environment::Global { global_this, .. } => Ok(Value::Object(*global_this)),
            _ => Ok(Value::Undefined),
        }
    }

    /// BindThisValue(V)
    pub fn bind_this_value(self, agent: &mut Agent, value: Value) -> JsResult<()> {
        let status = match agent.env(self) {
            Environment::Declarative {
                function: Some(f), ..
            } => f.this_binding_status,
            _ => unreachable!("BindThisValue on a non-function environment"),
        };
        if status == ThisBindingStatus::Initialized {
            return Err(agent.throw_reference_error("super() called twice in constructor"));
        }
        if let Environment::Declarative {
            function: Some(f), ..
        } = agent.env_mut(self)
        {
            f.this_value = value;
            f.this_binding_status = ThisBindingStatus::Initialized;
        }
        Ok(())
    }

    pub fn function_data(self, agent: &Agent) -> Option<&FunctionEnvData> {
        match agent.env(self) {
            Environment::Declarative {
                function: Some(f), ..
            } => Some(f),
            _ => None,
        }
    }

    pub fn outer(self, agent: &Agent) -> Option<EnvId> {
        agent.env(self).outer()
    }
}

impl Agent {
    fn global_object_of_env(&self, env: EnvId) -> Object {
        match self.env(env) {
            Environment::Global { object_record, .. } => match self.env(*object_record) {
                Environment::Object { binding_object, .. } => *binding_object,
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }

    /// GetThisEnvironment(): the nearest environment with a `this` binding.
    pub fn get_this_environment(&self, mut env: EnvId) -> EnvId {
        loop {
            if env.has_this_binding(self) {
                return env;
            }
            env = env
                .outer(self)
                .expect("the global environment has a this binding");
        }
    }
}
