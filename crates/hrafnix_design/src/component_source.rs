use crate::built_in_component::BuiltInComponent;
use crate::component_instance::ComponentInstance;
use crate::types::{ComponentSource, GuiMetadata, Metadata, Position};
use hrafnix_datastore::definition::{ParameterObjectDefinition, VariableObjectDefinition};
use hrafnix_datastore::frozen::ParameterObjectFrozen;
use hrafnix_files::data::component::ComponentFile;
use hrafnix_keys::ComponentKey;
use hrafnix_message::message::{Message, MessageCategory};
use hrafnix_shareable_string::SharedStringStore;
use std::sync::Arc;

/// An immutable source definition bound to the original requested provenance.
///
/// Resolving a relative path does not rewrite the stored source. Resolution is
/// shallow: nested components are not loaded automatically.
#[derive(Debug, Clone)]
pub struct ResolvedComponent {
    /// Original provenance used to request this definition.
    source: ComponentSource,
    /// Cached file snapshot or registered built-in descriptor.
    definition: ResolvedDefinition,
}

/// Immutable definitions available to the design layer.
#[derive(Debug, Clone)]
enum ResolvedDefinition {
    /// A loaded user-authored component file.
    File(Arc<ComponentFile>),
    /// A registered descriptor for an exact built-in version.
    Builtin(Arc<BuiltInComponent>),
}

impl ResolvedComponent {
    /// Binds a file snapshot to the requested source.
    pub(crate) const fn from_file(source: ComponentSource, file: Arc<ComponentFile>) -> Self {
        Self {
            source,
            definition: ResolvedDefinition::File(file),
        }
    }

    /// Binds a built-in descriptor to the requested source.
    pub(crate) const fn from_builtin(
        source: ComponentSource,
        component: Arc<BuiltInComponent>,
    ) -> Self {
        Self {
            source,
            definition: ResolvedDefinition::Builtin(component),
        }
    }

    /// Returns the original source, without rewriting relative paths.
    #[must_use]
    pub const fn source(&self) -> &ComponentSource {
        &self.source
    }

    /// Returns the loaded file snapshot, if this is a file source.
    #[must_use]
    pub fn file(&self) -> Option<&ComponentFile> {
        match &self.definition {
            ResolvedDefinition::File(file) => Some(file),
            ResolvedDefinition::Builtin(_) => None,
        }
    }

    /// Returns the registered descriptor, if this is a built-in source.
    #[must_use]
    pub fn built_in(&self) -> Option<&BuiltInComponent> {
        match &self.definition {
            ResolvedDefinition::Builtin(component) => Some(component),
            ResolvedDefinition::File(_) => None,
        }
    }

    /// Returns the source's descriptive metadata.
    #[must_use]
    pub fn metadata(&self) -> Metadata {
        match &self.definition {
            ResolvedDefinition::File(file) => Metadata::from_file(file.metadata()),
            ResolvedDefinition::Builtin(component) => component.metadata().clone(),
        }
    }

    /// Returns the source's GUI metadata, with port coordinates capped to its dimensions.
    #[must_use]
    pub fn gui_metadata(&self) -> GuiMetadata {
        match &self.definition {
            ResolvedDefinition::File(file) => GuiMetadata::from_file(file.gui_metadata()),
            ResolvedDefinition::Builtin(component) => {
                GuiMetadata::from_file(&component.gui_metadata().to_file())
            }
        }
    }

    /// Returns the source's ordered parameter definitions.
    #[must_use]
    pub fn parameter_definition(&self) -> &ParameterObjectDefinition {
        match &self.definition {
            ResolvedDefinition::File(file) => file.parameter_definition(),
            ResolvedDefinition::Builtin(component) => component.parameter_definition(),
        }
    }

    /// Returns the source's ordered variable definitions.
    #[must_use]
    pub fn variable_definition(&self) -> &VariableObjectDefinition {
        match &self.definition {
            ResolvedDefinition::File(file) => file.variable_definition(),
            ResolvedDefinition::Builtin(component) => component.variable_definition(),
        }
    }

    /// Instantiates this definition with default parameters and the original source.
    ///
    /// Relative sources remain relative to the containing file used for resolution.
    #[must_use]
    pub fn instantiate(
        &self,
        store: &SharedStringStore,
        id: ComponentKey,
        position: Position,
    ) -> ComponentInstance {
        ComponentInstance::new(
            store.clone(),
            id,
            self.source.clone(),
            position,
            self.gui_metadata(),
            ParameterObjectFrozen::new(self.parameter_definition().clone()),
        )
    }
}

/// Reports a missing instance without silently skipping resolution or refresh.
pub(crate) fn component_not_found(id: &ComponentKey) -> Message {
    Message::error_with_param(
        MessageCategory::Component,
        "design_component_not_found",
        "component",
        id.as_str(),
    )
}
