use crate::component_instance::ComponentInstance;
use crate::types::{
    ComponentSource, Dimension, GuiMetadata, Metadata, Port, PortLocation, Position, Rotation,
};
use hrafnix_built_in_registry::BuiltInComponentRegistration;
use hrafnix_datastore::definition::{ParameterObjectDefinition, VariableObjectDefinition};
use hrafnix_datastore::frozen::ParameterObjectFrozen;
use hrafnix_keys::ComponentKey;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};

/// Immutable design descriptor for a Hrafnix-provided built-in component.
///
/// Built-in components are supplied by the internal registry rather than loaded
/// from `.hxc` files, and can be instantiated into a [`Model`](crate::model::Model)
/// or [`UserComponent`](crate::user_component::UserComponent) as a [`ComponentInstance`].
#[derive(Debug, Clone)]
pub struct BuiltInComponent {
    /// Unique built-in component name (e.g., `"builtin.clock"`).
    name: ShareableString,
    /// Built-in component version.
    version: u32,
    /// Descriptive metadata for display and documentation.
    metadata: Metadata,
    /// Default icon, dimensions, rotation, and port layout.
    gui_metadata: GuiMetadata,
    /// Parameter definitions used to initialize new instances.
    parameter_definition: ParameterObjectDefinition,
    /// Variable definitions exposed by this component.
    variable_definition: VariableObjectDefinition,
    /// Static definition and runtime factory, absent for project-local descriptors.
    registration: Option<&'static BuiltInComponentRegistration>,
}

impl BuiltInComponent {
    /// Creates a new built-in component descriptor.
    #[must_use]
    pub fn new(
        name: impl Into<ShareableString>,
        version: u32,
        metadata: Metadata,
        gui_metadata: GuiMetadata,
        parameter_definition: ParameterObjectDefinition,
        variable_definition: VariableObjectDefinition,
    ) -> Self {
        Self {
            name: name.into(),
            version,
            metadata,
            gui_metadata,
            parameter_definition,
            variable_definition,
            registration: None,
        }
    }

    /// Creates a design descriptor from a bundled registry registration.
    ///
    /// Embedded SVGs stay in the registry; GUI snapshots use an empty icon path.
    /// Registry-only port details and runtime factories remain available through
    /// [`Self::registration`], without changing the saved file schema.
    #[must_use]
    pub fn from_registration(registration: &'static BuiltInComponentRegistration) -> Self {
        let definition = registration.definition();
        let icon = definition.icon();
        let ports = definition
            .ports()
            .iter()
            .map(|port| {
                Port::new(
                    port.id().into(),
                    port.kind().port_type(),
                    PortLocation::new(u16::from(port.canvas_x()), u16::from(port.canvas_y())),
                )
            })
            .collect();
        Self {
            name: definition.id().as_str().into(),
            version: u32::from(definition.version()),
            metadata: Metadata::new(definition.display_name(), "", "", 0, 0),
            gui_metadata: GuiMetadata::new(
                Dimension::new(u16::from(icon.width()), u16::from(icon.height())),
                Rotation::Degrees0,
                "".into(),
                ports,
            ),
            parameter_definition: definition.parameters().into_definition(),
            variable_definition: definition.variables().into_definition(),
            registration: Some(registration),
        }
    }

    /// Returns the static definition and isolated runtime factory, when registry-backed.
    ///
    /// Project-local overrides created with [`Self::new`] return `None` rather than
    /// inheriting behavior or embedded icons from the replaced registry definition.
    #[must_use]
    pub const fn registration(&self) -> Option<&'static BuiltInComponentRegistration> {
        self.registration
    }

    /// Returns the built-in component name.
    #[must_use]
    pub const fn name(&self) -> &ShareableString {
        &self.name
    }

    /// Returns the built-in component version.
    #[must_use]
    pub const fn version(&self) -> u32 {
        self.version
    }

    /// Returns the built-in component's provenance source.
    #[must_use]
    pub fn source(&self) -> ComponentSource {
        ComponentSource::Builtin {
            name: self.name.clone(),
            version: self.version,
        }
    }

    /// Returns the descriptive metadata.
    #[must_use]
    pub const fn metadata(&self) -> &Metadata {
        &self.metadata
    }

    /// Returns the default GUI metadata.
    #[must_use]
    pub const fn gui_metadata(&self) -> &GuiMetadata {
        &self.gui_metadata
    }

    /// Returns the parameter definitions.
    #[must_use]
    pub const fn parameter_definition(&self) -> &ParameterObjectDefinition {
        &self.parameter_definition
    }

    /// Returns the variable definitions.
    #[must_use]
    pub const fn variable_definition(&self) -> &VariableObjectDefinition {
        &self.variable_definition
    }

    /// Interns this descriptor's strings in `store` in place.
    pub fn launder(&mut self, store: &SharedStringStore) {
        self.name = store.launder(&self.name);
        self.metadata.launder(store);
        self.gui_metadata.launder(store);
        self.parameter_definition.launder(store);
        self.variable_definition.launder(store);
    }

    /// Instantiates this built-in component at `position` with default parameter values.
    #[must_use]
    pub fn instantiate(
        &self,
        shared_string_store: &SharedStringStore,
        id: ComponentKey,
        position: Position,
    ) -> ComponentInstance {
        let parameters = ParameterObjectFrozen::new(self.parameter_definition.clone());
        ComponentInstance::new(
            shared_string_store.clone(),
            id,
            self.source(),
            position,
            self.gui_metadata.clone(),
            parameters,
        )
    }
}
