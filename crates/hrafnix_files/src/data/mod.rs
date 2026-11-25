//! In-memory file data, independent of versioned JSON representations.
//!
//! Constructors are crate-private except for package provenance supplied to
//! component readers. Public getters expose validated file contents.

/// Component definitions and their data hashes.
pub mod component;
/// Saved component instances and their unresolved source references.
pub mod component_instance;
/// Connections between component ports and their drawing paths.
pub mod connection;
/// Shapes drawn behind model and component contents.
pub mod decoration_background;
/// Text and images drawn in front of model and component contents.
pub mod decoration_foreground;
/// Geometry and styling values shared by graphical elements.
pub mod graphics;
/// Metadata shared by model and component files.
pub mod metadata_general;
/// Drawing dimensions, rotation, icons, and port locations associated with components.
pub mod metadata_gui;
/// Simulation model definitions and their data hashes.
pub mod model;
/// Project manifest metadata.
pub mod project;
/// Component source references and package provenance.
pub mod source;
/// Source languages and translated text shared by model and component files.
pub mod translations;
