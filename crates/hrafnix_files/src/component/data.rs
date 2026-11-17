use crate::connection::data::Connection;
use crate::decoration_background::data::BackgroundDecoration;
use crate::decoration_foreground::data::ForegroundDecoration;
use crate::metadata_general::data::MetadataFile;
use crate::metadata_gui::data::GuiMetadata;
use crate::translations::data::TranslationsFile;
use hrafnix_datastore::definition::{ParameterObjectDefinition, VariableObjectDefinition};

/// Represents User component data from a Hrafnix component file.
#[derive(Debug, Clone, PartialEq)]
pub struct ComponentFile {
    /// Descriptive information about the component.
    pub metadata: MetadataFile,
    /// Drawing dimensions and icon associated with the component.
    pub gui_metadata: GuiMetadata,
    /// Text and images drawn in front of the component's contents.
    pub foreground_decorations: Vec<ForegroundDecoration>,
    /// Shapes drawn behind the component's contents.
    pub background_decorations: Vec<BackgroundDecoration>,
    /// Connections between component ports within the component.
    pub connections: Vec<Connection>,
    /// Source language and translated text used by the component.
    pub translations: TranslationsFile,
    /// Ordered parameter definitions, including defaults and descriptions.
    pub parameter_definition: ParameterObjectDefinition,
    /// Ordered variable definitions, including defaults and descriptions.
    pub variable_definition: VariableObjectDefinition,
}
