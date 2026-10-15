//! The built-in objects (ECMA-262 §19-28).
//!
//! Every built-in function has its own file under a directory per object
//! (`array/prototype_map.rs` is `Array.prototype.map`), generated from the
//! spec by `harness/gen/builtins.py` with the algorithm steps as comments.
//! `generated.rs` lists them all; `table.rs` installs them into a realm.
//! Helpers shared by several built-ins go in `support/`.

mod extensions;
mod generated;
pub mod introspect;
mod support;
mod table;

pub use table::install;
