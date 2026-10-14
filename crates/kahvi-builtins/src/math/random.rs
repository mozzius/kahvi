//! Math.random ( )
//!
//! Spec: https://tc39.es/ecma262/#sec-math.random

use kahvi_core::prelude::*;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};

/// Golden-ratio increment of the SplitMix64 generator.
const GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

/// Generator state shared by every realm. Each call advances it by one step,
/// so successive calls (from any realm) never replay an earlier state.
static STATE: OnceLock<AtomicU64> = OnceLock::new();

/// Returns the next 64 random bits (SplitMix64 over the shared state).
fn next_u64() -> u64 {
    let state = STATE.get_or_init(|| {
        // Seed from the std hasher's per-process random keys; no extra dependency needed.
        let mut hasher = RandomState::new().build_hasher();
        hasher.write_u64(0);
        AtomicU64::new(hasher.finish())
    });
    let mut z = state
        .fetch_add(GAMMA, Ordering::Relaxed)
        .wrapping_add(GAMMA);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Math.random
pub(crate) fn random(
    agent: &mut Agent,
    this: Value,
    args: &[Value],
    new_target: Option<Object>,
) -> JsResult<Value> {
    // This function returns a Number value with positive sign, greater than or equal to *+0*_𝔽 but strictly less than *1*_𝔽, chosen randomly or pseudo randomly with approximately uniform distribution over that range, using an implementation-defined algorithm or strategy.
    // Each `Math.random` function created for distinct realms must produce a distinct sequence of values from successive calls.
    let _ = (agent, this, args, new_target);
    // Take the top 53 bits so the value is an exact multiple of 2^-53, which lies in [0, 1).
    let bits = next_u64() >> 11;
    Ok(Value::Number(bits as f64 * (1.0 / 9_007_199_254_740_992.0)))
}
