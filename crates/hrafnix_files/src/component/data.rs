use crate::component_instance::data::ComponentInstance;
use crate::connection::data::Connection;
use crate::decoration_background::data::BackgroundDecoration;
use crate::decoration_foreground::data::ForegroundDecoration;
use crate::metadata_general::data::MetadataFile;
use crate::metadata_gui::data::GuiMetadata;
use crate::source::data::ComponentSource;
use crate::translations::data::TranslationsFile;
use hrafnix_datastore::definition::{ParameterObjectDefinition, VariableObjectDefinition};

/// Represents User component data from a Hrafnix component file.
#[derive(Debug, Clone)]
pub struct ComponentFile {
    /// Descriptive information about the component.
    pub metadata: MetadataFile,
    /// Unresolved provenance of the loaded component file.
    pub source: ComponentSource,
    /// Drawing dimensions and icon associated with the component.
    pub gui_metadata: GuiMetadata,
    /// Text and images drawn in front of the component's contents.
    pub foreground_decorations: Vec<ForegroundDecoration>,
    /// Shapes drawn behind the component's contents.
    pub background_decorations: Vec<BackgroundDecoration>,
    /// Ordered nested component instances with saved GUI and parameter snapshots.
    pub components: Vec<ComponentInstance>,
    /// Connections between component ports within the component.
    pub connections: Vec<Connection>,
    /// Source language and translated text used by the component.
    pub translations: TranslationsFile,
    /// Ordered parameter definitions, including defaults and descriptions.
    pub parameter_definition: ParameterObjectDefinition,
    /// Ordered variable definitions, including defaults and descriptions.
    pub variable_definition: VariableObjectDefinition,
    /// BLAKE3 hash of all validated component data, including source provenance.
    pub file_hash: [u8; 32],
    /// BLAKE3 hash of the component's metadata, source, GUI, and parameter interface.
    pub interface_hash: [u8; 32],
}

impl PartialEq for ComponentFile {
    fn eq(&self, other: &Self) -> bool {
        self.interface_hash == other.interface_hash
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
