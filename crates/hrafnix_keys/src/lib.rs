//! Keys and associated traits.
//!
//! Runtime key strings are private; use their accessors or conversions to read them.
//! Runtime keys can be interned in place with `key.launder(&store)` without
//! changing their validated contents or hashes.

// Test code favors clarity and brevity over the strictness we require of library code:
// panicking helpers (`unwrap`/`expect`/indexing/`panic!`) and approximate float comparisons
// are idiomatic and expected in tests.
#![cfg_attr(
    test,
    allow(
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::indexing_slicing,
        clippy::panic,
        clippy::float_cmp,
        clippy::as_conversions,
        clippy::cast_precision_loss,
        clippy::cast_sign_loss,
        clippy::cast_possible_truncation,
        clippy::unreadable_literal,
        clippy::unnecessary_wraps,
        clippy::similar_names,
        clippy::arithmetic_side_effects
    )
)]

/// Common utilities for keys.
pub(crate) mod common;
/// Keys for components.
pub mod component_key;
/// Keys for global objects.
pub mod global_key;
/// Keys for parameters.
pub mod parameter_key;
/// Keys for ports.
pub mod port_key;
/// Keys for store objects.
pub mod store_key;
/// Keys for units.
pub mod unit_key;
/// Keys for variables.
pub mod variable_key;

pub use component_key::*;
pub use global_key::*;
pub use parameter_key::*;
pub use port_key::*;
pub use store_key::*;
pub use unit_key::*;
pub use variable_key::*;

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_shareable_string::SharedStringStore;

    #[test]
    fn laundering_keys_preserves_contents_and_hashes() {
        let store = SharedStringStore::new();
        macro_rules! check {
            ($type:ty, $text:literal) => {{
                let interned = store.get($text);
                let mut key = <$type>::new($text.into()).unwrap();
                let original = key.clone();
                let hash = key.current_blake3_hash();
                assert!(!key.as_shareable_string().ptr_eq(&interned));
                key.launder(&store);
                assert_eq!(key, original);
                assert_eq!(key.current_blake3_hash(), hash);
                assert!(key.as_shareable_string().ptr_eq(&interned));
                assert!(!original.as_shareable_string().ptr_eq(&interned));
                key.launder(&store);
                assert!(key.as_shareable_string().ptr_eq(&interned));
            }};
        }
        check!(ComponentKey, "component");
        check!(GlobalKey, "g_setting");
        check!(ParameterKey, "p_parameter");
        check!(PortKey, "port");
        check!(StoreKey, "value");
        check!(UnitKey, "u_unit");
        check!(VariableKey, "v_variable");
    }
}
