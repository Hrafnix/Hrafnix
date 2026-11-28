//! Version 1 JSON deserialization into validated file data.

/// Component contents.
pub(crate) mod component;
/// Saved component instances.
pub(crate) mod component_instance;
/// Connections and their drawing paths.
pub(crate) mod connection;
/// Shared datastore definitions and frozen values.
pub(crate) mod datastore;
/// Background decorations.
pub(crate) mod decoration_background;
/// Foreground decorations.
pub(crate) mod decoration_foreground;
/// Ordered parameter definitions, preserving descriptions and defaults.
pub(crate) mod definition_parameter;
/// Ordered variable definitions, preserving descriptions and defaults.
pub(crate) mod definition_variable;
/// Ordered frozen global values with embedded definitions and unique keys.
pub(crate) mod frozen_global;
/// Ordered frozen parameter values with embedded definitions and unique keys.
pub(crate) mod frozen_parameter;
/// Geometry and styling values.
pub(crate) mod graphics;
/// General metadata.
pub(crate) mod metadata_general;
/// Component drawing metadata.
pub(crate) mod metadata_gui;
/// Model contents.
pub(crate) mod model;
/// Component source references.
pub(crate) mod source;
/// Source languages and translated text.
pub(crate) mod translations;
