//! The kahvi engine core: values, objects, environments, realms, abstract
//! operations, the bytecode compiler and the VM.
//!
//! Built-in objects live in `kahvi-builtins`; the embedding API in `kahvi`.

pub mod agent;
pub mod compiler;
pub mod env;
pub mod error;
pub mod function;
pub mod object;
pub mod ops;
pub mod realm;
pub mod value;
pub mod vm;

pub use agent::{Agent, ErrorKind};
pub use error::{JsError, JsResult};
pub use object::{Property, PropertyDescriptor, PropertyKey};
pub use realm::RealmId;
pub use value::{Args, BigInt, JsString, Object, Symbol, Value};

/// [[ScriptOrModule]]: which loaded script or module code came from (an
/// index into `Agent::scripts`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptOrModule {
    Script(u32),
    Module(u32),
}

/// Everything a builtin implementation usually needs.
pub mod prelude {
    pub use crate::agent::{Agent, ErrorKind};
    pub use crate::error::{JsError, JsResult};
    pub use crate::function::{NativeFn, builtin, create_builtin_function};
    pub use crate::object::{ObjectKind, Property, PropertyDescriptor, PropertyKey};
    pub use crate::ops;
    pub use crate::realm::RealmId;
    pub use crate::value::{Args, BigInt, JsString, Object, Symbol, Value};
}
