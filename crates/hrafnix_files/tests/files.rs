//! Integration tests for public model and component file APIs.

#![allow(
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
)]

#[path = "files/common/mod.rs"]
mod common;
#[path = "files/component.rs"]
mod component;
#[path = "files/component_instances.rs"]
mod component_instances;
#[path = "files/definitions.rs"]
mod definitions;
#[path = "files/hashes.rs"]
mod hashes;
#[path = "files/model.rs"]
mod model;
