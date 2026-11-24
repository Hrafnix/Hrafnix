use crate::component::data::ComponentFile;
use crate::component_instance::data::ComponentInstance;
use crate::connection::data::Connection;
use crate::decoration_background::data::{BackgroundDecoration, BackgroundDecorationKind};
use crate::decoration_foreground::data::{
    ForegroundDecoration, ForegroundDecorationKind, ImageFit,
};
use crate::graphics::data::{Dimension, Position};
use crate::metadata_general::data::MetadataFile;
use crate::metadata_gui::data::GuiMetadata;
use crate::model::data::ModelFile;
use crate::source::data::ComponentSource;
use crate::translations::data::TranslationsFile;
use hrafnix_common::port::PortType;
use hrafnix_datastore::definition::ParameterObjectDefinition;

/// Hashes all validated component data, including its unresolved provenance.
pub(crate) fn component_file_hash(component: &ComponentFile) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"ComponentFile");
    hasher.update(&component.interface_hash());
    hasher.update(&component.variable_definition().hash());
    update_foreground_decorations_hash(&mut hasher, component.foreground_decorations());
    update_background_decorations_hash(&mut hasher, component.background_decorations());
    update_components_hash(&mut hasher, component.components());
    update_connections_hash(&mut hasher, component.connections());
    update_translations_hash(&mut hasher, component.translations());
    *hasher.finalize().as_bytes()
}

/// Hashes all validated model data.
pub(crate) fn model_file_hash(model: &ModelFile) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"ModelFile");
    update_metadata_hash(&mut hasher, model.metadata());
    hasher.update(&model.parameter_definition().hash());
    hasher.update(&model.variable_definition().hash());
    hasher.update(&model.global_settings().definition().hash());
    hasher.update(&model.global_settings().hash());
    update_foreground_decorations_hash(&mut hasher, model.foreground_decorations());
    update_background_decorations_hash(&mut hasher, model.background_decorations());
    update_components_hash(&mut hasher, model.components());
    update_connections_hash(&mut hasher, model.connections());
    update_translations_hash(&mut hasher, model.translations());
    *hasher.finalize().as_bytes()
}

/// Hashes a component's externally visible metadata, source, GUI, and parameters.
pub(crate) fn component_interface_hash(
    metadata: &MetadataFile,
    source: &ComponentSource,
    gui_metadata: &GuiMetadata,
    parameter_definition: &ParameterObjectDefinition,
) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"ComponentInterface");
    update_metadata_hash(&mut hasher, metadata);
    update_source_hash(&mut hasher, source);
    update_gui_metadata_hash(&mut hasher, gui_metadata);
    hasher.update(&parameter_definition.hash());
    *hasher.finalize().as_bytes()
}

/// Hashes an instance's saved source, GUI, and embedded parameter interface.
pub(crate) fn instance_interface_hash(
    source: &ComponentSource,
    gui_metadata: &GuiMetadata,
    parameter_definition: &ParameterObjectDefinition,
) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"ComponentInstanceInterface");
    update_source_hash(&mut hasher, source);
    update_gui_metadata_hash(&mut hasher, gui_metadata);
    hasher.update(&parameter_definition.hash());
    *hasher.finalize().as_bytes()
}

/// Adds file metadata to a hash.
fn update_metadata_hash(hasher: &mut blake3::Hasher, metadata: &MetadataFile) {
    hasher.update(b"MetadataFile");
    update_string_hash(hasher, metadata.display_name().as_str());
    update_string_hash(hasher, metadata.description().as_str());
    update_string_hash(hasher, metadata.author().as_str());
    hasher.update(&metadata.minimum_version_major().to_le_bytes());
    hasher.update(&metadata.minimum_version_minor().to_le_bytes());
}

/// Adds unresolved component provenance to a hash.
fn update_source_hash(hasher: &mut blake3::Hasher, source: &ComponentSource) {
    match source {
        ComponentSource::File { package, path } => {
            hasher.update(&[0]);
            hasher.update(b"File");
            match package {
                Some(package) => {
                    hasher.update(&[1]);
                    update_string_hash(hasher, package.name().as_str());
                    update_string_hash(hasher, package.version().as_str());
                }
                None => {
                    hasher.update(&[0]);
                }
            }
            update_string_hash(hasher, path.as_str());
        }
        ComponentSource::Builtin { name, version } => {
            hasher.update(&[1]);
            hasher.update(b"Builtin");
            update_string_hash(hasher, name.as_str());
            update_string_hash(hasher, version.as_str());
        }
    }
}

/// Adds drawing metadata and port definitions to a hash.
fn update_gui_metadata_hash(hasher: &mut blake3::Hasher, metadata: &GuiMetadata) {
    hasher.update(b"GuiMetadata");
    hasher.update(&metadata.dimensions().width().to_le_bytes());
    hasher.update(&metadata.dimensions().height().to_le_bytes());
    update_string_hash(hasher, metadata.icon().as_str());
    update_length_hash(hasher, metadata.ports().len());
    for port in metadata.ports() {
        update_string_hash(hasher, port.id().as_str());
        hasher.update(&[port_type_discriminant(port.port_type())]);
        hasher.update(&port.location().x().to_le_bytes());
        hasher.update(&port.location().y().to_le_bytes());
    }
}

/// Adds ordered foreground decorations and their styles to a hash.
fn update_foreground_decorations_hash(
    hasher: &mut blake3::Hasher,
    decorations: &[ForegroundDecoration],
) {
    update_length_hash(hasher, decorations.len());
    for decoration in decorations {
        update_string_hash(hasher, decoration.id().as_str());
        update_position_hash(hasher, decoration.position());
        update_dimension_hash(hasher, decoration.dimensions());
        match decoration.kind() {
            ForegroundDecorationKind::Text(text) => {
                hasher.update(&[0]);
                update_string_hash(hasher, text.text().as_str());
                hasher.update(&text.font_size().size().to_le_bytes());
            }
            ForegroundDecorationKind::Image(image) => {
                hasher.update(&[1]);
                update_string_hash(hasher, image.path().as_str());
                hasher.update(&[match image.fit() {
                    ImageFit::Contain => 0,
                    ImageFit::Cover => 1,
                    ImageFit::Fill => 2,
                }]);
            }
        }
    }
}

/// Adds ordered background decorations to a hash.
fn update_background_decorations_hash(
    hasher: &mut blake3::Hasher,
    decorations: &[BackgroundDecoration],
) {
    update_length_hash(hasher, decorations.len());
    for decoration in decorations {
        update_string_hash(hasher, decoration.id().as_str());
        update_position_hash(hasher, decoration.position());
        update_dimension_hash(hasher, decoration.dimensions());
        hasher.update(&[match decoration.kind() {
            BackgroundDecorationKind::Frame => 0,
        }]);
    }
}

/// Adds ordered instances, including their saved interfaces and expressions.
fn update_components_hash(hasher: &mut blake3::Hasher, components: &[ComponentInstance]) {
    update_length_hash(hasher, components.len());
    for component in components {
        update_string_hash(hasher, component.id().as_str());
        update_source_hash(hasher, component.source());
        update_position_hash(hasher, component.position());
        update_gui_metadata_hash(hasher, component.gui_metadata());
        hasher.update(&component.parameters().definition().hash());
        hasher.update(&component.parameters().hash());
    }
}

/// Adds ordered connection endpoints and drawing paths to a hash.
fn update_connections_hash(hasher: &mut blake3::Hasher, connections: &[Connection]) {
    update_length_hash(hasher, connections.len());
    for connection in connections {
        update_string_hash(hasher, connection.from_component().as_str());
        update_string_hash(hasher, connection.from_port().as_str());
        update_string_hash(hasher, connection.to_component().as_str());
        update_string_hash(hasher, connection.to_port().as_str());
        update_length_hash(hasher, connection.line().midpoints().len());
        for midpoint in connection.line().midpoints() {
            update_position_hash(hasher, *midpoint);
        }
    }
}

/// Adds translations in deterministic key and language order to a hash.
fn update_translations_hash(hasher: &mut blake3::Hasher, translations: &TranslationsFile) {
    update_string_hash(hasher, translations.language().as_str());
    update_length_hash(hasher, translations.translations().len());
    for (original, languages) in translations.translations() {
        update_string_hash(hasher, original.as_str());
        update_length_hash(hasher, languages.len());
        for (language, translated) in languages {
            update_string_hash(hasher, language.as_str());
            update_string_hash(hasher, translated.as_str());
        }
    }
}

/// Adds signed drawing coordinates to a hash.
fn update_position_hash(hasher: &mut blake3::Hasher, position: Position) {
    hasher.update(&position.x().to_le_bytes());
    hasher.update(&position.y().to_le_bytes());
}

/// Adds drawing dimensions to a hash.
fn update_dimension_hash(hasher: &mut blake3::Hasher, dimensions: Dimension) {
    hasher.update(&dimensions.width().to_le_bytes());
    hasher.update(&dimensions.height().to_le_bytes());
}

/// Adds a length-prefixed UTF-8 string to a hash.
fn update_string_hash(hasher: &mut blake3::Hasher, value: &str) {
    update_length_hash(hasher, value.len());
    hasher.update(value.as_bytes());
}

/// Adds a platform-independent collection or string length to a hash.
fn update_length_hash(hasher: &mut blake3::Hasher, length: usize) {
    hasher.update(&u64::try_from(length).unwrap_or(u64::MAX).to_le_bytes());
}

/// Returns the stable interface encoding for a port type.
const fn port_type_discriminant(port_type: PortType) -> u8 {
    match port_type {
        PortType::Input => 0,
        PortType::Output => 1,
        PortType::Power => 2,
        PortType::BidirectionalSignal => 3,
        PortType::System => 4,
        PortType::PowerMulti => 5,
        PortType::InputMulti => 6,
    }
}
