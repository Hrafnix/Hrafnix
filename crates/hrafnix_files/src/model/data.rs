use crate::component_instance::data::ComponentInstance;
use crate::connection::data::Connection;
use crate::decoration_background::data::BackgroundDecoration;
use crate::decoration_foreground::data::ForegroundDecoration;
use crate::metadata_general::data::MetadataFile;
use crate::translations::data::TranslationsFile;
use hrafnix_datastore::definition::{ParameterObjectDefinition, VariableObjectDefinition};
use hrafnix_datastore::frozen::GlobalObjectFrozen;

/// Represents User Model data from a Hrafnix model file.
#[derive(Debug, Clone)]
pub struct ModelFile {
    /// Descriptive information about the model.
    pub metadata: MetadataFile,
    /// Text and images drawn in front of the model's contents.
    pub foreground_decorations: Vec<ForegroundDecoration>,
    /// Shapes drawn behind the model's contents.
    pub background_decorations: Vec<BackgroundDecoration>,
    /// Global settings for the model, including simulation parameters and other configuration.
    pub global_settings: GlobalObjectFrozen,
    /// Ordered component instances with saved GUI and parameter snapshots.
    pub components: Vec<ComponentInstance>,
    /// Connections between component ports within the model.
    pub connections: Vec<Connection>,
    /// Source language and translated text used by the model.
    pub translations: TranslationsFile,
    /// Ordered parameter definitions, including defaults and descriptions.
    pub parameter_definition: ParameterObjectDefinition,
    /// Ordered variable definitions, including defaults and descriptions.
    pub variable_definition: VariableObjectDefinition,
    /// BLAKE3 hash of all validated model data.
    pub file_hash: [u8; 32],
}

impl PartialEq for ModelFile {
    fn eq(&self, other: &Self) -> bool {
        self.metadata == other.metadata
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
