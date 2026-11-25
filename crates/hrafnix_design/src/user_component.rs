use crate::component_instance::{ComponentInstance, InterfaceRefresh};
use crate::component_source::{ResolvedComponent, component_not_found};
use crate::project::ProjectData;
use crate::types::{
    BackgroundDecoration, ComponentSource, Connection, ForegroundDecoration, GuiMetadata, Metadata,
    PackageReference, Position, Translations,
};
use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_datastore::definition::{ParameterObjectDefinition, VariableObjectDefinition};
use hrafnix_datastore::frozen::ParameterObjectFrozen;
use hrafnix_files::builder::ComponentFileBuilder;
use hrafnix_files::data::component::ComponentFile;
use hrafnix_files::formats::component::write_component_file;
use hrafnix_keys::ComponentKey;
use hrafnix_message::message::Message;
use hrafnix_shareable_string::SharedStringStore;
use parking_lot::RwLock;
use std::sync::Arc;

/// Internal editable state of an open user-authored component.
#[derive(Debug, Clone)]
struct UserComponentData {
    /// Resolved path to the backing component file.
    file_path: Utf8PathBuf,
    /// BLAKE3 hash of the last loaded or saved component file snapshot.
    source_hash: [u8; 32],
    /// BLAKE3 interface hash of the last loaded or saved component file snapshot.
    interface_hash: [u8; 32],
    /// Editable descriptive information about the component.
    metadata: Metadata,
    /// Unresolved provenance of the loaded component file.
    source: ComponentSource,
    /// Editable drawing dimensions, rotation, icon, and port locations.
    gui_metadata: GuiMetadata,
    /// Text and images drawn in front of the component's contents.
    foreground_decorations: Vec<ForegroundDecoration>,
    /// Shapes drawn behind the component's contents.
    background_decorations: Vec<BackgroundDecoration>,
    /// Ordered nested editable component instances.
    components: Vec<ComponentInstance>,
    /// Connections between component ports within the component.
    connections: Vec<Connection>,
    /// Source language and translated text used by the component.
    translations: Translations,
    /// Ordered parameter definitions, including defaults and descriptions.
    parameter_definition: ParameterObjectDefinition,
    /// Ordered variable definitions, including defaults and descriptions.
    variable_definition: VariableObjectDefinition,
}

impl UserComponentData {
    /// Converts a loaded [`ComponentFile`] snapshot into editable user component state.
    fn from_file(
        shared_string_store: &SharedStringStore,
        file_path: Utf8PathBuf,
        file: &ComponentFile,
    ) -> Self {
        Self {
            file_path,
            source_hash: file.file_hash(),
            interface_hash: file.interface_hash(),
            metadata: Metadata::from_file(file.metadata()),
            source: ComponentSource::from_file(file.source()),
            gui_metadata: GuiMetadata::from_file(file.gui_metadata()),
            foreground_decorations: file
                .foreground_decorations()
                .iter()
                .map(ForegroundDecoration::from_file)
                .collect(),
            background_decorations: file
                .background_decorations()
                .iter()
                .map(BackgroundDecoration::from_file)
                .collect(),
            components: file
                .components()
                .iter()
                .map(|inst| {
                    ComponentInstance::from_file_instance(shared_string_store.clone(), inst)
                })
                .collect(),
            connections: file
                .connections()
                .iter()
                .map(Connection::from_file)
                .collect(),
            translations: Translations::from_file(file.translations()),
            parameter_definition: file.parameter_definition().clone(),
            variable_definition: file.variable_definition().clone(),
        }
    }

    /// Converts editable user component state back into a frozen [`ComponentFile`] snapshot.
    fn to_file(&self) -> ComponentFile {
        ComponentFileBuilder::new(
            self.metadata.to_file(),
            self.source.to_file(),
            self.gui_metadata.to_file(),
            self.translations.to_file(),
        )
        .foreground_decorations(
            self.foreground_decorations
                .iter()
                .map(ForegroundDecoration::to_file)
                .collect(),
        )
        .background_decorations(
            self.background_decorations
                .iter()
                .map(BackgroundDecoration::to_file)
                .collect(),
        )
        .components(
            self.components
                .iter()
                .map(ComponentInstance::to_file_instance)
                .collect(),
        )
        .connections(self.connections.iter().map(Connection::to_file).collect())
        .parameter_definition(self.parameter_definition.clone())
        .variable_definition(self.variable_definition.clone())
        .build()
    }
}

/// Shared, interior-mutable handle to an editable user-authored component.
#[derive(Debug, Clone)]
pub struct UserComponent {
    /// Shared project context.
    project_data: ProjectData,
    /// Shared editable component state.
    data: Arc<RwLock<UserComponentData>>,
}

impl UserComponent {
    /// Creates a new in-memory editable user component backed by `path` (not yet written to disk).
    ///
    /// Relative paths are resolved inside the project's `components` directory.
    #[must_use]
    pub(crate) fn create(
        project_data: ProjectData,
        path: &Utf8Path,
        mut metadata: Metadata,
        mut gui_metadata: GuiMetadata,
    ) -> Self {
        let store = project_data.shared_string_store();
        metadata.launder(store);
        gui_metadata.launder(store);
        let mut translations = Translations::new("en");
        translations.launder(store);

        let resolved_path = project_data.resolve_component_path(path);
        let source = ComponentSource::File {
            package: None,
            path: resolved_path.clone(),
        };

        let mut data = UserComponentData {
            file_path: resolved_path,
            source_hash: [0; 32],
            interface_hash: [0; 32],
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
        };
        let initial_file = data.to_file();
        data.source_hash = initial_file.file_hash();
        data.interface_hash = initial_file.interface_hash();

        Self {
            project_data,
            data: Arc::new(RwLock::new(data)),
        }
    }

    /// Opens a user component from the project's file cache, reading from disk if not yet cached.
    ///
    /// Relative paths are resolved inside the project's `components` directory.
    ///
    /// # Errors
    ///
    /// Returns a localized error if the component file cannot be read or parsed.
    pub(crate) fn open(
        project_data: ProjectData,
        path: &Utf8Path,
        package: Option<&PackageReference>,
    ) -> Result<Self, Message> {
        let resolved_path = project_data.resolve_component_path(path);
        let file_package = package.map(PackageReference::to_file);
        let file = project_data
            .file_cache()
            .load_component(&resolved_path, file_package)?;
        let data =
            UserComponentData::from_file(project_data.shared_string_store(), resolved_path, &file);
        Ok(Self {
            project_data,
            data: Arc::new(RwLock::new(data)),
        })
    }

    /// Returns the resolved path of the backing component file.
    #[must_use]
    pub fn file_path(&self) -> Utf8PathBuf {
        self.data.read().file_path.clone()
    }

    /// Converts the current editable state into a frozen [`ComponentFile`] snapshot.
    #[must_use]
    pub fn to_component_file(&self) -> ComponentFile {
        let mut component_file = self.data.read().to_file();
        component_file.launder(self.project_data.shared_string_store());
        component_file
    }

    /// Writes the current editable state to the backing component file and updates the file cache.
    ///
    /// # Errors
    ///
    /// Returns a localized error if validation, serialization, or atomic file replacement fails.
    pub fn save(&self) -> Result<(), Message> {
        let (file_path, file_package, mut component_file) = {
            let guard = self.data.read();
            let file_package = match &guard.source {
                ComponentSource::File { package, .. } => {
                    package.as_ref().map(PackageReference::to_file)
                }
                ComponentSource::Builtin { .. } => None,
            };
            (guard.file_path.clone(), file_package, guard.to_file())
        };
        component_file.launder(self.project_data.shared_string_store());
        let new_file_hash = component_file.file_hash();
        let new_interface_hash = component_file.interface_hash();
        let cached = Arc::new(component_file.clone());
        write_component_file(component_file, &file_path)?;
        self.project_data
            .file_cache()
            .insert_component(&file_path, file_package.as_ref(), cached);
        let mut guard = self.data.write();
        guard.source_hash = new_file_hash;
        guard.interface_hash = new_interface_hash;
        Ok(())
    }

    /// Re-reads the backing component file from disk, refreshes the file cache, and resets editable state.
    ///
    /// # Errors
    ///
    /// Returns a localized error if the component file cannot be read or parsed.
    pub fn reload(&self) -> Result<(), Message> {
        let (file_path, package) = {
            let guard = self.data.read();
            let package = match &guard.source {
                ComponentSource::File { package, .. } => {
                    package.as_ref().map(PackageReference::to_file)
                }
                ComponentSource::Builtin { .. } => None,
            };
            (guard.file_path.clone(), package)
        };
        let file = self
            .project_data
            .file_cache()
            .reload_component(&file_path, package)?;
        *self.data.write() =
            UserComponentData::from_file(self.project_data.shared_string_store(), file_path, &file);
        Ok(())
    }

    /// Returns the BLAKE3 hash of the last loaded or saved component file snapshot.
    #[must_use]
    pub fn source_hash(&self) -> [u8; 32] {
        self.data.read().source_hash
    }

    /// Returns the BLAKE3 interface hash of the last loaded or saved component file snapshot.
    #[must_use]
    pub fn interface_hash(&self) -> [u8; 32] {
        self.data.read().interface_hash
    }

    /// Returns a clone of the component's editable metadata.
    #[must_use]
    pub fn metadata(&self) -> Metadata {
        self.data.read().metadata.clone()
    }

    /// Replaces the component's metadata, laundering its strings into the project store.
    pub fn set_metadata(&self, mut metadata: Metadata) {
        metadata.launder(self.project_data.shared_string_store());
        self.data.write().metadata = metadata;
    }

    /// Runs a mutating closure against the component's metadata, laundering updated strings.
    pub fn with_metadata_mut<R>(&self, f: impl FnOnce(&mut Metadata) -> R) -> R {
        let mut guard = self.data.write();
        let result = f(&mut guard.metadata);
        guard
            .metadata
            .launder(self.project_data.shared_string_store());
        result
    }

    /// Returns the unresolved provenance of the loaded component file.
    #[must_use]
    pub fn source(&self) -> ComponentSource {
        self.data.read().source.clone()
    }

    /// Returns the GUI metadata associated with the component.
    #[must_use]
    pub fn gui_metadata(&self) -> GuiMetadata {
        self.data.read().gui_metadata.clone()
    }

    /// Replaces the component's GUI metadata, laundering its strings into the project store.
    pub fn set_gui_metadata(&self, mut gui_metadata: GuiMetadata) {
        gui_metadata.launder(self.project_data.shared_string_store());
        self.data.write().gui_metadata = gui_metadata;
    }

    /// Returns handles to the ordered nested component instances in this component.
    #[must_use]
    pub fn components(&self) -> Vec<ComponentInstance> {
        self.data.read().components.clone()
    }

    /// Finds a nested component instance in this component by its identifier.
    #[must_use]
    pub fn component(&self, id: &ComponentKey) -> Option<ComponentInstance> {
        self.data
            .read()
            .components
            .iter()
            .find(|instance| instance.id() == *id)
            .cloned()
    }

    /// Resolves a nested instance's source relative to this file, using cached files when available.
    ///
    /// # Errors
    ///
    /// Returns a localized error for a missing instance or an unresolvable source.
    pub fn resolve_component(&self, id: &ComponentKey) -> Result<ResolvedComponent, Message> {
        let instance = self.component(id).ok_or_else(|| component_not_found(id))?;
        self.project_data
            .resolve_component_source(&instance.source(), &self.file_path(), false)
    }

    /// Rereads a nested instance's file source and checks its current source-defined interface.
    ///
    /// Does not mutate the instance. Built-ins use their currently registered exact version.
    ///
    /// # Errors
    ///
    /// Returns a localized error for a missing instance or an unresolvable source.
    pub fn is_component_interface_stale(&self, id: &ComponentKey) -> Result<bool, Message> {
        let instance = self.component(id).ok_or_else(|| component_not_found(id))?;
        let component = self.project_data.resolve_component_source(
            &instance.source(),
            &self.file_path(),
            true,
        )?;
        instance.is_interface_stale(&component)
    }

    /// Rereads a nested instance's source and refreshes its GUI and parameter definitions.
    ///
    /// Preserves compatible values, position, and rotation, and reports added,
    /// removed, and reset parameters. Connections are unchanged. A resolution
    /// failure leaves the instance untouched. Built-ins use the current registry.
    ///
    /// # Errors
    ///
    /// Returns a localized error for a missing instance or an unresolvable source.
    pub fn refresh_component(&self, id: &ComponentKey) -> Result<InterfaceRefresh, Message> {
        let instance = self.component(id).ok_or_else(|| component_not_found(id))?;
        let component = self.project_data.resolve_component_source(
            &instance.source(),
            &self.file_path(),
            true,
        )?;
        instance.refresh_interface(&component)
    }

    /// Adds a nested component instance if no instance with the same identifier exists.
    ///
    /// Returns `true` if inserted, or `false` if an instance with the same ID already exists.
    #[must_use]
    pub fn add_component(&self, instance: ComponentInstance) -> bool {
        let new_id = instance.id();
        let mut guard = self.data.write();
        if guard
            .components
            .iter()
            .any(|existing| existing.id() == new_id)
        {
            return false;
        }
        guard.components.push(instance);
        true
    }

    /// Removes and returns the nested component instance matching `id`, if present.
    #[must_use]
    pub fn remove_component(&self, id: &ComponentKey) -> Option<ComponentInstance> {
        let mut guard = self.data.write();
        let pos = guard
            .components
            .iter()
            .position(|instance| instance.id() == *id)?;
        Some(guard.components.remove(pos))
    }

    /// Instantiates this user component as a [`ComponentInstance`] at `position` with default parameters.
    #[must_use]
    pub fn instantiate(&self, id: ComponentKey, position: Position) -> ComponentInstance {
        let (source, gui_metadata, param_def) = {
            let guard = self.data.read();
            (
                guard.source.clone(),
                guard.gui_metadata.clone(),
                guard.parameter_definition.clone(),
            )
        };
        let parameters = ParameterObjectFrozen::new(param_def);
        ComponentInstance::new(
            self.project_data.shared_string_store().clone(),
            id,
            source,
            position,
            gui_metadata,
            parameters,
        )
    }

    /// Returns the connections between component ports in this component.
    #[must_use]
    pub fn connections(&self) -> Vec<Connection> {
        self.data.read().connections.clone()
    }

    /// Appends a connection, laundering its keys into the project store.
    pub fn add_connection(&self, mut connection: Connection) {
        connection.launder(self.project_data.shared_string_store());
        self.data.write().connections.push(connection);
    }

    /// Removes and returns the connection at `index`, if in bounds.
    #[must_use]
    pub fn remove_connection(&self, index: usize) -> Option<Connection> {
        let mut guard = self.data.write();
        if index < guard.connections.len() {
            Some(guard.connections.remove(index))
        } else {
            None
        }
    }

    /// Replaces all connections, laundering their keys into the project store.
    pub fn set_connections(&self, mut connections: Vec<Connection>) {
        let store = self.project_data.shared_string_store();
        for conn in &mut connections {
            conn.launder(store);
        }
        self.data.write().connections = connections;
    }

    /// Returns the foreground decorations in this component.
    #[must_use]
    pub fn foreground_decorations(&self) -> Vec<ForegroundDecoration> {
        self.data.read().foreground_decorations.clone()
    }

    /// Appends a foreground decoration, laundering its strings into the project store.
    pub fn add_foreground_decoration(&self, mut decoration: ForegroundDecoration) {
        decoration.launder(self.project_data.shared_string_store());
        self.data.write().foreground_decorations.push(decoration);
    }

    /// Removes and returns the first foreground decoration matching `id`, if present.
    #[must_use]
    pub fn remove_foreground_decoration(&self, id: &str) -> Option<ForegroundDecoration> {
        let mut guard = self.data.write();
        let pos = guard
            .foreground_decorations
            .iter()
            .position(|dec| dec.id().as_str() == id)?;
        Some(guard.foreground_decorations.remove(pos))
    }

    /// Replaces all foreground decorations, laundering their strings into the project store.
    pub fn set_foreground_decorations(&self, mut decorations: Vec<ForegroundDecoration>) {
        let store = self.project_data.shared_string_store();
        for dec in &mut decorations {
            dec.launder(store);
        }
        self.data.write().foreground_decorations = decorations;
    }

    /// Returns the background decorations in this component.
    #[must_use]
    pub fn background_decorations(&self) -> Vec<BackgroundDecoration> {
        self.data.read().background_decorations.clone()
    }

    /// Appends a background decoration, laundering its strings into the project store.
    pub fn add_background_decoration(&self, mut decoration: BackgroundDecoration) {
        decoration.launder(self.project_data.shared_string_store());
        self.data.write().background_decorations.push(decoration);
    }

    /// Removes and returns the first background decoration matching `id`, if present.
    #[must_use]
    pub fn remove_background_decoration(&self, id: &str) -> Option<BackgroundDecoration> {
        let mut guard = self.data.write();
        let pos = guard
            .background_decorations
            .iter()
            .position(|dec| dec.id().as_str() == id)?;
        Some(guard.background_decorations.remove(pos))
    }

    /// Replaces all background decorations, laundering their strings into the project store.
    pub fn set_background_decorations(&self, mut decorations: Vec<BackgroundDecoration>) {
        let store = self.project_data.shared_string_store();
        for dec in &mut decorations {
            dec.launder(store);
        }
        self.data.write().background_decorations = decorations;
    }

    /// Returns the translation table associated with this component.
    #[must_use]
    pub fn translations(&self) -> Translations {
        self.data.read().translations.clone()
    }

    /// Replaces the component's translation table, laundering its strings into the project store.
    pub fn set_translations(&self, mut translations: Translations) {
        translations.launder(self.project_data.shared_string_store());
        self.data.write().translations = translations;
    }

    /// Runs a mutating closure against the component's translations, laundering updated strings.
    pub fn with_translations_mut<R>(&self, f: impl FnOnce(&mut Translations) -> R) -> R {
        let mut guard = self.data.write();
        let result = f(&mut guard.translations);
        guard
            .translations
            .launder(self.project_data.shared_string_store());
        result
    }

    /// Returns the ordered parameter definitions for this component.
    #[must_use]
    pub fn parameter_definition(&self) -> ParameterObjectDefinition {
        self.data.read().parameter_definition.clone()
    }

    /// Replaces the component's parameter definitions, laundering their strings into the project store.
    pub fn set_parameter_definition(&self, mut definition: ParameterObjectDefinition) {
        definition.launder(self.project_data.shared_string_store());
        self.data.write().parameter_definition = definition;
    }

    /// Runs a mutating closure against the component's parameter definitions, laundering updated strings.
    pub fn with_parameter_definition_mut<R>(
        &self,
        f: impl FnOnce(&mut ParameterObjectDefinition) -> R,
    ) -> R {
        let mut guard = self.data.write();
        let result = f(&mut guard.parameter_definition);
        guard
            .parameter_definition
            .launder(self.project_data.shared_string_store());
        result
    }

    /// Returns the ordered variable definitions for this component.
    #[must_use]
    pub fn variable_definition(&self) -> VariableObjectDefinition {
        self.data.read().variable_definition.clone()
    }

    /// Replaces the component's variable definitions, laundering their strings into the project store.
    pub fn set_variable_definition(&self, mut definition: VariableObjectDefinition) {
        definition.launder(self.project_data.shared_string_store());
        self.data.write().variable_definition = definition;
    }

    /// Runs a mutating closure against the component's variable definitions, laundering updated strings.
    pub fn with_variable_definition_mut<R>(
        &self,
        f: impl FnOnce(&mut VariableObjectDefinition) -> R,
    ) -> R {
        let mut guard = self.data.write();
        let result = f(&mut guard.variable_definition);
        guard
            .variable_definition
            .launder(self.project_data.shared_string_store());
        result
    }

    /// Returns the shared project context for diagnostic inspection.
    #[cfg(feature = "diagnostics")]
    #[must_use]
    pub fn project_data(&self) -> ProjectData {
        self.project_data.clone()
    }
}
