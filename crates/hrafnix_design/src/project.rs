use crate::built_in_component::BuiltInComponent;
use crate::component_instance::ComponentInstance;
use crate::component_source::ResolvedComponent;
use crate::file_cache::FileCache;
use crate::model::Model;
use crate::types::{ComponentSource, GuiMetadata, Metadata, PackageReference, Position};
use crate::user_component::UserComponent;
use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_files::builder::project_file;
use hrafnix_files::data::project::ProjectFile;
use hrafnix_files::formats::project::{create_project_file, read_project_file, write_project_file};
use hrafnix_keys::ComponentKey;
use hrafnix_message::message::{Message, MessageCategory, MessageLevel};
use hrafnix_shareable_string::{ShareableString, SharedStringStore, SharedStringTranslationMap};
use hrafnix_translation::generate_translation_map;
use parking_lot::{Mutex, RwLock};
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::io;
use std::sync::Arc;

/// Shared editable project metadata, replaced together when reloading a manifest.
#[derive(Debug)]
struct ProjectMetadata {
    /// User-visible project name.
    name: ShareableString,
    /// Project version string.
    version: ShareableString,
}

/// Exact-version built-in descriptors shared by an open project.
type BuiltInComponents = FxHashMap<(ShareableString, u32), Arc<BuiltInComponent>>;

/// Directory name holding project dependencies (`<project>/.hrafnix`).
pub const DEPENDENCIES_DIR: &str = ".hrafnix";
/// Directory name holding miscellaneous project data files (`<project>/data`).
pub const DATA_DIR: &str = "data";
/// Directory name holding user component `.hxc` files (`<project>/components`).
pub const COMPONENTS_DIR: &str = "components";
/// Directory name holding model `.hxm` files (`<project>/models`).
pub const MODELS_DIR: &str = "models";
/// File name holding project configuration (`<project>/hrafnix.toml`).
pub const MANIFEST_FILE: &str = "hrafnix.toml";

/// Shared interior-mutable state backing a [`Project`] and its open design objects.
#[derive(Debug, Clone)]
pub struct ProjectData {
    /// Root working directory of the project.
    working_directory: Arc<Utf8PathBuf>,
    /// Shared interned string store for the project.
    shared_string_store: SharedStringStore,
    /// Shared translation catalog for the project.
    translation_map: SharedStringTranslationMap,
    /// Shared cache of loaded model and component file snapshots.
    file_cache: FileCache,
    /// Shared project name and version.
    metadata: Arc<Mutex<ProjectMetadata>>,
    /// Immutable built-in descriptors keyed by exact name and version.
    built_in_components: Arc<RwLock<BuiltInComponents>>,
    /// Explicit roots for relative paths in exact package versions.
    package_roots: Arc<RwLock<FxHashMap<PackageReference, Utf8PathBuf>>>,
}

impl ProjectData {
    /// Returns the project's root working directory.
    #[must_use]
    pub(crate) fn working_directory(&self) -> &Utf8Path {
        self.working_directory.as_path()
    }

    /// Returns the path to `<project>/.hrafnix`.
    #[must_use]
    pub(crate) fn dependencies_dir(&self) -> Utf8PathBuf {
        self.working_directory.join(DEPENDENCIES_DIR)
    }

    /// Returns the path to `<project>/data`.
    #[must_use]
    pub(crate) fn data_dir(&self) -> Utf8PathBuf {
        self.working_directory.join(DATA_DIR)
    }

    /// Returns the path to `<project>/components`.
    #[must_use]
    pub(crate) fn components_dir(&self) -> Utf8PathBuf {
        self.working_directory.join(COMPONENTS_DIR)
    }

    /// Returns the path to `<project>/models`.
    #[must_use]
    pub(crate) fn models_dir(&self) -> Utf8PathBuf {
        self.working_directory.join(MODELS_DIR)
    }

    /// Returns the path to `<project>/hrafnix.toml`.
    #[must_use]
    pub(crate) fn manifest_path(&self) -> Utf8PathBuf {
        self.working_directory.join(MANIFEST_FILE)
    }

    /// Resolves a model path inside `<project>/models` when relative.
    #[must_use]
    pub(crate) fn resolve_model_path(&self, path: &Utf8Path) -> Utf8PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.models_dir().join(path)
        }
    }

    /// Resolves a user component path inside `<project>/components` when relative.
    #[must_use]
    pub(crate) fn resolve_component_path(&self, path: &Utf8Path) -> Utf8PathBuf {
        if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.components_dir().join(path)
        }
    }

    /// Resolves a source using registered provenance and the containing file's directory.
    ///
    /// `reload` bypasses cached file snapshots; no project or registry lock is held during I/O.
    pub(crate) fn resolve_component_source(
        &self,
        source: &ComponentSource,
        containing_file: &Utf8Path,
        reload: bool,
    ) -> Result<ResolvedComponent, Message> {
        let mut source = source.clone();
        source.launder(self.shared_string_store());
        match &source {
            ComponentSource::Builtin { name, version } => {
                let component = self
                    .built_in_components
                    .read()
                    .get(&(name.clone(), *version))
                    .cloned()
                    .ok_or_else(|| {
                        Message::new_with_params(
                            MessageLevel::Error,
                            MessageCategory::Component,
                            "design_builtin_not_found".into(),
                            HashMap::from([
                                ("name".into(), name.clone()),
                                ("version".into(), version.to_string().into()),
                            ]),
                            None,
                        )
                    })?;
                Ok(ResolvedComponent::from_builtin(source, component))
            }
            ComponentSource::File { package, path } => {
                let resolved_path = if path.is_absolute() {
                    path.clone()
                } else if let Some(package) = package {
                    let root =
                        self.package_roots
                            .read()
                            .get(package)
                            .cloned()
                            .ok_or_else(|| {
                                let version = package.version();
                                Message::new_with_params(
                                    MessageLevel::Error,
                                    MessageCategory::Component,
                                    "design_package_not_registered".into(),
                                    HashMap::from([
                                        ("name".into(), package.name().clone()),
                                        (
                                            "version".into(),
                                            format!(
                                                "{}.{}.{}",
                                                version.major(),
                                                version.minor(),
                                                version.patch()
                                            )
                                            .into(),
                                        ),
                                    ]),
                                    None,
                                )
                            })?;
                    root.join(path)
                } else {
                    let parent = containing_file.parent().ok_or_else(|| {
                        Message::error_with_param(
                            MessageCategory::File,
                            "files_io_invalid_input",
                            "path",
                            containing_file.as_str(),
                        )
                    })?;
                    parent.join(path)
                };
                let package = package.as_ref().map(PackageReference::to_file);
                let file = if reload {
                    self.file_cache.reload_component(&resolved_path, package)?
                } else {
                    self.file_cache.load_component(&resolved_path, package)?
                };
                Ok(ResolvedComponent::from_file(source, file))
            }
        }
    }

    /// Returns the shared interned string store.
    #[must_use]
    pub(crate) const fn shared_string_store(&self) -> &SharedStringStore {
        &self.shared_string_store
    }

    /// Returns the shared translation catalog.
    #[must_use]
    pub(crate) const fn translation_map(&self) -> &SharedStringTranslationMap {
        &self.translation_map
    }

    /// Returns the shared file cache.
    #[must_use]
    pub(crate) const fn file_cache(&self) -> &FileCache {
        &self.file_cache
    }

    /// Returns the project name.
    #[must_use]
    pub(crate) fn name(&self) -> ShareableString {
        self.metadata.lock().name.clone()
    }

    /// Sets the project name.
    pub(crate) fn set_name(&self, name: ShareableString) {
        self.metadata.lock().name = name;
    }

    /// Returns the project version.
    #[must_use]
    pub(crate) fn version(&self) -> ShareableString {
        self.metadata.lock().version.clone()
    }

    /// Sets the project version.
    pub(crate) fn set_version(&self, version: ShareableString) {
        self.metadata.lock().version = version;
    }
}

/// Top-level handle for an open Hrafnix project and its shared resources.
///
/// Project directory layout:
/// - `<root>/.hrafnix` — project dependencies
/// - `<root>/data` — miscellaneous project data files
/// - `<root>/components` — user component `.hxc` files
/// - `<root>/models` — simulation model `.hxm` files
/// - `<root>/hrafnix.toml` — project manifest
#[derive(Debug, Clone)]
pub struct Project {
    /// Shared project state.
    data: ProjectData,
}

impl Project {
    /// Opens an in-memory project context rooted at `path`, with empty name and version.
    ///
    /// Does not read a manifest, create directories, or otherwise touch the filesystem.
    /// Use [`Self::load_project`] to load an existing manifest or
    /// [`Self::create_project`] to create the standard project layout on disk.
    #[must_use]
    pub fn open_project(path: &Utf8Path) -> Self {
        let working_directory = path.to_path_buf();
        let shared_string_store = SharedStringStore::new();
        shared_string_store.add("");
        let translation_map = generate_translation_map(&shared_string_store);

        let file_cache = FileCache::new(shared_string_store.clone());
        let metadata = ProjectMetadata {
            name: shared_string_store.get(""),
            version: shared_string_store.get(""),
        };

        Self {
            data: ProjectData {
                working_directory: Arc::new(working_directory),
                shared_string_store,
                translation_map,
                file_cache,
                metadata: Arc::new(Mutex::new(metadata)),
                built_in_components: Arc::new(RwLock::new(FxHashMap::default())),
                package_roots: Arc::new(RwLock::new(FxHashMap::default())),
            },
        }
    }

    /// Loads a project rooted at `path` from its required `hrafnix.toml` manifest.
    ///
    /// Does not create or require the standard subdirectories; models and components
    /// are loaded on demand. Name and version are interned in the project's store.
    ///
    /// # Errors
    ///
    /// Returns a localized error if the manifest cannot be read or parsed.
    pub fn load_project(path: &Utf8Path) -> Result<Self, Message> {
        let project = Self::open_project(path);
        project.reload()?;
        Ok(project)
    }

    /// Creates the standard project directories and a new `hrafnix.toml` manifest.
    ///
    /// An existing root directory and its contents are preserved, but an existing
    /// manifest path is never replaced, including during concurrent creation.
    /// If creation fails, directories already created are left in place. The
    /// manifest is published only after all directories have been created.
    /// Atomic manifest creation requires filesystem support for hard links.
    ///
    /// # Errors
    ///
    /// Returns a localized error if the manifest already exists, directory creation
    /// fails, or the manifest cannot be serialized or created.
    pub fn create_project(path: &Utf8Path, name: &str, version: &str) -> Result<Self, Message> {
        let project = Self::open_project(path);
        let manifest_path = project.manifest_path();
        match std::fs::symlink_metadata(&manifest_path) {
            Ok(_) => {
                return Err(Message::error_with_param(
                    MessageCategory::File,
                    "files_io_already_exists",
                    "path",
                    manifest_path.as_str(),
                ));
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(project_io_error(&manifest_path, &error)),
        }

        for directory in [
            project.dependencies_dir(),
            project.data_dir(),
            project.components_dir(),
            project.models_dir(),
        ] {
            std::fs::create_dir_all(&directory)
                .map_err(|error| project_io_error(&directory, &error))?;
        }
        project.set_name(name);
        project.set_version(version);
        create_project_file(&project.to_project_file(), &manifest_path)?;
        Ok(project)
    }

    /// Returns an immutable snapshot of the current project name and version.
    #[must_use]
    pub fn to_project_file(&self) -> ProjectFile {
        let metadata = self.data.metadata.lock();
        project_file(metadata.name.clone(), metadata.version.clone())
    }

    /// Atomically saves the current project name and version to `hrafnix.toml`.
    ///
    /// Does not create directories or save open models and components.
    ///
    /// # Errors
    ///
    /// Returns a localized error if serialization or atomic manifest replacement fails.
    pub fn save(&self) -> Result<(), Message> {
        write_project_file(&self.to_project_file(), &self.manifest_path())
    }

    /// Reloads name and version from the manifest, updating all cloned project handles.
    ///
    /// Keeps the string store, file cache, and open design objects intact. If reading
    /// or parsing fails, existing metadata is unchanged.
    ///
    /// # Errors
    ///
    /// Returns a localized error if the manifest cannot be read or parsed.
    pub fn reload(&self) -> Result<(), Message> {
        let mut file = read_project_file(&self.manifest_path())?;
        file.launder(self.shared_string_store());
        *self.data.metadata.lock() = ProjectMetadata {
            name: file.name().clone(),
            version: file.version().clone(),
        };
        Ok(())
    }

    /// Returns the project's root working directory.
    #[must_use]
    pub fn working_directory(&self) -> &Utf8Path {
        self.data.working_directory()
    }

    /// Returns the path to `<project>/.hrafnix` holding dependencies.
    #[must_use]
    pub fn dependencies_dir(&self) -> Utf8PathBuf {
        self.data.dependencies_dir()
    }

    /// Returns the path to `<project>/data` holding miscellaneous data files.
    #[must_use]
    pub fn data_dir(&self) -> Utf8PathBuf {
        self.data.data_dir()
    }

    /// Returns the path to `<project>/components` holding `.hxc` files.
    #[must_use]
    pub fn components_dir(&self) -> Utf8PathBuf {
        self.data.components_dir()
    }

    /// Returns the path to `<project>/models` holding `.hxm` files.
    #[must_use]
    pub fn models_dir(&self) -> Utf8PathBuf {
        self.data.models_dir()
    }

    /// Returns the path to `<project>/hrafnix.toml` holding project data.
    #[must_use]
    pub fn manifest_path(&self) -> Utf8PathBuf {
        self.data.manifest_path()
    }

    /// Returns the shared interned string store for this project.
    #[must_use]
    pub const fn shared_string_store(&self) -> &SharedStringStore {
        self.data.shared_string_store()
    }

    /// Returns the shared translation catalog for this project.
    #[must_use]
    pub const fn translation_map(&self) -> &SharedStringTranslationMap {
        self.data.translation_map()
    }

    /// Returns the shared cache of loaded model and component file snapshots.
    #[must_use]
    pub const fn file_cache(&self) -> &FileCache {
        self.data.file_cache()
    }

    /// Registers or replaces a built-in descriptor for its exact name and version.
    ///
    /// Returns the previous descriptor, if any. Existing resolved snapshots remain
    /// immutable; subsequent resolution sees the replacement. Strings are interned
    /// in this project's store, and registration is shared by cloned project handles.
    #[must_use]
    pub fn register_built_in(
        &self,
        mut component: BuiltInComponent,
    ) -> Option<Arc<BuiltInComponent>> {
        component.launder(self.shared_string_store());
        self.data.built_in_components.write().insert(
            (component.name().clone(), component.version()),
            Arc::new(component),
        )
    }

    /// Registers or replaces the root for an exact package version.
    ///
    /// Relative roots use the project directory. Relative packaged source paths
    /// use this root directly, without an implicit `components` subdirectory.
    /// Returns the previous root, if any. Registration does not perform file I/O
    /// or alter the project manifest and is shared by cloned project handles.
    pub fn register_package_root(
        &self,
        mut package: PackageReference,
        path: impl AsRef<Utf8Path>,
    ) -> Option<Utf8PathBuf> {
        package.launder(self.shared_string_store());
        let path = path.as_ref();
        let root = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.working_directory().join(path)
        };
        self.data.package_roots.write().insert(package, root)
    }

    /// Resolves a component source, using a cached file snapshot when available.
    ///
    /// Loose relative paths use `containing_file`'s directory. Packaged relative
    /// paths require an explicitly registered package root. Absolute paths are
    /// used unchanged, retaining any package provenance. Built-in names and
    /// versions must match a registered descriptor exactly. Resolution is shallow.
    ///
    /// # Errors
    ///
    /// Returns a localized error for unregistered sources, invalid containing
    /// paths, or file I/O and parsing failures.
    pub fn resolve_component_source(
        &self,
        source: &ComponentSource,
        containing_file: &Utf8Path,
    ) -> Result<ResolvedComponent, Message> {
        self.data
            .resolve_component_source(source, containing_file, false)
    }

    /// Resolves a source after rereading its file and replacing its cached snapshot.
    ///
    /// Built-ins use the currently registered exact version without file I/O.
    /// Existing resolved snapshots and instances are not changed.
    ///
    /// # Errors
    ///
    /// Returns the same localized errors as [`Self::resolve_component_source`].
    pub fn reload_component_source(
        &self,
        source: &ComponentSource,
        containing_file: &Utf8Path,
    ) -> Result<ResolvedComponent, Message> {
        self.data
            .resolve_component_source(source, containing_file, true)
    }

    /// Creates a new in-memory editable [`Model`] backed by `<project>/models/<path>` (or an absolute path).
    ///
    /// Call [`Model::save`] to write the new model file to disk.
    #[must_use]
    pub fn create_model(&self, path: impl AsRef<Utf8Path>, metadata: Metadata) -> Model {
        Model::create(self.data.clone(), path.as_ref(), metadata)
    }

    /// Opens an editable [`Model`] from `<project>/models/<path>` (or an absolute path).
    ///
    /// # Errors
    ///
    /// Returns a localized error if the model file cannot be read or parsed.
    pub fn open_model(&self, path: impl AsRef<Utf8Path>) -> Result<Model, Message> {
        Model::open(self.data.clone(), path.as_ref())
    }

    /// Creates a new in-memory editable [`UserComponent`] backed by `<project>/components/<path>` (or an absolute path).
    ///
    /// Call [`UserComponent::save`] to write the new component file to disk.
    #[must_use]
    pub fn create_user_component(
        &self,
        path: impl AsRef<Utf8Path>,
        metadata: Metadata,
        gui_metadata: GuiMetadata,
    ) -> UserComponent {
        UserComponent::create(self.data.clone(), path.as_ref(), metadata, gui_metadata)
    }

    /// Opens an editable [`UserComponent`] from `<project>/components/<path>` (or an absolute path).
    ///
    /// # Errors
    ///
    /// Returns a localized error if the component file cannot be read or parsed.
    pub fn open_user_component(
        &self,
        path: impl AsRef<Utf8Path>,
        package: Option<&PackageReference>,
    ) -> Result<UserComponent, Message> {
        UserComponent::open(self.data.clone(), path.as_ref(), package)
    }

    /// Instantiates a [`BuiltInComponent`] into a [`ComponentInstance`] using this project's string store.
    ///
    /// Does not register the descriptor. Use [`Self::register_built_in`] to make
    /// its exact name and version available for later source resolution.
    #[must_use]
    pub fn instantiate_built_in(
        &self,
        component: &BuiltInComponent,
        id: ComponentKey,
        position: Position,
    ) -> ComponentInstance {
        component.instantiate(self.shared_string_store(), id, position)
    }

    /// Returns a clone of the shared internal project state for diagnostic inspection.
    #[cfg(feature = "diagnostics")]
    #[must_use]
    pub fn data(&self) -> ProjectData {
        self.data.clone()
    }

    /// Returns the name of the project.
    #[must_use]
    pub fn name(&self) -> ShareableString {
        self.data.name()
    }

    /// Sets the name of the project.
    pub fn set_name(&self, name: &str) {
        let shareable_name = self.data.shared_string_store.get(name);
        self.data.set_name(shareable_name);
    }

    /// Returns the version of the project.
    #[must_use]
    pub fn version(&self) -> ShareableString {
        self.data.version()
    }

    /// Sets the version of the project.
    pub fn set_version(&self, version: &str) {
        let shareable_version = self.data.shared_string_store.get(version);
        self.data.set_version(shareable_version);
    }
}

/// Reports project directory I/O errors with a localized message and OS details.
fn project_io_error(path: &Utf8Path, error: &io::Error) -> Message {
    Message::new_with_params(
        MessageLevel::Error,
        MessageCategory::File,
        "files_io_error".into(),
        HashMap::from([("path".into(), path.as_str().into())]),
        Some(error.to_string().into()),
    )
}
