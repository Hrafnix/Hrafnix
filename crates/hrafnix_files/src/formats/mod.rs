//! Public file-format reading and writing.

/// Component file reading and writing.
pub mod component;
/// Model file reading and writing.
pub mod model;

#[cfg(test)]
mod tests {
    use super::component::write_component_file;
    use super::model::write_model_file;
    use crate::data::component::ComponentFile;
    use crate::data::component_instance::ComponentInstance;
    use crate::data::graphics::{Dimension, Position, Rotation};
    use crate::data::metadata_general::MetadataFile;
    use crate::data::metadata_gui::GuiMetadata;
    use crate::data::model::ModelFile;
    use crate::data::source::ComponentSource;
    use crate::data::translations::TranslationsFile;
    use camino::Utf8PathBuf;
    use hrafnix_datastore::definition::{
        GlobalObjectDefinition, ParameterObjectDefinition, VariableObjectDefinition,
    };
    use hrafnix_datastore::frozen::{GlobalObjectFrozen, ParameterObjectFrozen};
    use hrafnix_keys::component_key;
    use hrafnix_message::message::{MessageCategory, MessageLevel};
    use std::collections::BTreeMap;
    use std::fs;
    use std::io::ErrorKind;
    use std::sync::atomic::{AtomicUsize, Ordering};

    /// Owns only the directory and document created by the preflight test.
    #[derive(Debug)]
    struct PreflightDirectory {
        /// Unique directory relative to the test's working directory.
        path: Utf8PathBuf,
    }

    impl PreflightDirectory {
        /// Creates a unique local directory without replacing an existing path.
        fn new() -> Self {
            static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
            loop {
                let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
                let path = Utf8PathBuf::from(format!(
                    ".hrafnix-files-preflight-test-{}-{id}",
                    std::process::id()
                ));
                match fs::create_dir(&path) {
                    Ok(()) => return Self { path },
                    Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                    Err(error) => panic!("Failed to create test directory '{path}': {error}"),
                }
            }
        }
    }

    impl Drop for PreflightDirectory {
        fn drop(&mut self) {
            drop(fs::remove_file(self.path.join("duplicate-ids.json")));
            drop(fs::remove_dir(&self.path));
        }
    }

    #[test]
    fn writing_duplicate_instance_ids_preserves_the_existing_document() {
        let directory = PreflightDirectory::new();
        let path = directory.path.join("duplicate-ids.json");
        let original = b"{\"existing\":\"document\"}\n";
        for root in ["model", "component"] {
            fs::write(&path, original).unwrap();
            let metadata = MetadataFile::new("".into(), "".into(), "".into(), 0, 0);
            let source = ComponentSource::Builtin {
                name: "builtin.clock".into(),
                version: 1,
            };
            let gui_metadata = GuiMetadata::new(
                Dimension::new(0, 0),
                Rotation::Degrees0,
                "".into(),
                Vec::new(),
            );
            // Public readers reject duplicates, so construct this invalid snapshot privately.
            let instance = ComponentInstance::new(
                component_key!("temperature_sensor").into(),
                source.clone(),
                Position::new(0, 0),
                gui_metadata.clone(),
                ParameterObjectFrozen::new(ParameterObjectDefinition::default()),
            );
            let instances = vec![instance.clone(), instance];
            let translations = TranslationsFile::new("en".into(), BTreeMap::new());
            let error = if root == "model" {
                write_model_file(
                    ModelFile::new(
                        metadata,
                        Vec::new(),
                        Vec::new(),
                        GlobalObjectFrozen::new(GlobalObjectDefinition::default()),
                        instances,
                        Vec::new(),
                        translations,
                        ParameterObjectDefinition::default(),
                        VariableObjectDefinition::default(),
                    ),
                    &path,
                )
            } else {
                write_component_file(
                    ComponentFile::new(
                        metadata,
                        source,
                        gui_metadata,
                        Vec::new(),
                        Vec::new(),
                        instances,
                        Vec::new(),
                        translations,
                        ParameterObjectDefinition::default(),
                        VariableObjectDefinition::default(),
                    ),
                    &path,
                )
            }
            .unwrap_err();

            assert_eq!(error.level(), MessageLevel::Error);
            assert_eq!(error.category(), MessageCategory::File);
            assert_eq!(
                error.translate_data().message_key().as_str(),
                "files_duplicate_component_id"
            );
            assert_eq!(error.translate_data().message_params().len(), 1);
            assert_eq!(
                error.translate_data().message_params()["key"].as_str(),
                "temperature_sensor"
            );
            assert!(error.extra_detail().is_none());
            assert_eq!(fs::read(&path).unwrap(), original);
            let entries: Vec<_> = fs::read_dir(&directory.path)
                .unwrap()
                .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
                .collect();
            assert_eq!(entries, ["duplicate-ids.json"]);
        }
    }
}
