//! Design objects for authoring Hrafnix models, components, and systems.
//!
//! [`project::Project::open_project`] creates an in-memory context without file I/O.
//! [`project::Project::load_project`] reads `hrafnix.toml`, while
//! [`project::Project::create_project`] creates the standard directories and a
//! new manifest without overwriting an existing one. Manifests require a TOML
//! `[project]` table with string `name` and `version` fields. Project save and
//! reload operations affect only manifest metadata, not open models or components.
//!
//! Component sources resolve explicitly through [`project::Project`]. Loose
//! relative file sources use the containing file's directory; packaged relative
//! sources use registered package roots. Built-ins require an exact registered
//! name and version. Resolution preserves stored provenance and does not recurse.
//! Interface refresh rereads file sources, keeps placed position and rotation,
//! and merges compatible parameter values into the latest definitions. Added,
//! removed, and reset parameters are reported; connections are not rewritten.

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

/// Immutable built-in component descriptors and instantiation.
pub mod built_in_component;
/// Editable component instances placed inside models and user components.
pub mod component_instance;
/// Resolved immutable component definitions bound to their requested provenance.
pub mod component_source;
/// Thread-safe cache of loaded model and component file snapshots.
pub mod file_cache;
/// Editable top-level simulation models.
pub mod model;
/// Project context and directory layout shared across open design objects.
pub mod project;
/// Editable design value types decoupled from on-disk file snapshots.
pub mod types;
/// Editable user-authored reusable components.
pub mod user_component;
