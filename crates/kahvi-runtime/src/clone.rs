//! `structuredClone(value)`: HTML's StructuredSerializeInternal followed by
//! StructuredDeserialize in the same realm, done in one pass.
//!
//! Cloned: primitives other than symbols, Boolean/Number/String/BigInt
//! wrappers, Date, RegExp, ArrayBuffer (with its resizability), typed
//! arrays, DataView, Map, Set, Error (name and message), arrays and ordinary
//! objects (own enumerable string-keyed properties). Shared references and
//! cycles are kept. Anything else (symbols, functions, proxies, other
//! objects with internal slots) throws a DataCloneError.
//!
//! The `transfer` option is not supported.

use kahvi::{Agent, JsResult, Persistent, Runtime, Value};
use kahvi_builtins::introspect::{self, Kind};
use kahvi_core::object::typed_array::array_buffer_data;
use kahvi_core::prelude::*;

/// The intrinsic methods a clone uses, as they were when the runtime
/// started, so a script that replaces `Map.prototype.set` cannot see them.
struct Methods {
    map_set: Persistent,
    set_add: Persistent,
}

pub fn install(rt: &mut Runtime) {
    let (map_set, set_add) = rt.enter(|agent| {
        let map_proto = agent.intrinsic("%Map.prototype%");
        let set_proto = agent.intrinsic("%Set.prototype%");
        let set_key = agent.key("set");
        let add_key = agent.key("add");
        let map_set = ops::get(agent, map_proto, set_key).expect("Map.prototype.set");
        let set_add = ops::get(agent, set_proto, add_key).expect("Set.prototype.add");
        (map_set, set_add)
    });
    let methods = Methods {
        map_set: rt.persist(map_set),
        set_add: rt.persist(set_add),
    };
    let f = rt.function("structuredClone", 1, move |agent, _, args| {
        if let Some(&options) = args.get(1)
            && !matches!(options, Value::Undefined)
        {
            let Value::Object(options) = options else {
                return Err(agent.throw_type_error("structuredClone: options must be an object"));
            };
            let transfer = agent.key("transfer");
            if !matches!(ops::get(agent, options, transfer)?, Value::Undefined) {
                return Err(agent.not_implemented("structuredClone transfer"));
            }
        }
        let mark = agent.gc_roots_len();
        let mut cx = Cloner {
            methods: &methods,
            memory: Vec::new(),
        };
        let r = cx.clone_value(agent, args.get_or_undefined(0));
        agent.pop_roots(mark);
        r
    });
    rt.set_global("structuredClone", Value::Object(f));
}

struct Cloner<'a> {
    methods: &'a Methods,
    /// Objects already cloned, with their clones (both rooted).
    memory: Vec<(Object, Object)>,
}

impl Cloner<'_> {
    fn remember(&mut self, agent: &mut Agent, from: Object, to: Object) {
        agent.push_root(Value::Object(from));
        agent.push_root(Value::Object(to));
        self.memory.push((from, to));
    }

    fn clone_value(&mut self, agent: &mut Agent, value: Value) -> JsResult<Value> {
        let o = match value {
            Value::Symbol(_) => {
                return Err(data_clone_error(agent, "a symbol could not be cloned"));
            }
            Value::Object(o) => o,
            primitive => return Ok(primitive),
        };
        if let Some(&(_, to)) = self.memory.iter().find(|&&(from, _)| from == o) {
            return Ok(Value::Object(to));
        }
        agent.push_root(value);
        let wrapped = match agent.obj(o).kind {
            ObjectKind::BooleanObject(b) => Some(Value::Boolean(b)),
            ObjectKind::NumberObject(n) => Some(Value::Number(n)),
            ObjectKind::StringObject(s) => Some(Value::String(s)),
            ObjectKind::BigIntObject(b) => Some(Value::BigInt(b)),
            _ => None,
        };
        if let Some(primitive) = wrapped {
            let to = ops::to_object(agent, primitive)?;
            self.remember(agent, o, to);
            return Ok(Value::Object(to));
        }
        if ops::is_callable(agent, value) {
            return Err(data_clone_error(agent, "a function could not be cloned"));
        }
        match introspect::kind(agent, o) {
            Kind::Date(t) => {
                let date = agent.intrinsic("%Date%");
                let to = ops::construct(agent, date, &[Value::Number(t)], None)?;
                self.remember(agent, o, to);
                return Ok(Value::Object(to));
            }
            Kind::RegExp(source, flags) => {
                let regexp = agent.intrinsic("%RegExp%");
                let args = [Value::String(source), Value::String(flags)];
                let to = ops::construct(agent, regexp, &args, None)?;
                self.remember(agent, o, to);
                return Ok(Value::Object(to));
            }
            Kind::Map(entries) => {
                let map = agent.intrinsic("%Map%");
                let to = ops::construct(agent, map, &[], None)?;
                self.remember(agent, o, to);
                for &(k, v) in &entries {
                    agent.push_root(k);
                    agent.push_root(v);
                }
                let set = agent.persisted(&self.methods.map_set);
                for (k, v) in entries {
                    let k = self.clone_value(agent, k)?;
                    agent.push_root(k);
                    let v = self.clone_value(agent, v)?;
                    ops::call(agent, set, Value::Object(to), &[k, v])?;
                }
                return Ok(Value::Object(to));
            }
            Kind::Set(values) => {
                let set_ctor = agent.intrinsic("%Set%");
                let to = ops::construct(agent, set_ctor, &[], None)?;
                self.remember(agent, o, to);
                for &v in &values {
                    agent.push_root(v);
                }
                let add = agent.persisted(&self.methods.set_add);
                for v in values {
                    let v = self.clone_value(agent, v)?;
                    ops::call(agent, add, Value::Object(to), &[v])?;
                }
                return Ok(Value::Object(to));
            }
            Kind::Other => {}
        }
        match &agent.obj(o).kind {
            ObjectKind::ArrayBuffer(_) => self.clone_array_buffer(agent, o).map(Value::Object),
            ObjectKind::TypedArray(t) => {
                let (name, buffer, offset, length) = (
                    t.kind.name(),
                    t.buffer.expect("a typed array's buffer"),
                    t.byte_offset,
                    t.array_length,
                );
                let buffer = self.clone_value(agent, Value::Object(buffer))?;
                let ctor = agent.intrinsic(&format!("%{name}%"));
                let length = length.map_or(Value::Undefined, |n| Value::Number(n as f64));
                let args = [buffer, Value::Number(offset as f64), length];
                let to = ops::construct(agent, ctor, &args, None)?;
                self.remember(agent, o, to);
                Ok(Value::Object(to))
            }
            ObjectKind::DataView(d) => {
                let (buffer, offset, length) = (d.buffer, d.byte_offset, d.byte_length);
                let buffer = self.clone_value(agent, Value::Object(buffer))?;
                let ctor = agent.intrinsic("%DataView%");
                let length = length.map_or(Value::Undefined, |n| Value::Number(n as f64));
                let args = [buffer, Value::Number(offset as f64), length];
                let to = ops::construct(agent, ctor, &args, None)?;
                self.remember(agent, o, to);
                Ok(Value::Object(to))
            }
            ObjectKind::Error => self.clone_error(agent, o).map(Value::Object),
            ObjectKind::Array => {
                let length_key = agent.key("length");
                let length = ops::get(agent, o, length_key)?;
                let length = ops::to_length(agent, length)?;
                let proto = agent.intrinsic("%Array.prototype%");
                let to = kahvi_core::object::array::array_create(agent, length, Some(proto))?;
                self.remember(agent, o, to);
                self.copy_properties(agent, o, to)?;
                Ok(Value::Object(to))
            }
            ObjectKind::Ordinary | ObjectKind::Arguments(_) => {
                let proto = agent.intrinsic("%Object.prototype%");
                let to = agent.ordinary_object_create(Some(proto));
                self.remember(agent, o, to);
                self.copy_properties(agent, o, to)?;
                Ok(Value::Object(to))
            }
            _ => Err(data_clone_error(agent, "the object could not be cloned")),
        }
    }

    fn clone_array_buffer(&mut self, agent: &mut Agent, o: Object) -> JsResult<Object> {
        let data = array_buffer_data(agent, o).expect("an ArrayBuffer");
        // A SharedArrayBuffer would need its Shared Data Block shared by the
        // clone, which this engine cannot represent (one block per object).
        if data.shared {
            return Err(data_clone_error(
                agent,
                "a SharedArrayBuffer could not be cloned",
            ));
        }
        let Some(bytes) = data.data.clone() else {
            return Err(data_clone_error(
                agent,
                "a detached ArrayBuffer could not be cloned",
            ));
        };
        let max = data.max_byte_length;
        let ctor = agent.intrinsic("%ArrayBuffer%");
        let mut args = vec![Value::Number(bytes.len() as f64)];
        if let Some(max) = max {
            let proto = agent.intrinsic("%Object.prototype%");
            let options = agent.ordinary_object_create(Some(proto));
            agent.push_root(Value::Object(options));
            let key = agent.key("maxByteLength");
            ops::create_data_property_or_throw(agent, options, key, Value::Number(max as f64))?;
            args.push(Value::Object(options));
        }
        let to = ops::construct(agent, ctor, &args, None)?;
        if let ObjectKind::ArrayBuffer(d) = &mut agent.obj_mut(to).kind {
            d.data = Some(bytes);
        }
        self.remember(agent, o, to);
        Ok(to)
    }

    fn clone_error(&mut self, agent: &mut Agent, o: Object) -> JsResult<Object> {
        let name_key = agent.key("name");
        let name = ops::get(agent, o, name_key)?;
        let name = match name {
            Value::String(s) => agent.str_lossy(s),
            _ => String::new(),
        };
        let name = match name.as_str() {
            "EvalError" | "RangeError" | "ReferenceError" | "SyntaxError" | "TypeError"
            | "URIError" => name,
            _ => "Error".to_string(),
        };
        let message_key = agent.key("message");
        let message = match o.internal_get_own_property(agent, message_key)? {
            Some(desc) if desc.is_data_descriptor() => {
                let v = desc.value.unwrap_or(Value::Undefined);
                Some(Value::String(ops::to_string(agent, v)?))
            }
            _ => None,
        };
        let ctor = agent.intrinsic(&format!("%{name}%"));
        let args: Vec<Value> = message.into_iter().collect();
        let to = ops::construct(agent, ctor, &args, None)?;
        self.remember(agent, o, to);
        Ok(to)
    }

    /// The own enumerable string-keyed properties of `from`, cloned onto `to`.
    fn copy_properties(&mut self, agent: &mut Agent, from: Object, to: Object) -> JsResult<()> {
        let keys = ops::enumerable_own_properties(agent, from, ops::EnumerableKind::Key)?;
        for &k in &keys {
            agent.push_root(k);
        }
        for k in keys {
            let key = ops::to_property_key(agent, k)?;
            if ops::has_own_property(agent, from, key)? {
                let v = ops::get(agent, from, key)?;
                let v = self.clone_value(agent, v)?;
                ops::create_data_property(agent, to, key, v)?;
            }
        }
        Ok(())
    }
}

/// HTML throws a "DataCloneError" DOMException; this runtime has no
/// DOMException, so it is an Error named DataCloneError.
fn data_clone_error(agent: &mut Agent, message: &str) -> kahvi::JsError {
    let e = agent.create_error(ErrorKind::Error, message);
    let name_key = agent.key("name");
    let name = agent.string("DataCloneError");
    ops::create_data_property_or_throw(agent, e, name_key, Value::String(name))
        .expect("a new error object takes a name");
    kahvi::JsError::Throw(Value::Object(e))
}
