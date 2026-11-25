use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_files::data::component::ComponentFile;
use hrafnix_files::data::model::ModelFile;
use hrafnix_files::data::source::{PackageReference, PackageVersion};
use hrafnix_files::formats::component::read_component_file;
use hrafnix_files::formats::model::read_model_file;
use hrafnix_message::message::Message;
use hrafnix_shareable_string::{ShareableString, SharedStringStore};
use parking_lot::RwLock;
use rustc_hash::FxHashMap;
use std::sync::Arc;

/// Cache key distinguishing a component file path and its optional package provenance.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ComponentCacheKey {
    /// Resolved path to the component file.
    path: Utf8PathBuf,
    /// Optional `(package_name, package_version)` provenance.
    package: Option<(ShareableString, PackageVersion)>,
}

impl ComponentCacheKey {
    /// Builds a cache key from a component path and optional package reference.
    fn new(path: &Utf8Path, package: Option<&PackageReference>) -> Self {
        Self {
            path: path.to_path_buf(),
            package: package.map(|pkg| (pkg.name().clone(), pkg.version())),
        }
    }
}

/// Internal storage for cached model and component file snapshots.
#[derive(Debug, Default)]
struct FileCacheInner {
    /// Cached model file snapshots keyed by resolved file path.
    models: FxHashMap<Utf8PathBuf, Arc<ModelFile>>,
    /// Cached component file snapshots keyed by resolved file path and package provenance.
    components: FxHashMap<ComponentCacheKey, Arc<ComponentFile>>,
}

/// Thread-safe cache of loaded [`ModelFile`] and [`ComponentFile`] snapshots.
///
/// Every file loaded or reloaded through the cache is laundered against the
/// project's [`SharedStringStore`] to deduplicate strings across snapshots.
#[derive(Debug, Clone)]
pub struct FileCache {
    /// Shared interned string store used to launder loaded files.
    shared_string_store: SharedStringStore,
    /// Shared interior-mutable cache maps.
    inner: Arc<RwLock<FileCacheInner>>,
}

impl FileCache {
    /// Creates an empty file cache backed by `shared_string_store`.
    #[must_use]
    pub fn new(shared_string_store: SharedStringStore) -> Self {
        Self {
            shared_string_store,
            inner: Arc::new(RwLock::new(FileCacheInner::default())),
        }
    }

    /// Returns a cached model file snapshot if present, or reads, launders, and caches it from disk.
    ///
    /// # Errors
    ///
    /// Returns a localized error if reading or parsing the model file fails.
    pub fn load_model(&self, path: &Utf8Path) -> Result<Arc<ModelFile>, Message> {
        if let Some(cached) = self.get_model(path) {
            return Ok(cached);
        }

        let mut model_file = read_model_file(path)?;
        model_file.launder(&self.shared_string_store);
        let loaded = Arc::new(model_file);
        let mut guard = self.inner.write();
        let entry = guard
            .models
            .entry(path.to_path_buf())
            .or_insert_with(|| Arc::clone(&loaded));
        Ok(Arc::clone(entry))
    }

    /// Re-reads a model file from disk, launders it, replaces any cached snapshot, and returns it.
    ///
    /// # Errors
    ///
    /// Returns a localized error if reading or parsing the model file fails.
    pub fn reload_model(&self, path: &Utf8Path) -> Result<Arc<ModelFile>, Message> {
        let mut model_file = read_model_file(path)?;
        model_file.launder(&self.shared_string_store);
        let loaded = Arc::new(model_file);
        self.inner
            .write()
            .models
            .insert(path.to_path_buf(), Arc::clone(&loaded));
        Ok(loaded)
    }

    /// Returns a cached model file snapshot without reading from disk.
    #[must_use]
    pub fn get_model(&self, path: &Utf8Path) -> Option<Arc<ModelFile>> {
        self.inner.read().models.get(path).map(Arc::clone)
    }

    /// Stores a model file snapshot in the cache for `path`.
    pub(crate) fn insert_model(&self, path: Utf8PathBuf, file: Arc<ModelFile>) {
        self.inner.write().models.insert(path, file);
    }

    /// Removes and returns a cached model file snapshot if present.
    #[must_use]
    pub fn invalidate_model(&self, path: &Utf8Path) -> Option<Arc<ModelFile>> {
        self.inner.write().models.remove(path)
    }

    /// Returns a cached component file snapshot if present, or reads, launders, and caches it from disk.
    ///
    /// # Errors
    ///
    /// Returns a localized error if reading or parsing the component file fails.
    pub fn load_component(
        &self,
        path: &Utf8Path,
        mut package: Option<PackageReference>,
    ) -> Result<Arc<ComponentFile>, Message> {
        if let Some(pkg) = &mut package {
            pkg.launder(&self.shared_string_store);
        }
        let key = ComponentCacheKey::new(path, package.as_ref());
        if let Some(cached) = self.inner.read().components.get(&key).map(Arc::clone) {
            return Ok(cached);
        }

        let mut component_file = read_component_file(path, package)?;
        component_file.launder(&self.shared_string_store);
        let loaded = Arc::new(component_file);
        let mut guard = self.inner.write();
        let entry = guard
            .components
            .entry(key)
            .or_insert_with(|| Arc::clone(&loaded));
        Ok(Arc::clone(entry))
    }

    /// Re-reads a component file from disk, launders it, replaces any cached snapshot, and returns it.
    ///
    /// # Errors
    ///
    /// Returns a localized error if reading or parsing the component file fails.
    pub fn reload_component(
        &self,
        path: &Utf8Path,
        mut package: Option<PackageReference>,
    ) -> Result<Arc<ComponentFile>, Message> {
        if let Some(pkg) = &mut package {
            pkg.launder(&self.shared_string_store);
        }
        let key = ComponentCacheKey::new(path, package.as_ref());
        let mut component_file = read_component_file(path, package)?;
        component_file.launder(&self.shared_string_store);
        let loaded = Arc::new(component_file);
        self.inner
            .write()
            .components
            .insert(key, Arc::clone(&loaded));
        Ok(loaded)
    }

    /// Returns a cached component file snapshot without reading from disk.
    #[must_use]
    pub fn get_component(
        &self,
        path: &Utf8Path,
        package: Option<&PackageReference>,
    ) -> Option<Arc<ComponentFile>> {
        let key = ComponentCacheKey::new(path, package);
        self.inner.read().components.get(&key).map(Arc::clone)
    }

    /// Stores a component file snapshot in the cache for `path` and `package`.
    pub(crate) fn insert_component(
        &self,
        path: &Utf8Path,
        package: Option<&PackageReference>,
        file: Arc<ComponentFile>,
    ) {
        let key = ComponentCacheKey::new(path, package);
        self.inner.write().components.insert(key, file);
    }

    /// Removes and returns a cached component file snapshot if present.
    #[must_use]
    pub fn invalidate_component(
        &self,
        path: &Utf8Path,
        package: Option<&PackageReference>,
    ) -> Option<Arc<ComponentFile>> {
        let key = ComponentCacheKey::new(path, package);
        self.inner.write().components.remove(&key)
    }

    /// Removes all cached model and component file snapshots.
    pub fn clear(&self) {
        let mut guard = self.inner.write();
        guard.models.clear();
        guard.components.clear();
    }

    /// Returns the number of cached model file snapshots.
    #[cfg(feature = "diagnostics")]
    #[must_use]
    pub fn model_count(&self) -> usize {
        self.inner.read().models.len()
    }

    /// Returns the number of cached component file snapshots.
    #[cfg(feature = "diagnostics")]
    #[must_use]
    pub fn component_count(&self) -> usize {
        self.inner.read().components.len()
    }
}
