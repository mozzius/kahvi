//! `process.env`: the environment variables, as strings, for the many
//! packages that read `process.env.NODE_ENV`. Nothing else of Node's
//! `process` is provided.

use kahvi::{Runtime, Value};
use kahvi_core::prelude::*;

pub fn install(rt: &mut Runtime) {
    let process = rt.enter(|agent| {
        let proto = agent.intrinsic("%Object.prototype%");
        let process = agent.ordinary_object_create(Some(proto));
        let env = agent.ordinary_object_create(Some(proto));
        for (name, value) in std::env::vars_os() {
            let key = agent.key(&name.to_string_lossy());
            let value = agent.string(&value.to_string_lossy());
            ops::create_data_property_or_throw(agent, env, key, Value::String(value))
                .expect("a new object takes properties");
        }
        let env_key = agent.key("env");
        ops::create_data_property_or_throw(agent, process, env_key, Value::Object(env))
            .expect("a new object takes properties");
        process
    });
    rt.set_global("process", Value::Object(process));
}
