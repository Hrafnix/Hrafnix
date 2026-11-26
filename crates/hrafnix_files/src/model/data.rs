use crate::common::hash::model_file_hash;
use crate::component_instance::data::ComponentInstance;
use crate::connection::data::Connection;
use crate::decoration_background::data::BackgroundDecoration;
use crate::decoration_foreground::data::ForegroundDecoration;
use crate::metadata_general::data::MetadataFile;
use crate::translations::data::TranslationsFile;
use hrafnix_datastore::definition::{ParameterObjectDefinition, VariableObjectDefinition};
use hrafnix_datastore::frozen::GlobalObjectFrozen;
use hrafnix_shareable_string::SharedStringStore;

/// Represents User Model data from a Hrafnix model file.
#[derive(Debug, Clone)]
pub struct ModelFile {
    /// Descriptive information about the model.
    metadata: MetadataFile,
    /// Text and images drawn in front of the model's contents.
    foreground_decorations: Vec<ForegroundDecoration>,
    /// Shapes drawn behind the model's contents.
    background_decorations: Vec<BackgroundDecoration>,
    /// Global settings for the model, including simulation parameters and other configuration.
    global_settings: GlobalObjectFrozen,
    /// Ordered component instances with saved GUI and parameter snapshots.
    components: Vec<ComponentInstance>,
    /// Connections between component ports within the model.
    connections: Vec<Connection>,
    /// Source language and translated text used by the model.
    translations: TranslationsFile,
    /// Ordered parameter definitions, including defaults and descriptions.
    parameter_definition: ParameterObjectDefinition,
    /// Ordered variable definitions, including defaults and descriptions.
    variable_definition: VariableObjectDefinition,
    /// BLAKE3 hash of all validated model data.
    file_hash: [u8; 32],
}

impl ModelFile {
    /// Creates a new model file and computes its file hash.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        metadata: MetadataFile,
        foreground_decorations: Vec<ForegroundDecoration>,
        background_decorations: Vec<BackgroundDecoration>,
        global_settings: GlobalObjectFrozen,
        components: Vec<ComponentInstance>,
        connections: Vec<Connection>,
        translations: TranslationsFile,
        parameter_definition: ParameterObjectDefinition,
        variable_definition: VariableObjectDefinition,
    ) -> Self {
        let mut model = Self {
            metadata,
            foreground_decorations,
            background_decorations,
            global_settings,
            components,
            connections,
            translations,
            parameter_definition,
            variable_definition,
            file_hash: [0; 32],
        };
        model.file_hash = model_file_hash(&model);
        model
    }

    /// Returns descriptive information about the model.
    #[must_use]
    pub const fn metadata(&self) -> &MetadataFile {
        &self.metadata
    }

    /// Returns the text and images drawn in front of the model's contents.
    #[must_use]
    pub fn foreground_decorations(&self) -> &[ForegroundDecoration] {
        &self.foreground_decorations
    }

    /// Returns the shapes drawn behind the model's contents.
    #[must_use]
    pub fn background_decorations(&self) -> &[BackgroundDecoration] {
        &self.background_decorations
    }

    /// Returns the global settings for the model.
    #[must_use]
    pub const fn global_settings(&self) -> &GlobalObjectFrozen {
        &self.global_settings
    }

    /// Returns the ordered component instances with saved GUI and parameter snapshots.
    #[must_use]
    pub fn components(&self) -> &[ComponentInstance] {
        &self.components
    }

    /// Returns the connections between component ports within the model.
    #[must_use]
    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    /// Returns the source language and translated text used by the model.
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

    /// Returns the BLAKE3 hash of all validated model data.
    #[must_use]
    pub const fn file_hash(&self) -> [u8; 32] {
        self.file_hash
    }

    /// Interns this value's strings in `store` in place, preserving its contents.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.metadata.launder(store);
        self.global_settings.launder(store);
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

impl PartialEq for ModelFile {
    fn eq(&self, other: &Self) -> bool {
        self.file_hash == other.file_hash
            && self.metadata == other.metadata
            && self.foreground_decorations == other.foreground_decorations
            && self.background_decorations == other.background_decorations
            && self.global_settings == other.global_settings
            && self.components == other.components
            && self.connections == other.connections
            && self.translations == other.translations
            && self.parameter_definition == other.parameter_definition
            && self.variable_definition == other.variable_definition
    }
}
