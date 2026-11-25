//! Integration tests for `hrafnix_design` objects and file caching.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::arithmetic_side_effects
)]

use camino::Utf8PathBuf;
use hrafnix_common::port::PortType;
use hrafnix_datastore::definition::{ParameterObjectDefinition, VariableObjectDefinition};
use hrafnix_datastore::editable::ItemEditable;
use hrafnix_datastore::frozen::ItemFrozen;
use hrafnix_design::built_in_component::BuiltInComponent;
use hrafnix_design::project::Project;
use hrafnix_design::types::{
    BackgroundDecoration, BackgroundDecorationKind, ComponentSource, Connection, ConnectionLine,
    Dimension, FontSize, ForegroundDecoration, ForegroundDecorationKind, GuiMetadata, Metadata,
    PackageReference, PackageVersion, Port, PortLocation, Position, Rotation, TextDecoration,
};
use hrafnix_files::formats::component::read_component_file;
use hrafnix_files::formats::model::read_model_file;
use hrafnix_files::formats::project::read_project_file;
use hrafnix_keys::{ComponentKey, PortKey};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

fn files_fixture_dir() -> Utf8PathBuf {
    let manifest_dir = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    manifest_dir.join("../hrafnix_files/tests/data")
}

fn unique_temp_dir(label: &str) -> Utf8PathBuf {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let id = COUNTER.fetch_add(1, Ordering::Relaxed);
    let path = Utf8PathBuf::from_path_buf(std::env::temp_dir())
        .expect("temp dir should be valid UTF-8")
        .join(format!(
            "hrafnix_design_{label}_{}_{id}",
            std::process::id()
        ));
    std::fs::create_dir_all(&path).expect("temp dir should be created");
    path
}

fn create_project_layout(root: &Utf8PathBuf) {
    std::fs::create_dir_all(root.join(".hrafnix")).expect("create .hrafnix");
    std::fs::create_dir_all(root.join("data")).expect("create data");
    std::fs::create_dir_all(root.join("components")).expect("create components");
    std::fs::create_dir_all(root.join("models")).expect("create models");
}

#[test]
fn in_memory_projects_do_not_load_or_create_manifest_files() {
    let root = unique_temp_dir("in_memory");
    let missing = root.join("not_created");
    let project = Project::open_project(&missing);
    assert_eq!(project.name().as_str(), "");
    assert_eq!(project.version().as_str(), "");
    assert!(!missing.exists());
    assert!(project.save().is_err());
    assert!(!missing.exists());

    std::fs::write(root.join("hrafnix.toml"), "not a valid manifest").unwrap();
    let project = Project::open_project(&root);
    assert_eq!(project.name().as_str(), "");
    assert_eq!(project.version().as_str(), "");
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn creating_a_project_builds_its_layout_and_persists_metadata() {
    let root = unique_temp_dir("create_project");
    let project_root = root.join("nested").join("demo");
    let project = Project::create_project(&project_root, "Demo Project", "1.2.3").unwrap();
    for path in [
        project.dependencies_dir(),
        project.data_dir(),
        project.components_dir(),
        project.models_dir(),
    ] {
        assert!(path.is_dir(), "{path}");
    }
    assert_eq!(project.working_directory(), project_root.as_path());
    let file = read_project_file(&project.manifest_path()).unwrap();
    assert_eq!(file.name().as_str(), "Demo Project");
    assert_eq!(file.version().as_str(), "1.2.3");
    assert_eq!(file, project.to_project_file());
    let loaded = Project::load_project(&project_root).unwrap();
    assert_eq!(loaded.name().as_str(), "Demo Project");
    assert_eq!(loaded.version().as_str(), "1.2.3");

    let model = loaded.create_model("demo.hxm", Metadata::new("Demo", "", "", 1, 0));
    model.save().unwrap();
    assert!(loaded.models_dir().join("demo.hxm").is_file());
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn project_save_and_reload_share_metadata_without_resetting_open_models_or_cache() {
    let root = unique_temp_dir("project_reload");
    let project = Project::create_project(&root, "Initial", "1.0.0").unwrap();
    let cloned = project.clone();
    let model = project.create_model("demo.hxm", Metadata::new("Initial Model", "", "", 1, 0));
    model.save().unwrap();
    let cached = project.file_cache().get_model(&model.file_path()).unwrap();
    model.with_metadata_mut(|metadata| metadata.set_display_name("Unsaved Model"));
    project.set_name("Quoted \"Project\"\n\u{e9}");
    project.set_version("development");
    let snapshot = project.to_project_file();
    cloned.save().unwrap();
    assert_eq!(
        read_project_file(&project.manifest_path()).unwrap(),
        snapshot
    );
    assert_eq!(
        read_model_file(&model.file_path())
            .unwrap()
            .metadata()
            .display_name()
            .as_str(),
        "Initial Model"
    );

    project.set_name("Discarded");
    project.set_version("Discarded");
    cloned.reload().unwrap();
    assert_eq!(project.to_project_file(), snapshot);
    assert_eq!(model.metadata().display_name().as_str(), "Unsaved Model");
    assert!(Arc::ptr_eq(
        &cached,
        &project.file_cache().get_model(&model.file_path()).unwrap()
    ));
    assert!(std::ptr::eq(
        project.name().as_str().as_ptr(),
        project
            .shared_string_store()
            .get(snapshot.name().as_str())
            .as_str()
            .as_ptr(),
    ));
    assert!(std::ptr::eq(
        project.version().as_str().as_ptr(),
        project
            .shared_string_store()
            .get("development")
            .as_str()
            .as_ptr(),
    ));
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn project_loading_and_reload_failures_are_localized_and_keep_existing_metadata() {
    let root = unique_temp_dir("project_errors");
    let project = Project::open_project(&root);
    project.set_name("Keep Name");
    project.set_version("Keep Version");
    let expected = project.to_project_file();
    let missing = Project::load_project(&root).unwrap_err();
    assert_eq!(
        missing.translate_data().message_key().as_str(),
        "files_io_not_found"
    );

    for text in [
        "",
        "[project]\nname = 'Missing version'\n",
        "not valid TOML",
    ] {
        std::fs::write(project.manifest_path(), text).unwrap();
        let error = project.reload().unwrap_err();
        assert_eq!(
            error.translate_data().message_key().as_str(),
            "files_toml_data"
        );
        assert!(
            error
                .translated_message(project.translation_map(), "en")
                .unwrap()
                .as_str()
                .contains(project.manifest_path().as_str())
        );
        assert_eq!(project.to_project_file(), expected);
        assert!(Project::load_project(&root).is_err());
    }
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn project_creation_preserves_existing_data_and_refuses_existing_manifests() {
    let root = unique_temp_dir("project_preserve");
    std::fs::write(root.join("notes.txt"), "keep").unwrap();
    let project = Project::create_project(&root, "Original", "1").unwrap();
    let manifest = std::fs::read(project.manifest_path()).unwrap();
    let error = Project::create_project(&root, "Replacement", "2").unwrap_err();
    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_already_exists"
    );
    assert_eq!(std::fs::read(project.manifest_path()).unwrap(), manifest);
    assert_eq!(
        std::fs::read_to_string(root.join("notes.txt")).unwrap(),
        "keep"
    );

    let blocked = root.join("blocked");
    std::fs::create_dir(&blocked).unwrap();
    std::fs::write(blocked.join("hrafnix.toml"), "existing invalid manifest").unwrap();
    assert!(Project::create_project(&blocked, "Replacement", "2").is_err());
    assert!(!blocked.join(".hrafnix").exists());
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn directory_creation_failures_do_not_publish_a_manifest_or_remove_existing_files() {
    let root = unique_temp_dir("project_directory_failure");
    std::fs::write(root.join("data"), "not a directory").unwrap();
    let error = Project::create_project(&root, "Demo", "1").unwrap_err();
    assert_eq!(
        error.translate_data().message_key().as_str(),
        "files_io_error"
    );
    assert!(error.extra_detail().is_some());
    assert!(!root.join("hrafnix.toml").exists());
    assert_eq!(
        std::fs::read_to_string(root.join("data")).unwrap(),
        "not a directory"
    );
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn loading_a_manifest_does_not_create_missing_project_directories() {
    let root = unique_temp_dir("project_load_only");
    std::fs::write(
        root.join("hrafnix.toml"),
        "[project]\nname = 'Demo'\nversion = '1'\n",
    )
    .unwrap();
    let project = Project::load_project(&root).unwrap();
    assert_eq!(project.name().as_str(), "Demo");
    assert!(!project.components_dir().exists());
    assert!(!project.models_dir().exists());
    assert!(!project.dependencies_dir().exists());
    assert!(!project.data_dir().exists());
    std::fs::remove_dir_all(&root).unwrap();
}

#[test]
fn project_exposes_standard_directory_layout_and_shares_metadata() {
    let root = unique_temp_dir("layout");
    let project = Project::open_project(&root);
    let cloned = project.clone();

    assert_eq!(project.dependencies_dir(), root.join(".hrafnix"));
    assert_eq!(project.data_dir(), root.join("data"));
    assert_eq!(project.components_dir(), root.join("components"));
    assert_eq!(project.models_dir(), root.join("models"));
    assert_eq!(project.manifest_path(), root.join("hrafnix.toml"));

    assert_eq!(project.name().as_str(), "");
    assert_eq!(project.version().as_str(), "");

    project.set_name("Demo Project");
    project.set_version("1.2.3");

    assert_eq!(cloned.name().as_str(), "Demo Project");
    assert_eq!(cloned.version().as_str(), "1.2.3");

    std::fs::remove_dir_all(&root).expect("temp dir should be removed");
}

#[test]
fn opening_model_resolves_in_models_dir_and_uses_file_cache() {
    let root = unique_temp_dir("model_cache");
    create_project_layout(&root);
    std::fs::copy(
        files_fixture_dir().join("minimal.hxm"),
        root.join("models").join("minimal.hxm"),
    )
    .expect("copy minimal.hxm");

    let project = Project::open_project(&root);

    let model_a = project
        .open_model("minimal.hxm")
        .expect("model should open");
    assert_eq!(model_a.file_path(), root.join("models").join("minimal.hxm"));

    let cached_first = project
        .file_cache()
        .get_model(&model_a.file_path())
        .expect("model should be cached");

    let model_b = project
        .open_model("minimal.hxm")
        .expect("model should open from cache");
    let cached_second = project
        .file_cache()
        .get_model(&model_b.file_path())
        .expect("model should remain cached");

    assert!(Arc::ptr_eq(&cached_first, &cached_second));
    assert_eq!(model_a.source_hash(), cached_first.file_hash());
    assert_eq!(model_a.metadata().display_name().as_str(), "Example");

    #[cfg(feature = "diagnostics")]
    assert_eq!(project.file_cache().model_count(), 1);

    let removed = project.file_cache().invalidate_model(&model_a.file_path());
    assert!(removed.is_some());
    assert!(
        project
            .file_cache()
            .get_model(&model_a.file_path())
            .is_none()
    );

    std::fs::remove_dir_all(&root).expect("temp dir should be removed");
}

#[test]
fn opening_user_component_resolves_in_components_dir_and_respects_package_provenance() {
    let root = unique_temp_dir("comp_cache");
    create_project_layout(&root);
    std::fs::copy(
        files_fixture_dir().join("minimal.hxc"),
        root.join("components").join("minimal.hxc"),
    )
    .expect("copy minimal.hxc");

    let project = Project::open_project(&root);

    let loose = project
        .open_user_component("minimal.hxc", None)
        .expect("loose component should open");
    assert_eq!(
        loose.file_path(),
        root.join("components").join("minimal.hxc")
    );

    let pkg_ref = PackageReference::new("hrafnix.demo", PackageVersion::new(1, 0, 0));
    let packaged = project
        .open_user_component("minimal.hxc", Some(&pkg_ref))
        .expect("packaged component should open");

    assert_ne!(loose.source_hash(), packaged.source_hash());
    assert_ne!(loose.interface_hash(), packaged.interface_hash());
    assert!(matches!(
        packaged.source(),
        ComponentSource::File {
            package: Some(_),
            ..
        }
    ));

    let file_pkg_ref = hrafnix_files::data::source::PackageReference::new(
        project.shared_string_store().get("hrafnix.demo"),
        hrafnix_files::data::source::PackageVersion::new(1, 0, 0),
    );
    let cached_loose = project
        .file_cache()
        .get_component(&loose.file_path(), None)
        .expect("loose component should be cached");
    let cached_packaged = project
        .file_cache()
        .get_component(&packaged.file_path(), Some(&file_pkg_ref))
        .expect("packaged component should be cached");
    assert!(!Arc::ptr_eq(&cached_loose, &cached_packaged));

    #[cfg(feature = "diagnostics")]
    assert_eq!(project.file_cache().component_count(), 2);

    project.file_cache().clear();
    assert!(
        project
            .file_cache()
            .get_component(&loose.file_path(), None)
            .is_none()
    );

    std::fs::remove_dir_all(&root).expect("temp dir should be removed");
}

#[test]
fn model_and_user_component_round_trip_edit_and_save_back_to_files() {
    let root = unique_temp_dir("round_trip");
    create_project_layout(&root);
    let model_path = root.join("models").join("demo.hxm");
    let comp_path = root.join("components").join("sub.hxc");

    let initial_model_json = r#"{
  "file-version": "1",
  "model": {
    "metadata": {
      "display-name": "Initial",
      "description": "Before edit.",
      "author": "Hrafnix",
      "minimum-version-major": 1,
      "minimum-version-minor": 0
    },
    "parameter_definition": [],
    "variable_definition": [],
    "foreground_decorations": [],
    "background_decorations": [],
    "global_settings": [],
    "components": [
      {
        "id": "clock",
        "source": {
          "type": "builtin",
          "name": "builtin.clock",
          "version": 1
        },
        "position": { "x": 10, "y": 20 },
        "gui_metadata": {
          "dimensions": { "width": 64, "height": 64 },
          "rotation": 0,
          "icon": "icons/clock.svg",
          "ports": []
        },
        "parameters": [
          {
            "key": "p_label",
            "type": "string",
            "definition": {
              "description": "Clock label",
              "default": "default"
            },
            "value": "initial"
          }
        ]
      }
    ],
    "connections": [],
    "translations": {
      "language": "en",
      "translations": {}
    }
  }
}"#;
    std::fs::write(&model_path, initial_model_json).expect("write model");
    std::fs::copy(files_fixture_dir().join("minimal.hxc"), &comp_path).expect("copy component");

    let project = Project::open_project(&root);

    // Edit Model and save back to disk
    let model = project.open_model("demo.hxm").expect("model should open");
    let original_model_hash = model.source_hash();
    model.with_metadata_mut(|meta| {
        meta.set_display_name("Edited Model");
    });
    let key = ComponentKey::new(project.shared_string_store().get("clock")).expect("valid key");
    let instance = model.component(&key).expect("component should exist");
    instance.set_position(Position::new(100, 200));

    // Thaw frozen parameters, edit an item, and merge back into frozen state
    let original_params_hash = instance.parameters().hash();
    let mut editable_params = instance.thaw_parameters();
    let Some(ItemEditable::String(label)) = editable_params.get_mut("p_label") else {
        panic!("expected string parameter 'p_label'");
    };
    label.set("updated");
    instance.merge_parameters(&editable_params);
    assert_ne!(instance.parameters().hash(), original_params_hash);

    // Verify loaded and edited strings are interned in the project's SharedStringStore
    let interned_name = project.shared_string_store().get("Edited Model");
    assert!(std::ptr::eq(
        model.metadata().display_name().as_str().as_ptr(),
        interned_name.as_str().as_ptr()
    ));
    let interned_param_value = project.shared_string_store().get("updated");
    let merged_params = instance.parameters();
    let Some(ItemFrozen::String(merged_label)) = merged_params.get("p_label") else {
        panic!("expected merged string parameter 'p_label'");
    };
    assert!(std::ptr::eq(
        merged_label.value().as_str().as_ptr(),
        interned_param_value.as_str().as_ptr()
    ));

    model.save().expect("model save should succeed");
    assert_ne!(model.source_hash(), original_model_hash);

    let reloaded_model_file = read_model_file(&model_path).expect("saved model should read back");
    assert_eq!(reloaded_model_file.file_hash(), model.source_hash());
    assert_eq!(
        reloaded_model_file.metadata().display_name().as_str(),
        "Edited Model"
    );
    assert_eq!(reloaded_model_file.components()[0].position().x(), 100);
    assert_eq!(reloaded_model_file.components()[0].position().y(), 200);
    let Some(ItemFrozen::String(saved_label)) = reloaded_model_file.components()[0]
        .parameters()
        .get("p_label")
    else {
        panic!("expected saved string parameter 'p_label'");
    };
    assert_eq!(saved_label.value().as_str(), "updated");

    // Edit UserComponent and save back to disk
    let comp = project
        .open_user_component("sub.hxc", None)
        .expect("component should open");
    let original_comp_hash = comp.source_hash();
    let original_iface_hash = comp.interface_hash();
    comp.with_metadata_mut(|meta| {
        meta.set_display_name("Edited Component");
    });
    let mut gui = comp.gui_metadata();
    gui.set_rotation(Rotation::Degrees90);
    comp.set_gui_metadata(gui);

    comp.save().expect("component save should succeed");
    assert_ne!(comp.source_hash(), original_comp_hash);
    assert_ne!(comp.interface_hash(), original_iface_hash);

    let reloaded_comp_file =
        read_component_file(&comp_path, None).expect("saved component should read back");
    assert_eq!(reloaded_comp_file.file_hash(), comp.source_hash());
    assert_eq!(reloaded_comp_file.interface_hash(), comp.interface_hash());
    assert_eq!(
        reloaded_comp_file.metadata().display_name().as_str(),
        "Edited Component"
    );
    assert_eq!(
        reloaded_comp_file.gui_metadata().rotation(),
        hrafnix_files::data::graphics::Rotation::Degrees90
    );

    std::fs::remove_dir_all(&root).expect("temp dir should be removed");
}

#[test]
fn create_models_user_components_instantiate_built_ins_and_mutate_collections() {
    let root = unique_temp_dir("create_and_mutate");
    create_project_layout(&root);
    let project = Project::open_project(&root);

    let port_out = PortKey::new(project.shared_string_store().get("out")).expect("valid port");
    let port_in = PortKey::new(project.shared_string_store().get("in")).expect("valid port");

    // Define a built-in component descriptor
    let built_in_clock = BuiltInComponent::new(
        "builtin.clock",
        1,
        Metadata::new("Clock", "Built-in clock", "Hrafnix", 1, 0),
        GuiMetadata::new(
            Dimension::new(64, 64),
            Rotation::Degrees0,
            Utf8PathBuf::from("icons/clock.svg"),
            vec![Port::new(
                port_out.clone(),
                PortType::Output,
                PortLocation::new(64, 32),
            )],
        ),
        ParameterObjectDefinition::default(),
        VariableObjectDefinition::default(),
    );

    // Create a brand-new UserComponent from scratch and save it
    let sub_comp = project.create_user_component(
        "filter.hxc",
        Metadata::new("Filter", "Custom filter", "Author", 1, 0),
        GuiMetadata::new(
            Dimension::new(80, 48),
            Rotation::Degrees0,
            Utf8PathBuf::from("icons/filter.svg"),
            vec![Port::new(
                port_in.clone(),
                PortType::Input,
                PortLocation::new(0, 24),
            )],
        ),
    );
    sub_comp.save().expect("new user component should save");
    assert!(root.join("components").join("filter.hxc").exists());

    // Create a brand-new Model from scratch
    let model = project.create_model(
        "created.hxm",
        Metadata::new("Created Model", "Built programmatically", "Author", 1, 0),
    );

    let clock_id = ComponentKey::new(project.shared_string_store().get("clock1")).expect("key");
    let filter_id = ComponentKey::new(project.shared_string_store().get("filter1")).expect("key");

    let clock_inst =
        project.instantiate_built_in(&built_in_clock, clock_id.clone(), Position::new(10, 20));
    let filter_inst = sub_comp.instantiate(filter_id.clone(), Position::new(150, 20));

    assert!(model.add_component(clock_inst.clone()));
    assert!(!model.add_component(clock_inst)); // duplicate ID rejected
    assert!(model.add_component(filter_inst));
    assert_eq!(model.components().len(), 2);

    // Add connection and decorations
    model.add_connection(Connection::new(
        clock_id,
        port_out,
        filter_id.clone(),
        port_in,
        ConnectionLine::new(vec![Position::new(80, 52)]),
    ));
    assert_eq!(model.connections().len(), 1);

    model.add_foreground_decoration(ForegroundDecoration::new(
        "title",
        Position::new(0, 0),
        Dimension::new(200, 30),
        ForegroundDecorationKind::Text(TextDecoration::new("Main Loop", FontSize::new(1400))),
    ));
    model.add_background_decoration(BackgroundDecoration::new(
        "frame1",
        Position::new(0, 0),
        Dimension::new(300, 200),
        BackgroundDecorationKind::Frame,
    ));
    model.add_background_decoration(BackgroundDecoration::new(
        "frame_tmp",
        Position::new(5, 5),
        Dimension::new(50, 50),
        BackgroundDecorationKind::Frame,
    ));
    assert!(model.remove_background_decoration("frame_tmp").is_some());
    assert_eq!(model.background_decorations().len(), 1);

    model.save().expect("new model should save");

    let saved = read_model_file(&root.join("models").join("created.hxm"))
        .expect("saved model should read back");
    assert_eq!(saved.components().len(), 2);
    assert_eq!(saved.connections().len(), 1);
    assert_eq!(saved.foreground_decorations().len(), 1);
    assert_eq!(saved.background_decorations().len(), 1);

    // Verify component removal
    assert!(model.remove_component(&filter_id).is_some());
    assert!(model.remove_component(&filter_id).is_none());
    assert_eq!(model.components().len(), 1);

    std::fs::remove_dir_all(&root).expect("temp dir should be removed");
}
