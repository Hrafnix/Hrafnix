use crate::common::hash::{component_file_hash, component_interface_hash};
use crate::data::component_instance::ComponentInstance;
use crate::data::connection::Connection;
use crate::data::decoration_background::BackgroundDecoration;
use crate::data::decoration_foreground::ForegroundDecoration;
use crate::data::metadata_general::MetadataFile;
use crate::data::metadata_gui::GuiMetadata;
use crate::data::source::ComponentSource;
use crate::data::translations::TranslationsFile;
use hrafnix_datastore::definition::{ParameterObjectDefinition, VariableObjectDefinition};
use hrafnix_shareable_string::SharedStringStore;

/// Represents User component data from a Hrafnix component file.
#[derive(Debug, Clone)]
pub struct ComponentFile {
    /// Descriptive information about the component.
    metadata: MetadataFile,
    /// Unresolved provenance of the loaded component file.
    source: ComponentSource,
    /// Drawing dimensions, rotation, icon, and port locations associated with the component.
    gui_metadata: GuiMetadata,
    /// Text and images drawn in front of the component's contents.
    foreground_decorations: Vec<ForegroundDecoration>,
    /// Shapes drawn behind the component's contents.
    background_decorations: Vec<BackgroundDecoration>,
    /// Ordered nested component instances with saved GUI and parameter snapshots.
    components: Vec<ComponentInstance>,
    /// Connections between component ports within the component.
    connections: Vec<Connection>,
    /// Source language and translated text used by the component.
    translations: TranslationsFile,
    /// Ordered parameter definitions, including defaults and descriptions.
    parameter_definition: ParameterObjectDefinition,
    /// Ordered variable definitions, including defaults and descriptions.
    variable_definition: VariableObjectDefinition,
    /// BLAKE3 hash of all validated component data, including source provenance.
    file_hash: [u8; 32],
    /// BLAKE3 hash of the component's metadata, source, GUI, and parameter interface.
    interface_hash: [u8; 32],
}

impl ComponentFile {
    /// Creates a new component file and computes its file and interface hashes.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        metadata: MetadataFile,
        source: ComponentSource,
        gui_metadata: GuiMetadata,
        foreground_decorations: Vec<ForegroundDecoration>,
        background_decorations: Vec<BackgroundDecoration>,
        components: Vec<ComponentInstance>,
        connections: Vec<Connection>,
        translations: TranslationsFile,
        parameter_definition: ParameterObjectDefinition,
        variable_definition: VariableObjectDefinition,
    ) -> Self {
        let interface_hash =
            component_interface_hash(&metadata, &source, &gui_metadata, &parameter_definition);
        let mut component = Self {
            metadata,
            source,
            gui_metadata,
            foreground_decorations,
            background_decorations,
            components,
            connections,
            translations,
            parameter_definition,
            variable_definition,
            file_hash: [0; 32],
            interface_hash,
        };
        component.file_hash = component_file_hash(&component);
        component
    }

    /// Returns descriptive information about the component.
    #[must_use]
    pub const fn metadata(&self) -> &MetadataFile {
        &self.metadata
    }

    /// Returns the unresolved provenance of the loaded component file.
    #[must_use]
    pub const fn source(&self) -> &ComponentSource {
        &self.source
    }

    /// Returns the drawing dimensions, rotation, icon, and port locations associated with the component.
    #[must_use]
    pub const fn gui_metadata(&self) -> &GuiMetadata {
        &self.gui_metadata
    }

    /// Returns the text and images drawn in front of the component's contents.
    #[must_use]
    pub fn foreground_decorations(&self) -> &[ForegroundDecoration] {
        &self.foreground_decorations
    }

    /// Returns the shapes drawn behind the component's contents.
    #[must_use]
    pub fn background_decorations(&self) -> &[BackgroundDecoration] {
        &self.background_decorations
    }

    /// Returns the ordered nested component instances with saved GUI and parameter snapshots.
    #[must_use]
    pub fn components(&self) -> &[ComponentInstance] {
        &self.components
    }

    /// Returns the connections between component ports within the component.
    #[must_use]
    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    /// Returns the source language and translated text used by the component.
    #[must_use]
    pub const fn translations(&self) -> &TranslationsFile {
        &self.translations
    }

    /// Returns the ordered parameter definitions, including defaults and descriptions.
    #[must_use]
    pub const fn parameter_definition(&self) -> &ParameterObjectDefinition {
        &self.parameter_definition
    }

    /// Returns the ordered variable definitions, including defaults and descriptions.
    #[must_use]
    pub const fn variable_definition(&self) -> &VariableObjectDefinition {
        &self.variable_definition
    }

    /// Returns the BLAKE3 hash of all validated component data, including source provenance.
    #[must_use]
    pub const fn file_hash(&self) -> [u8; 32] {
        self.file_hash
    }

    /// Returns the BLAKE3 hash of the component's metadata, source, GUI, and parameter interface.
    #[must_use]
    pub const fn interface_hash(&self) -> [u8; 32] {
        self.interface_hash
    }

    /// Interns this value's strings in `store` in place, preserving its contents.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.metadata.launder(store);
        self.source.launder(store);
        self.gui_metadata.launder(store);
        self.translations.launder(store);
        self.parameter_definition.launder(store);
        self.variable_definition.launder(store);
        for value in &mut self.foreground_decorations {
            value.launder(store);
        }
        for value in &mut self.background_decorations {
            value.launder(store);
        }
        for value in &mut self.components {
            value.launder(store);
        }
        for value in &mut self.connections {
            value.launder(store);
        }
    }
}

impl PartialEq for ComponentFile {
    fn eq(&self, other: &Self) -> bool {
        self.file_hash == other.file_hash
            && self.interface_hash == other.interface_hash
            && self.metadata == other.metadata
            && self.source == other.source
            && self.gui_metadata == other.gui_metadata
            && self.foreground_decorations == other.foreground_decorations
            && self.background_decorations == other.background_decorations
            && self.components == other.components
            && self.connections == other.connections
            && self.translations == other.translations
            && self.parameter_definition == other.parameter_definition
            && self.variable_definition == other.variable_definition
    }
}
