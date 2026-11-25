use crate::component_instance::{ComponentInstance, InterfaceRefresh};
use crate::component_source::{ResolvedComponent, component_not_found};
use crate::project::ProjectData;
use crate::types::{
    BackgroundDecoration, Connection, ForegroundDecoration, Metadata, Translations,
};
use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_datastore::definition::{
    GlobalObjectDefinition, ParameterObjectDefinition, VariableObjectDefinition,
};
use hrafnix_datastore::editable::GlobalObjectEditable;
use hrafnix_datastore::frozen::GlobalObjectFrozen;
use hrafnix_files::builder::ModelFileBuilder;
use hrafnix_files::data::model::ModelFile;
use hrafnix_files::formats::model::write_model_file;
use hrafnix_keys::ComponentKey;
use hrafnix_message::message::Message;
use hrafnix_shareable_string::SharedStringStore;
use parking_lot::RwLock;
use std::sync::Arc;

/// Internal editable state of an open simulation model.
#[derive(Debug, Clone)]
struct ModelData {
    /// Resolved path to the backing model file.
    file_path: Utf8PathBuf,
    /// BLAKE3 hash of the last loaded or saved model file snapshot.
    source_hash: [u8; 32],
    /// Editable descriptive information about the model.
    metadata: Metadata,
    /// Text and images drawn in front of the model's contents.
    foreground_decorations: Vec<ForegroundDecoration>,
    /// Shapes drawn behind the model's contents.
    background_decorations: Vec<BackgroundDecoration>,
    /// Frozen global settings for the model.
    global_settings: GlobalObjectFrozen,
    /// Ordered editable component instances placed in the model.
    components: Vec<ComponentInstance>,
    /// Connections between component ports within the model.
    connections: Vec<Connection>,
    /// Source language and translated text used by the model.
    translations: Translations,
    /// Ordered parameter definitions, including defaults and descriptions.
    parameter_definition: ParameterObjectDefinition,
    /// Ordered variable definitions, including defaults and descriptions.
    variable_definition: VariableObjectDefinition,
}

impl ModelData {
    /// Converts a loaded [`ModelFile`] snapshot into editable model state.
    fn from_file(
        shared_string_store: &SharedStringStore,
        file_path: Utf8PathBuf,
        file: &ModelFile,
    ) -> Self {
        Self {
            file_path,
            source_hash: file.file_hash(),
            metadata: Metadata::from_file(file.metadata()),
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
            global_settings: file.global_settings().clone(),
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

    /// Converts editable model state back into a frozen [`ModelFile`] snapshot.
    fn to_file(&self) -> ModelFile {
        ModelFileBuilder::new(
            self.metadata.to_file(),
            self.global_settings.clone(),
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

/// Shared, interior-mutable handle to an editable simulation model.
#[derive(Debug, Clone)]
pub struct Model {
    /// Shared project context.
    project_data: ProjectData,
    /// Shared editable model state.
    data: Arc<RwLock<ModelData>>,
}

impl Model {
    /// Creates a new in-memory editable model backed by `path` (not yet written to disk).
    ///
    /// Relative paths are resolved inside the project's `models` directory.
    #[must_use]
    pub(crate) fn create(
        project_data: ProjectData,
        path: &Utf8Path,
        mut metadata: Metadata,
    ) -> Self {
        let store = project_data.shared_string_store();
        metadata.launder(store);
        let mut translations = Translations::new("en");
        translations.launder(store);
        let mut global_settings = GlobalObjectFrozen::new(GlobalObjectDefinition::default());
        global_settings.launder(store);

        let resolved_path = project_data.resolve_model_path(path);
        let mut data = ModelData {
            file_path: resolved_path,
            source_hash: [0; 32],
            metadata,
            foreground_decorations: Vec::new(),
            background_decorations: Vec::new(),
            global_settings,
            components: Vec::new(),
            connections: Vec::new(),
            translations,
            parameter_definition: ParameterObjectDefinition::default(),
            variable_definition: VariableObjectDefinition::default(),
        };
        data.source_hash = data.to_file().file_hash();

        Self {
            project_data,
            data: Arc::new(RwLock::new(data)),
        }
    }

    /// Opens a model from the project's file cache, reading from disk if not yet cached.
    ///
    /// Relative paths are resolved inside the project's `models` directory.
    ///
    /// # Errors
    ///
    /// Returns a localized error if the model file cannot be read or parsed.
    pub(crate) fn open(project_data: ProjectData, path: &Utf8Path) -> Result<Self, Message> {
        let resolved_path = project_data.resolve_model_path(path);
        let file = project_data.file_cache().load_model(&resolved_path)?;
        let data = ModelData::from_file(project_data.shared_string_store(), resolved_path, &file);
        Ok(Self {
            project_data,
            data: Arc::new(RwLock::new(data)),
        })
    }

    /// Returns the resolved path of the backing model file.
    #[must_use]
    pub fn file_path(&self) -> Utf8PathBuf {
        self.data.read().file_path.clone()
    }

    /// Converts the current editable state into a frozen [`ModelFile`] snapshot.
    #[must_use]
    pub fn to_model_file(&self) -> ModelFile {
        let mut model_file = self.data.read().to_file();
        model_file.launder(self.project_data.shared_string_store());
        model_file
    }

    /// Writes the current editable state to the backing model file and updates the file cache.
    ///
    /// # Errors
    ///
    /// Returns a localized error if validation, serialization, or atomic file replacement fails.
    pub fn save(&self) -> Result<(), Message> {
        let (file_path, mut model_file) = {
            let guard = self.data.read();
            (guard.file_path.clone(), guard.to_file())
        };
        model_file.launder(self.project_data.shared_string_store());
        let new_hash = model_file.file_hash();
        let cached = Arc::new(model_file.clone());
        write_model_file(model_file, &file_path)?;
        self.project_data
            .file_cache()
            .insert_model(file_path, cached);
        self.data.write().source_hash = new_hash;
        Ok(())
    }

    /// Re-reads the backing model file from disk, refreshes the file cache, and resets editable state.
    ///
    /// # Errors
    ///
    /// Returns a localized error if the model file cannot be read or parsed.
    pub fn reload(&self) -> Result<(), Message> {
        let file_path = self.file_path();
        let file = self.project_data.file_cache().reload_model(&file_path)?;
        *self.data.write() =
            ModelData::from_file(self.project_data.shared_string_store(), file_path, &file);
        Ok(())
    }

    /// Returns the BLAKE3 hash of the last loaded or saved model file snapshot.
    #[must_use]
    pub fn source_hash(&self) -> [u8; 32] {
        self.data.read().source_hash
    }

    /// Returns a clone of the model's editable metadata.
    #[must_use]
    pub fn metadata(&self) -> Metadata {
        self.data.read().metadata.clone()
    }

    /// Replaces the model's metadata, laundering its strings into the project store.
    pub fn set_metadata(&self, mut metadata: Metadata) {
        metadata.launder(self.project_data.shared_string_store());
        self.data.write().metadata = metadata;
    }

    /// Runs a mutating closure against the model's metadata, laundering updated strings.
    pub fn with_metadata_mut<R>(&self, f: impl FnOnce(&mut Metadata) -> R) -> R {
        let mut guard = self.data.write();
        let result = f(&mut guard.metadata);
        guard
            .metadata
            .launder(self.project_data.shared_string_store());
        result
    }

    /// Returns a cloned frozen snapshot of the model's global settings.
    #[must_use]
    pub fn global_settings(&self) -> GlobalObjectFrozen {
        self.data.read().global_settings.clone()
    }

    /// Thaws the model's frozen global settings into an editable copy.
    #[must_use]
    pub fn thaw_global_settings(&self) -> GlobalObjectEditable {
        self.data.read().global_settings.thaw()
    }

    /// Freezes `edited`, launders its strings, and merges its values back into global settings.
    pub fn merge_global_settings(&self, edited: &GlobalObjectEditable) {
        let mut frozen = edited.freeze();
        frozen.launder(self.project_data.shared_string_store());
        self.data.write().global_settings.merge_from(&frozen);
    }

    /// Thaws global settings, applies `f` without holding the write lock, and merges changes back in.
    pub fn edit_global_settings<R>(&self, f: impl FnOnce(&mut GlobalObjectEditable) -> R) -> R {
        let mut editable = self.thaw_global_settings();
        let result = f(&mut editable);
        self.merge_global_settings(&editable);
        result
    }

    /// Returns handles to the ordered component instances in this model.
    #[must_use]
    pub fn components(&self) -> Vec<ComponentInstance> {
        self.data.read().components.clone()
    }

    /// Finds a component instance in this model by its identifier.
    #[must_use]
    pub fn component(&self, id: &ComponentKey) -> Option<ComponentInstance> {
        self.data
            .read()
            .components
            .iter()
            .find(|instance| instance.id() == *id)
            .cloned()
    }

    /// Resolves an instance's source relative to this model, using cached files when available.
    ///
    /// # Errors
    ///
    /// Returns a localized error for a missing instance or an unresolvable source.
    pub fn resolve_component(&self, id: &ComponentKey) -> Result<ResolvedComponent, Message> {
        let instance = self.component(id).ok_or_else(|| component_not_found(id))?;
        self.project_data
            .resolve_component_source(&instance.source(), &self.file_path(), false)
    }

    /// Rereads an instance's file source and checks its current source-defined interface.
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

    /// Rereads an instance's file source and refreshes its GUI and parameter definitions.
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

    /// Adds a component instance if no instance with the same identifier exists.
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

    /// Removes and returns the component instance matching `id`, if present.
    #[must_use]
    pub fn remove_component(&self, id: &ComponentKey) -> Option<ComponentInstance> {
        let mut guard = self.data.write();
        let pos = guard
            .components
            .iter()
            .position(|instance| instance.id() == *id)?;
        Some(guard.components.remove(pos))
    }

    /// Returns the connections between component ports in this model.
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

    /// Returns the foreground decorations in this model.
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

    /// Returns the background decorations in this model.
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

    /// Returns the translation table associated with this model.
    #[must_use]
    pub fn translations(&self) -> Translations {
        self.data.read().translations.clone()
    }

    /// Replaces the model's translation table, laundering its strings into the project store.
    pub fn set_translations(&self, mut translations: Translations) {
        translations.launder(self.project_data.shared_string_store());
        self.data.write().translations = translations;
    }

    /// Runs a mutating closure against the model's translations, laundering updated strings.
    pub fn with_translations_mut<R>(&self, f: impl FnOnce(&mut Translations) -> R) -> R {
        let mut guard = self.data.write();
        let result = f(&mut guard.translations);
        guard
            .translations
            .launder(self.project_data.shared_string_store());
        result
    }

    /// Returns the ordered parameter definitions for this model.
    #[must_use]
    pub fn parameter_definition(&self) -> ParameterObjectDefinition {
        self.data.read().parameter_definition.clone()
    }

    /// Replaces the model's parameter definitions, laundering their strings into the project store.
    pub fn set_parameter_definition(&self, mut definition: ParameterObjectDefinition) {
        definition.launder(self.project_data.shared_string_store());
        self.data.write().parameter_definition = definition;
    }

    /// Runs a mutating closure against the model's parameter definitions, laundering updated strings.
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

    /// Returns the ordered variable definitions for this model.
    #[must_use]
    pub fn variable_definition(&self) -> VariableObjectDefinition {
        self.data.read().variable_definition.clone()
    }

    /// Replaces the model's variable definitions, laundering their strings into the project store.
    pub fn set_variable_definition(&self, mut definition: VariableObjectDefinition) {
        definition.launder(self.project_data.shared_string_store());
        self.data.write().variable_definition = definition;
    }

    /// Runs a mutating closure against the model's variable definitions, laundering updated strings.
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
