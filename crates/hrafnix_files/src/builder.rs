//! Public builders for constructing validated file data snapshots.
//!
//! File-data `new` constructors remain crate-private so callers build snapshots
//! through these builders, which preserve invariants (such as capping port
//! coordinates to icon dimensions) and compute BLAKE3 hashes.

use crate::data::component::ComponentFile;
use crate::data::component_instance::ComponentInstance;
use crate::data::connection::{Connection, ConnectionLine};
use crate::data::decoration_background::{BackgroundDecoration, BackgroundDecorationKind};
use crate::data::decoration_foreground::{
    ForegroundDecoration, ForegroundDecorationKind, ImageDecoration, ImageFit, TextDecoration,
};
use crate::data::graphics::{Dimension, FontSize, Position, Rotation};
use crate::data::metadata_general::MetadataFile;
use crate::data::metadata_gui::{GuiMetadata, Port, PortLocation};
use crate::data::model::ModelFile;
use crate::data::project::ProjectFile;
use crate::data::source::ComponentSource;
use crate::data::translations::TranslationsFile;
use camino::Utf8PathBuf;
use hrafnix_common::port::PortType;
use hrafnix_datastore::definition::{ParameterObjectDefinition, VariableObjectDefinition};
use hrafnix_datastore::frozen::{GlobalObjectFrozen, ParameterObjectFrozen};
use hrafnix_keys::{ComponentKey, PortKey};
use hrafnix_shareable_string::ShareableString;
use std::collections::BTreeMap;

/// Creates a [`ProjectFile`] with the required project name and version.
#[must_use]
pub const fn project_file(name: ShareableString, version: ShareableString) -> ProjectFile {
    ProjectFile::new(name, version)
}

/// Creates a [`Position`] in signed 32-bit drawing coordinates.
#[must_use]
pub const fn position(x: i32, y: i32) -> Position {
    Position::new(x, y)
}

/// Creates a [`Dimension`] in unsigned 16-bit drawing coordinates.
#[must_use]
pub const fn dimension(width: u16, height: u16) -> Dimension {
    Dimension::new(width, height)
}

/// Creates a [`FontSize`] in hundredths of a drawing unit.
#[must_use]
pub const fn font_size(size: u16) -> FontSize {
    FontSize::new(size)
}

/// Creates a [`PortLocation`] in icon-local drawing coordinates.
#[must_use]
pub const fn port_location(x: u16, y: u16) -> PortLocation {
    PortLocation::new(x, y)
}

/// Creates a [`Port`] with the given identifier, type, and icon-local location.
#[must_use]
pub const fn port(id: PortKey, port_type: PortType, location: PortLocation) -> Port {
    Port::new(id, port_type, location)
}

/// Creates a [`TextDecoration`] with the given text and font size.
#[must_use]
pub const fn text_decoration(text: ShareableString, font_size: FontSize) -> TextDecoration {
    TextDecoration::new(text, font_size)
}

/// Creates an [`ImageDecoration`] with the given image path and fit mode.
#[must_use]
pub const fn image_decoration(path: Utf8PathBuf, fit: ImageFit) -> ImageDecoration {
    ImageDecoration::new(path, fit)
}

/// Creates a [`ConnectionLine`] with the given ordered midpoints.
#[must_use]
pub const fn connection_line(midpoints: Vec<Position>) -> ConnectionLine {
    ConnectionLine::new(midpoints)
}

/// Creates a directed [`Connection`] between two component ports.
#[must_use]
pub const fn connection(
    from_component: ComponentKey,
    from_port: PortKey,
    to_component: ComponentKey,
    to_port: PortKey,
    line: ConnectionLine,
) -> Connection {
    Connection::new(from_component, from_port, to_component, to_port, line)
}

/// Creates a [`BackgroundDecoration`].
#[must_use]
pub const fn background_decoration(
    id: ShareableString,
    position: Position,
    dimensions: Dimension,
    kind: BackgroundDecorationKind,
) -> BackgroundDecoration {
    BackgroundDecoration::new(id, position, dimensions, kind)
}

/// Creates a [`ForegroundDecoration`].
#[must_use]
pub const fn foreground_decoration(
    id: ShareableString,
    position: Position,
    dimensions: Dimension,
    kind: ForegroundDecorationKind,
) -> ForegroundDecoration {
    ForegroundDecoration::new(id, position, dimensions, kind)
}

/// Builder for [`MetadataFile`].
#[derive(Debug, Clone)]
pub struct MetadataFileBuilder {
    /// Name shown to users.
    display_name: ShareableString,
    /// Free-form description.
    description: ShareableString,
    /// Author of the file.
    author: ShareableString,
    /// Major part of the minimum Hrafnix version required to open the file.
    minimum_version_major: u16,
    /// Minor part of the minimum Hrafnix version required to open the file.
    minimum_version_minor: u16,
}

impl MetadataFileBuilder {
    /// Creates a new builder with the required descriptive metadata fields.
    #[must_use]
    pub const fn new(
        display_name: ShareableString,
        description: ShareableString,
        author: ShareableString,
        minimum_version_major: u16,
        minimum_version_minor: u16,
    ) -> Self {
        Self {
            display_name,
            description,
            author,
            minimum_version_major,
            minimum_version_minor,
        }
    }

    /// Builds the [`MetadataFile`].
    #[must_use]
    pub fn build(self) -> MetadataFile {
        MetadataFile::new(
            self.display_name,
            self.description,
            self.author,
            self.minimum_version_major,
            self.minimum_version_minor,
        )
    }
}

/// Builder for [`TranslationsFile`].
#[derive(Debug, Clone)]
pub struct TranslationsFileBuilder {
    /// Source language of the translation keys.
    language: ShareableString,
    /// Original text mapped to target language codes and their translated text.
    translations: BTreeMap<ShareableString, BTreeMap<ShareableString, ShareableString>>,
}

impl TranslationsFileBuilder {
    /// Creates a new builder with the given source language and empty translations map.
    #[must_use]
    pub const fn new(language: ShareableString) -> Self {
        Self {
            language,
            translations: BTreeMap::new(),
        }
    }

    /// Replaces the entire translation map.
    #[must_use]
    pub fn translations(
        mut self,
        translations: BTreeMap<ShareableString, BTreeMap<ShareableString, ShareableString>>,
    ) -> Self {
        self.translations = translations;
        self
    }

    /// Builds the [`TranslationsFile`].
    #[must_use]
    pub fn build(self) -> TranslationsFile {
        TranslationsFile::new(self.language, self.translations)
    }
}

/// Builder for [`GuiMetadata`].
#[derive(Debug, Clone)]
pub struct GuiMetadataBuilder {
    /// Unrotated dimensions in drawing coordinates.
    dimensions: Dimension,
    /// Clockwise quarter-turn rotation about the center of the icon bounds.
    rotation: Rotation,
    /// Icon path, stored unchanged.
    icon: Utf8PathBuf,
    /// Ports placed within the icon's drawing dimensions.
    ports: Vec<Port>,
}

impl GuiMetadataBuilder {
    /// Creates a new builder with the required dimensions and icon path.
    #[must_use]
    pub const fn new(dimensions: Dimension, icon: Utf8PathBuf) -> Self {
        Self {
            dimensions,
            rotation: Rotation::Degrees0,
            icon,
            ports: Vec::new(),
        }
    }

    /// Sets the clockwise rotation.
    #[must_use]
    pub const fn rotation(mut self, rotation: Rotation) -> Self {
        self.rotation = rotation;
        self
    }

    /// Sets the ports list; coordinates are capped to `dimensions` on [`Self::build`].
    #[must_use]
    pub fn ports(mut self, ports: Vec<Port>) -> Self {
        self.ports = ports;
        self
    }

    /// Builds the [`GuiMetadata`], capping port coordinates to the declared dimensions.
    #[must_use]
    pub fn build(self) -> GuiMetadata {
        let max_x = self.dimensions.width();
        let max_y = self.dimensions.height();
        let ports = self
            .ports
            .into_iter()
            .map(|p| {
                let loc =
                    PortLocation::new(p.location().x().min(max_x), p.location().y().min(max_y));
                Port::new(p.id().clone(), p.port_type(), loc)
            })
            .collect();
        GuiMetadata::new(self.dimensions, self.rotation, self.icon, ports)
    }
}

/// Builder for [`ComponentInstance`].
#[derive(Debug, Clone)]
pub struct ComponentInstanceBuilder {
    /// Identifier unique within the containing model or component.
    id: ComponentKey,
    /// Unresolved reference to the instance's component definition.
    source: ComponentSource,
    /// Position in the containing drawing.
    position: Position,
    /// Saved GUI metadata snapshot.
    gui_metadata: GuiMetadata,
    /// Frozen parameter definitions and values.
    parameters: ParameterObjectFrozen,
}

impl ComponentInstanceBuilder {
    /// Creates a new builder for a component instance.
    #[must_use]
    pub const fn new(
        id: ComponentKey,
        source: ComponentSource,
        position: Position,
        gui_metadata: GuiMetadata,
        parameters: ParameterObjectFrozen,
    ) -> Self {
        Self {
            id,
            source,
            position,
            gui_metadata,
            parameters,
        }
    }

    /// Builds the [`ComponentInstance`] and computes its BLAKE3 interface hash.
    #[must_use]
    pub fn build(self) -> ComponentInstance {
        ComponentInstance::new(
            self.id,
            self.source,
            self.position,
            self.gui_metadata,
            self.parameters,
        )
    }
}

/// Builder for [`ModelFile`].
#[derive(Debug, Clone)]
pub struct ModelFileBuilder {
    /// Descriptive metadata.
    metadata: MetadataFile,
    /// Foreground decorations.
    foreground_decorations: Vec<ForegroundDecoration>,
    /// Background decorations.
    background_decorations: Vec<BackgroundDecoration>,
    /// Frozen global settings.
    global_settings: GlobalObjectFrozen,
    /// Ordered component instances.
    components: Vec<ComponentInstance>,
    /// Connections between component ports.
    connections: Vec<Connection>,
    /// Source language and translations.
    translations: TranslationsFile,
    /// Ordered parameter definitions.
    parameter_definition: ParameterObjectDefinition,
    /// Ordered variable definitions.
    variable_definition: VariableObjectDefinition,
}

impl ModelFileBuilder {
    /// Creates a new builder with the required metadata, global settings, and translations.
    #[must_use]
    pub fn new(
        metadata: MetadataFile,
        global_settings: GlobalObjectFrozen,
        translations: TranslationsFile,
    ) -> Self {
        Self {
            metadata,
            foreground_decorations: Vec::new(),
            background_decorations: Vec::new(),
            global_settings,
            components: Vec::new(),
            connections: Vec::new(),
            translations,
            parameter_definition: ParameterObjectDefinition::default(),
            variable_definition: VariableObjectDefinition::default(),
        }
    }

    /// Sets the foreground decorations.
    #[must_use]
    pub fn foreground_decorations(mut self, decorations: Vec<ForegroundDecoration>) -> Self {
        self.foreground_decorations = decorations;
        self
    }

    /// Sets the background decorations.
    #[must_use]
    pub fn background_decorations(mut self, decorations: Vec<BackgroundDecoration>) -> Self {
        self.background_decorations = decorations;
        self
    }

    /// Sets the ordered component instances.
    #[must_use]
    pub fn components(mut self, components: Vec<ComponentInstance>) -> Self {
        self.components = components;
        self
    }

    /// Sets the connections between component ports.
    #[must_use]
    pub fn connections(mut self, connections: Vec<Connection>) -> Self {
        self.connections = connections;
        self
    }

    /// Sets the parameter definitions.
    #[must_use]
    pub fn parameter_definition(mut self, definition: ParameterObjectDefinition) -> Self {
        self.parameter_definition = definition;
        self
    }

    /// Sets the variable definitions.
    #[must_use]
    pub fn variable_definition(mut self, definition: VariableObjectDefinition) -> Self {
        self.variable_definition = definition;
        self
    }

    /// Builds the [`ModelFile`] and computes its BLAKE3 file hash.
    #[must_use]
    pub fn build(self) -> ModelFile {
        ModelFile::new(
            self.metadata,
            self.foreground_decorations,
            self.background_decorations,
            self.global_settings,
            self.components,
            self.connections,
            self.translations,
            self.parameter_definition,
            self.variable_definition,
        )
    }
}

/// Builder for [`ComponentFile`].
#[derive(Debug, Clone)]
pub struct ComponentFileBuilder {
    /// Descriptive metadata.
    metadata: MetadataFile,
    /// Unresolved provenance of the component file.
    source: ComponentSource,
    /// GUI metadata for the component.
    gui_metadata: GuiMetadata,
    /// Foreground decorations.
    foreground_decorations: Vec<ForegroundDecoration>,
    /// Background decorations.
    background_decorations: Vec<BackgroundDecoration>,
    /// Ordered nested component instances.
    components: Vec<ComponentInstance>,
    /// Connections between component ports.
    connections: Vec<Connection>,
    /// Source language and translations.
    translations: TranslationsFile,
    /// Ordered parameter definitions.
    parameter_definition: ParameterObjectDefinition,
    /// Ordered variable definitions.
    variable_definition: VariableObjectDefinition,
}

impl ComponentFileBuilder {
    /// Creates a new builder with the required metadata, source, GUI metadata, and translations.
    #[must_use]
    pub fn new(
        metadata: MetadataFile,
        source: ComponentSource,
        gui_metadata: GuiMetadata,
        translations: TranslationsFile,
    ) -> Self {
        Self {
            metadata,
            source,
            gui_metadata,
            foreground_decorations: Vec::new(),
            background_decorations: Vec::new(),
            components: Vec::new(),
            connections: Vec::new(),
            translations,
            parameter_definition: ParameterObjectDefinition::default(),
            variable_definition: VariableObjectDefinition::default(),
        }
    }

    /// Sets the foreground decorations.
    #[must_use]
    pub fn foreground_decorations(mut self, decorations: Vec<ForegroundDecoration>) -> Self {
        self.foreground_decorations = decorations;
        self
    }

    /// Sets the background decorations.
    #[must_use]
    pub fn background_decorations(mut self, decorations: Vec<BackgroundDecoration>) -> Self {
        self.background_decorations = decorations;
        self
    }

    /// Sets the ordered nested component instances.
    #[must_use]
    pub fn components(mut self, components: Vec<ComponentInstance>) -> Self {
        self.components = components;
        self
    }

    /// Sets the connections between component ports.
    #[must_use]
    pub fn connections(mut self, connections: Vec<Connection>) -> Self {
        self.connections = connections;
        self
    }

    /// Sets the parameter definitions.
    #[must_use]
    pub fn parameter_definition(mut self, definition: ParameterObjectDefinition) -> Self {
        self.parameter_definition = definition;
        self
    }

    /// Sets the variable definitions.
    #[must_use]
    pub fn variable_definition(mut self, definition: VariableObjectDefinition) -> Self {
        self.variable_definition = definition;
        self
    }

    /// Builds the [`ComponentFile`] and computes its BLAKE3 file and interface hashes.
    #[must_use]
    pub fn build(self) -> ComponentFile {
        ComponentFile::new(
            self.metadata,
            self.source,
            self.gui_metadata,
            self.foreground_decorations,
            self.background_decorations,
            self.components,
            self.connections,
            self.translations,
            self.parameter_definition,
            self.variable_definition,
        )
    }
}
