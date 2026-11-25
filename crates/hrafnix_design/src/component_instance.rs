use crate::component_source::ResolvedComponent;
use crate::types::{ComponentSource, GuiMetadata, Position};
use hrafnix_datastore::editable::ParameterObjectEditable;
use hrafnix_datastore::frozen::ParameterObjectFrozen;
use hrafnix_files::builder::ComponentInstanceBuilder;
use hrafnix_files::data::component_instance::ComponentInstance as FileComponentInstance;
use hrafnix_keys::{ComponentKey, ParameterKey};
use hrafnix_message::message::{Message, MessageCategory};
use hrafnix_shareable_string::SharedStringStore;
use parking_lot::RwLock;
use std::sync::Arc;

/// Internal state of a placed component instance.
#[derive(Debug, Clone)]
pub(crate) struct ComponentInstanceData {
    /// Identifier unique within the containing model or user component.
    id: ComponentKey,
    /// Unresolved reference to the instance's component definition.
    source: ComponentSource,
    /// Position in the containing drawing.
    position: Position,
    /// Editable dimensions, rotation, icon, and icon-local port locations.
    gui_metadata: GuiMetadata,
    /// Frozen parameter definitions and unevaluated expressions.
    parameters: ParameterObjectFrozen,
    /// BLAKE3 interface hash captured at creation, loading, or the last refresh.
    saved_interface_hash: [u8; 32],
}

impl ComponentInstanceData {
    /// Builds a file snapshot using the instance hash domain, not the source file's hash.
    fn to_file(&self) -> FileComponentInstance {
        ComponentInstanceBuilder::new(
            self.id.clone(),
            self.source.to_file(),
            self.position.to_file(),
            self.gui_metadata.to_file(),
            self.parameters.clone(),
        )
        .build()
    }

    /// Rejects a definition bound to different requested provenance.
    fn ensure_source(&self, component: &ResolvedComponent) -> Result<(), Message> {
        if self.source != *component.source() {
            return Err(Message::error_with_param(
                MessageCategory::Component,
                "design_component_source_mismatch",
                "component",
                self.id.as_str(),
            ));
        }
        Ok(())
    }
}

/// Changes made while refreshing an instance's source-defined interface.
///
/// Parameter lists retain the corresponding old or new definition order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InterfaceRefresh {
    /// Whether the current GUI or parameter definitions changed.
    changed: bool,
    /// Parameters added with their source defaults.
    added_parameters: Vec<ParameterKey>,
    /// Parameters removed because their keys no longer exist in the source.
    removed_parameters: Vec<ParameterKey>,
    /// Existing parameters reset to defaults because definitions are incompatible.
    reset_parameters: Vec<ParameterKey>,
}

impl InterfaceRefresh {
    /// Returns whether the instance's source-defined interface changed.
    #[must_use]
    pub const fn changed(&self) -> bool {
        self.changed
    }

    /// Returns new parameter keys, in the new definition order.
    #[must_use]
    pub fn added_parameters(&self) -> &[ParameterKey] {
        &self.added_parameters
    }

    /// Returns removed parameter keys, in the old definition order.
    #[must_use]
    pub fn removed_parameters(&self) -> &[ParameterKey] {
        &self.removed_parameters
    }

    /// Returns incompatible parameter keys reset to defaults, in the new definition order.
    #[must_use]
    pub fn reset_parameters(&self) -> &[ParameterKey] {
        &self.reset_parameters
    }
}

/// Shared, interior-mutable handle to a component instance placed in a model or user component.
#[derive(Debug, Clone)]
pub struct ComponentInstance {
    /// Shared interned string store for laundering updated strings.
    shared_string_store: SharedStringStore,
    /// Shared editable instance state.
    data: Arc<RwLock<ComponentInstanceData>>,
}

impl ComponentInstance {
    /// Creates a new component instance, laundering its strings into `shared_string_store`.
    #[must_use]
    pub fn new(
        shared_string_store: SharedStringStore,
        mut id: ComponentKey,
        mut source: ComponentSource,
        position: Position,
        mut gui_metadata: GuiMetadata,
        mut parameters: ParameterObjectFrozen,
    ) -> Self {
        id.launder(&shared_string_store);
        source.launder(&shared_string_store);
        gui_metadata.launder(&shared_string_store);
        parameters.launder(&shared_string_store);

        let initial_file = ComponentInstanceBuilder::new(
            id.clone(),
            source.to_file(),
            position.to_file(),
            gui_metadata.to_file(),
            parameters.clone(),
        )
        .build();
        let saved_interface_hash = initial_file.interface_hash();

        let data = ComponentInstanceData {
            id,
            source,
            position,
            gui_metadata,
            parameters,
            saved_interface_hash,
        };
        Self {
            shared_string_store,
            data: Arc::new(RwLock::new(data)),
        }
    }

    /// Creates a component instance from a loaded file snapshot.
    #[must_use]
    pub(crate) fn from_file_instance(
        shared_string_store: SharedStringStore,
        instance: &FileComponentInstance,
    ) -> Self {
        let data = ComponentInstanceData {
            id: instance.id().clone(),
            source: ComponentSource::from_file(instance.source()),
            position: Position::from_file(instance.position()),
            gui_metadata: GuiMetadata::from_file(instance.gui_metadata()),
            parameters: instance.parameters().clone(),
            saved_interface_hash: instance.interface_hash(),
        };
        Self {
            shared_string_store,
            data: Arc::new(RwLock::new(data)),
        }
    }

    /// Converts this component instance back into a file snapshot.
    #[must_use]
    pub(crate) fn to_file_instance(&self) -> FileComponentInstance {
        self.data.read().to_file()
    }

    /// Returns the component instance identifier.
    #[must_use]
    pub fn id(&self) -> ComponentKey {
        self.data.read().id.clone()
    }

    /// Updates the component instance identifier, laundering its string into the project store.
    pub fn set_id(&self, mut id: ComponentKey) {
        id.launder(&self.shared_string_store);
        self.data.write().id = id;
    }

    /// Returns the unresolved provenance of the component definition.
    #[must_use]
    pub fn source(&self) -> ComponentSource {
        self.data.read().source.clone()
    }

    /// Returns the instance position in the containing drawing.
    #[must_use]
    pub fn position(&self) -> Position {
        self.data.read().position
    }

    /// Updates the instance position in the containing drawing.
    pub fn set_position(&self, position: Position) {
        self.data.write().position = position;
    }

    /// Returns the editable GUI metadata for this instance.
    #[must_use]
    pub fn gui_metadata(&self) -> GuiMetadata {
        self.data.read().gui_metadata.clone()
    }

    /// Updates the GUI metadata for this instance, laundering its strings into the project store.
    pub fn set_gui_metadata(&self, mut gui_metadata: GuiMetadata) {
        gui_metadata.launder(&self.shared_string_store);
        self.data.write().gui_metadata = gui_metadata;
    }

    /// Returns a cloned frozen snapshot of this instance's parameters.
    #[must_use]
    pub fn parameters(&self) -> ParameterObjectFrozen {
        self.data.read().parameters.clone()
    }

    /// Thaws this instance's frozen parameters into an editable copy.
    #[must_use]
    pub fn thaw_parameters(&self) -> ParameterObjectEditable {
        self.data.read().parameters.thaw()
    }

    /// Freezes `edited`, launders its strings, and merges its values back into frozen parameters.
    pub fn merge_parameters(&self, edited: &ParameterObjectEditable) {
        let mut frozen = edited.freeze();
        frozen.launder(&self.shared_string_store);
        self.data.write().parameters.merge_from(&frozen);
    }

    /// Thaws parameters, applies `f` without holding the write lock, and merges changes back in.
    pub fn edit_parameters<R>(&self, f: impl FnOnce(&mut ParameterObjectEditable) -> R) -> R {
        let mut editable = self.thaw_parameters();
        let result = f(&mut editable);
        self.merge_parameters(&editable);
        result
    }

    /// Checks whether this instance's GUI or parameter definitions differ from `component`.
    ///
    /// Preserves placed rotation when comparing GUI metadata. Parameter values,
    /// source metadata, and variable definitions do not define an instance interface.
    /// This method performs no I/O; reload the source to check current disk contents.
    /// Relative sources must be resolved using this instance's containing file.
    ///
    /// # Errors
    ///
    /// Returns a localized error if `component` was requested using different provenance.
    pub fn is_interface_stale(&self, component: &ResolvedComponent) -> Result<bool, Message> {
        let guard = self.data.read();
        guard.ensure_source(component)?;
        let mut gui_metadata = component.gui_metadata();
        gui_metadata.set_rotation(guard.gui_metadata.rotation());
        Ok(guard.gui_metadata.to_file() != gui_metadata.to_file()
            || guard.parameters.definition() != component.parameter_definition())
    }

    /// Refreshes GUI and parameter definitions from an explicitly resolved source.
    ///
    /// Keeps ID, provenance, position, and placed rotation. Source-compatible
    /// parameter values are merged into the new definitions using datastore merge
    /// compatibility; new or incompatible keys use source defaults, and removed
    /// keys are discarded. The result reports all additions, removals, and resets.
    /// Strings are interned in this instance's store and the interface hash is
    /// updated using the saved-instance hash domain. No file I/O is performed.
    /// Relative sources must be resolved using this instance's containing file.
    ///
    /// # Errors
    ///
    /// Returns a localized error without changing the instance if `component` was
    /// requested using different provenance.
    pub fn refresh_interface(
        &self,
        component: &ResolvedComponent,
    ) -> Result<InterfaceRefresh, Message> {
        let mut guard = self.data.write();
        guard.ensure_source(component)?;
        let mut gui_metadata = component.gui_metadata();
        gui_metadata.set_rotation(guard.gui_metadata.rotation());
        gui_metadata.launder(&self.shared_string_store);
        let mut parameters = ParameterObjectFrozen::new(component.parameter_definition().clone());
        parameters.launder(&self.shared_string_store);
        let mut report = InterfaceRefresh {
            changed: guard.gui_metadata.to_file() != gui_metadata.to_file()
                || guard.parameters.definition() != parameters.definition(),
            added_parameters: Vec::new(),
            removed_parameters: Vec::new(),
            reset_parameters: Vec::new(),
        };
        for (key, definition) in parameters.definition().iter() {
            match guard.parameters.definition().get_str(key.as_str()) {
                None => report.added_parameters.push(key.clone()),
                Some(previous) if !definition.is_merge_compatible(previous) => {
                    report.reset_parameters.push(key.clone());
                }
                Some(_) => {}
            }
        }
        for key in guard.parameters.definition().keys() {
            if !parameters.definition().contains_str(key.as_str()) {
                report.removed_parameters.push(key.clone());
            }
        }
        parameters.merge_from(&guard.parameters);
        guard.gui_metadata = gui_metadata;
        guard.parameters = parameters;
        guard.saved_interface_hash = guard.to_file().interface_hash();
        Ok(report)
    }

    /// Returns the BLAKE3 instance interface hash captured at creation, load, or refresh.
    #[must_use]
    pub fn saved_interface_hash(&self) -> [u8; 32] {
        self.data.read().saved_interface_hash
    }
}
