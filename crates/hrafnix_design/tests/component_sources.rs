//! Regression tests for source resolution and compatible interface refresh.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::indexing_slicing,
    clippy::panic,
    clippy::arithmetic_side_effects
)]

use camino::{Utf8Path, Utf8PathBuf};
use hrafnix_common::port::PortType;
use hrafnix_datastore::definition::{
    ChoiceDefinition, ChoiceItemDefinition, IntegerConstraint, IntegerDefinition,
    ParameterObjectDefinition, ParameterObjectDefinitionBuilder, StringDefinition,
    VariableObjectDefinition, VariableObjectDefinitionBuilder,
};
use hrafnix_datastore::editable::ItemEditable;
use hrafnix_datastore::frozen::ItemFrozen;
use hrafnix_design::built_in_component::BuiltInComponent;
use hrafnix_design::project::Project;
use hrafnix_design::types::{
    ComponentSource, Connection, ConnectionLine, Dimension, GuiMetadata, Metadata,
    PackageReference, PackageVersion, Port, PortLocation, Position, Rotation,
};
use hrafnix_design::user_component::UserComponent;
use hrafnix_files::formats::component::write_component_file;
use hrafnix_files::formats::model::read_model_file;
use hrafnix_keys::{
    ComponentKey, ParameterKey, component_key, parameter_key, port_key, store_key, variable_key,
};
use hrafnix_message::message::{Message, MessageCategory, MessageLevel};
use std::fs;
use std::io::ErrorKind;
use std::sync::atomic::{AtomicU64, Ordering};

/// An isolated project whose files are removed after its test finishes.
#[derive(Debug)]
struct TestProject {
    /// Root owned by this test.
    root: Utf8PathBuf,
    /// Project context and shared resources.
    project: Project,
}

impl TestProject {
    /// Creates a unique project using the standard directory layout.
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        loop {
            let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
            let root = Utf8PathBuf::try_from(std::env::temp_dir())
                .unwrap()
                .join(format!(
                    "hrafnix-design-sources-{}-{id}",
                    std::process::id(),
                ));
            match fs::create_dir(&root) {
                Ok(()) => {
                    let project = Project::create_project(&root, "Sources", "1").unwrap();
                    return Self { root, project };
                }
                Err(error) if error.kind() == ErrorKind::AlreadyExists => {}
                Err(error) => panic!("Failed to create test project '{root}': {error}"),
            }
        }
    }

    /// Creates a source file at an absolute path, including its parent directories.
    fn source(&self, path: &Utf8Path, name: &str) -> UserComponent {
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        let component = self.project.create_user_component(
            path,
            Metadata::new(name, "", "", 1, 0),
            gui(40, Rotation::Degrees0),
        );
        component.save().unwrap();
        component
    }
}

impl Drop for TestProject {
    fn drop(&mut self) {
        drop(fs::remove_dir_all(&self.root));
    }
}

/// A GUI with one output port for testing dimensions, rotation, and port updates.
fn gui(width: u16, rotation: Rotation) -> GuiMetadata {
    GuiMetadata::new(
        Dimension::new(width, 20),
        rotation,
        "icons/device.svg".into(),
        vec![Port::new(
            port_key!("out").into(),
            PortType::Output,
            PortLocation::new(width, 10),
        )],
    )
}

/// A descriptor for an exact version of the test built-in.
fn built_in(version: u32, width: u16) -> BuiltInComponent {
    BuiltInComponent::new(
        "builtin.device",
        version,
        Metadata::new("Built-in", "", "", 1, 0),
        gui(width, Rotation::Degrees0),
        ParameterObjectDefinition::default(),
        VariableObjectDefinition::default(),
    )
}

/// A file source retaining the caller's exact path and package provenance.
fn file_source(path: &Utf8Path, package: Option<PackageReference>) -> ComponentSource {
    ComponentSource::File {
        package,
        path: path.to_path_buf(),
    }
}

/// Checks error classification and complete localization in every supported language.
fn assert_error(project: &Project, error: &Message, key: &str) {
    assert_eq!(error.level(), MessageLevel::Error);
    assert_eq!(error.category(), MessageCategory::Component);
    assert_eq!(error.translate_data().message_key().as_str(), key);
    for language in ["en", "zh", "de", "es", "fr", "ja", "ko"] {
        let translated = error
            .translated_message(project.translation_map(), language)
            .unwrap();
        assert!(!translated.as_str().contains("%{"));
        for value in error.translate_data().message_params().values() {
            assert!(translated.as_str().contains(value.as_str()));
        }
    }
}

#[test]
fn built_ins_resolve_exact_versions_and_keep_registered_snapshots_immutable() {
    let test = TestProject::new();
    let clone = test.project.clone();
    assert!(test.project.register_built_in(built_in(1, 40)).is_none());
    assert!(test.project.register_built_in(built_in(2, 60)).is_none());
    let source = ComponentSource::Builtin {
        name: "builtin.device".into(),
        version: 1,
    };
    let original = clone
        .resolve_component_source(&source, Utf8Path::new(""))
        .unwrap();
    assert!(original.file().is_none());
    assert_eq!(original.built_in().unwrap().version(), 1);
    assert_eq!(original.gui_metadata().dimensions().width(), 40);
    assert_eq!(original.source(), &source);
    assert!(original.parameter_definition().keys().next().is_none());
    assert_eq!(
        original.variable_definition(),
        &VariableObjectDefinition::default()
    );
    assert_eq!(original.metadata().display_name().as_str(), "Built-in");

    assert!(clone.register_built_in(built_in(1, 80)).is_some());
    let replacement = test
        .project
        .reload_component_source(&source, Utf8Path::new(""))
        .unwrap();
    assert_eq!(replacement.gui_metadata().dimensions().width(), 80);
    assert_eq!(original.gui_metadata().dimensions().width(), 40);
    for source in [
        ComponentSource::Builtin {
            name: "builtin.device".into(),
            version: 3,
        },
        ComponentSource::Builtin {
            name: "missing".into(),
            version: 1,
        },
    ] {
        let error = test
            .project
            .resolve_component_source(&source, Utf8Path::new(""))
            .unwrap_err();
        assert_error(&test.project, &error, "design_builtin_not_found");
    }

    let id: ComponentKey = component_key!("device").into();
    let instance =
        original.instantiate(test.project.shared_string_store(), id, Position::new(7, 8));
    assert!(instance.is_interface_stale(&replacement).unwrap());
    let report = instance.refresh_interface(&replacement).unwrap();
    assert!(report.changed());
    assert!(!instance.is_interface_stale(&replacement).unwrap());
    assert_eq!(instance.position(), Position::new(7, 8));
    assert_eq!(instance.gui_metadata().dimensions().width(), 80);
}

#[test]
fn loose_relative_sources_use_the_containing_file_directory_and_preserve_provenance() {
    let test = TestProject::new();
    let first = test.root.join("models/one/device.hxc");
    let second = test.root.join("models/two/device.hxc");
    test.source(&first, "First");
    test.source(&second, "Second");
    test.source(
        &test.root.join("components/device.hxc"),
        "Wrong project-level fallback",
    );
    let source = file_source(Utf8Path::new("device.hxc"), None);
    for (containing, name, expected_path) in [
        (test.root.join("models/one/model.hxm"), "First", first),
        (test.root.join("models/two/model.hxm"), "Second", second),
    ] {
        let resolved = test
            .project
            .resolve_component_source(&source, &containing)
            .unwrap();
        assert_eq!(resolved.source(), &source);
        assert_eq!(resolved.metadata().display_name().as_str(), name);
        assert_eq!(
            resolved.file().unwrap().source(),
            &hrafnix_files::data::source::ComponentSource::File {
                package: None,
                path: expected_path,
            }
        );
    }
    let missing = test
        .project
        .resolve_component_source(&source, &test.root.join("models/absent/model.hxm"))
        .unwrap_err();
    assert_eq!(
        missing.translate_data().message_key().as_str(),
        "files_io_not_found"
    );

    let absolute = file_source(&test.root.join("components/device.hxc"), None);
    let resolved = test
        .project
        .resolve_component_source(&absolute, Utf8Path::new(""))
        .unwrap();
    assert_eq!(resolved.source(), &absolute);
    let invalid = test
        .project
        .resolve_component_source(&source, Utf8Path::new(""))
        .unwrap_err();
    assert_eq!(
        invalid.translate_data().message_key().as_str(),
        "files_io_invalid_input"
    );
}

#[test]
fn package_roots_resolve_exact_versions_without_inventing_a_package_layout() {
    let test = TestProject::new();
    let first = test.root.join(".hrafnix/first/components/device.hxc");
    let second = test.root.join(".hrafnix/second/components/device.hxc");
    test.source(&first, "Version one");
    test.source(&second, "Version two");
    let package_one = PackageReference::new("demo.package", PackageVersion::new(1, 0, 0));
    let package_two = PackageReference::new("demo.package", PackageVersion::new(2, 0, 0));
    let source_one = file_source(
        Utf8Path::new("components/device.hxc"),
        Some(package_one.clone()),
    );
    let source_two = file_source(
        Utf8Path::new("components/device.hxc"),
        Some(package_two.clone()),
    );
    let anchor = test.root.join("models/model.hxm");
    let error = test
        .project
        .resolve_component_source(&source_one, &anchor)
        .unwrap_err();
    assert_error(&test.project, &error, "design_package_not_registered");
    assert!(
        test.project
            .register_package_root(package_one.clone(), ".hrafnix/first")
            .is_none()
    );
    assert!(
        test.project
            .clone()
            .register_package_root(package_two.clone(), test.root.join(".hrafnix/second"))
            .is_none()
    );
    for (source, name, package) in [
        (&source_one, "Version one", &package_one),
        (&source_two, "Version two", &package_two),
    ] {
        let resolved = test
            .project
            .resolve_component_source(source, &anchor)
            .unwrap();
        assert_eq!(resolved.metadata().display_name().as_str(), name);
        assert_eq!(resolved.source(), source);
        let hrafnix_files::data::source::ComponentSource::File {
            package: Some(actual),
            ..
        } = resolved.file().unwrap().source()
        else {
            panic!("expected package provenance");
        };
        assert_eq!(actual.name().as_str(), package.name().as_str());
        assert_eq!(actual.version().major(), package.version().major());
    }
    assert_eq!(
        test.project
            .register_package_root(package_one, test.root.join(".hrafnix/second")),
        Some(test.root.join(".hrafnix/first")),
    );
    assert_eq!(
        test.project
            .resolve_component_source(&source_one, &anchor)
            .unwrap()
            .metadata()
            .display_name()
            .as_str(),
        "Version two",
    );

    let absolute = file_source(
        &first,
        Some(PackageReference::new(
            "unregistered",
            PackageVersion::new(9, 8, 7),
        )),
    );
    assert!(
        test.project
            .resolve_component_source(&absolute, Utf8Path::new(""))
            .is_ok()
    );
    test.project.reload().unwrap();
    assert!(
        test.project
            .resolve_component_source(&source_two, &anchor)
            .is_ok()
    );
}

#[test]
fn cached_resolution_and_explicit_reload_keep_old_snapshots_and_failures_intact() {
    let test = TestProject::new();
    let path = test.root.join("components/device.hxc");
    let component = test.source(&path, "Initial");
    let source = file_source(&path, None);
    let initial = test
        .project
        .resolve_component_source(&source, Utf8Path::new(""))
        .unwrap();
    component.with_metadata_mut(|metadata| metadata.set_display_name("External edit"));
    write_component_file(component.to_component_file(), &path).unwrap();
    let cached = test
        .project
        .resolve_component_source(&source, Utf8Path::new(""))
        .unwrap();
    assert!(std::ptr::eq(
        initial.file().unwrap(),
        cached.file().unwrap()
    ));
    assert_eq!(cached.metadata().display_name().as_str(), "Initial");
    let reloaded = test
        .project
        .reload_component_source(&source, Utf8Path::new(""))
        .unwrap();
    assert_eq!(reloaded.metadata().display_name().as_str(), "External edit");
    assert_eq!(initial.metadata().display_name().as_str(), "Initial");
    assert!(!std::ptr::eq(
        initial.file().unwrap(),
        reloaded.file().unwrap()
    ));

    fs::write(&path, "invalid JSON").unwrap();
    assert!(
        test.project
            .reload_component_source(&source, Utf8Path::new(""))
            .is_err()
    );
    let retained = test
        .project
        .resolve_component_source(&source, Utf8Path::new(""))
        .unwrap();
    assert!(std::ptr::eq(
        reloaded.file().unwrap(),
        retained.file().unwrap()
    ));
}

/// The original schema includes a compatible scalar, a changed type, and a removed key.
fn original_parameters() -> ParameterObjectDefinition {
    ParameterObjectDefinitionBuilder::new()
        .with(
            parameter_key!("p_label"),
            StringDefinition::new_with_default("Old description", "old default"),
        )
        .with(
            parameter_key!("p_reset"),
            StringDefinition::new_with_default("Old type", "old value"),
        )
        .with(
            parameter_key!("p_remove"),
            StringDefinition::new_with_default("Removed", "discard"),
        )
        .with(
            parameter_key!("p_counter"),
            IntegerDefinition::new_with_default("Counter", "10"),
        )
        .with(
            parameter_key!("p_choice"),
            ChoiceDefinition::new_with_default(
                "Choice",
                vec![ChoiceItemDefinition::new(store_key!("old"), "Old")],
                "old",
            ),
        )
        .finish()
}

/// A reordered schema changes descriptions, defaults, constraints, a type, and choice IDs.
fn refreshed_parameters() -> ParameterObjectDefinition {
    ParameterObjectDefinitionBuilder::new()
        .with(
            parameter_key!("p_add"),
            StringDefinition::new_with_default("Added", "new default"),
        )
        .with(
            parameter_key!("p_counter"),
            IntegerDefinition::new_with_constraint_and_default(
                "Updated counter",
                IntegerConstraint::min(0, true),
                "20",
            ),
        )
        .with(
            parameter_key!("p_reset"),
            IntegerDefinition::new_with_default("New type", "42"),
        )
        .with(
            parameter_key!("p_label"),
            StringDefinition::new_with_default("New description", "replacement default"),
        )
        .with(
            parameter_key!("p_choice"),
            ChoiceDefinition::new_with_default(
                "Updated choice",
                vec![ChoiceItemDefinition::new(store_key!("new"), "New")],
                "new",
            ),
        )
        .finish()
}

#[test]
fn model_refresh_preserves_compatible_expressions_rotation_connections_and_hash_domain() {
    let test = TestProject::new();
    let path = test.root.join("components/device.hxc");
    let component = test.source(&path, "Device");
    component.set_parameter_definition(original_parameters());
    component.save().unwrap();
    let source = file_source(Utf8Path::new("../components/device.hxc"), None);
    let model = test
        .project
        .create_model("model.hxm", Metadata::new("Model", "", "", 1, 0));
    let resolved = test
        .project
        .resolve_component_source(&source, &model.file_path())
        .unwrap();
    let id: ComponentKey = component_key!("device").into();
    let instance = resolved.instantiate(
        test.project.shared_string_store(),
        id.clone(),
        Position::new(-20, 50),
    );
    let mut placed_gui = instance.gui_metadata();
    placed_gui.set_rotation(Rotation::Degrees90);
    instance.set_gui_metadata(placed_gui);
    instance.edit_parameters(|parameters| {
        let Some(ItemEditable::String(label)) = parameters.get_mut("p_label") else {
            panic!("expected string");
        };
        label.set("user label");
        let Some(ItemEditable::Integer(counter)) = parameters.get_mut("p_counter") else {
            panic!("expected integer");
        };
        counter.set("p_base + 1");
    });
    assert!(!instance.is_interface_stale(&resolved).unwrap());
    assert_ne!(
        instance.saved_interface_hash(),
        resolved.file().unwrap().interface_hash()
    );
    assert!(model.add_component(instance.clone()));
    let connection = Connection::new(
        id.clone(),
        port_key!("out").into(),
        id.clone(),
        port_key!("out").into(),
        ConnectionLine::new(Vec::new()),
    );
    model.add_connection(connection.clone());
    let previous_hash = instance.saved_interface_hash();

    let new_definition = refreshed_parameters();
    component.set_parameter_definition(new_definition.clone());
    let mut updated_gui = gui(80, Rotation::Degrees180);
    updated_gui.ports_mut().clear();
    updated_gui.ports_mut().push(Port::new(
        port_key!("renamed").into(),
        PortType::Output,
        PortLocation::new(80, 10),
    ));
    component.set_gui_metadata(updated_gui);
    component.save().unwrap();
    assert!(model.is_component_interface_stale(&id).unwrap());
    assert_eq!(instance.saved_interface_hash(), previous_hash);
    let report = model.refresh_component(&id).unwrap();
    assert!(report.changed());
    assert_eq!(report.added_parameters(), &[parameter_key!("p_add")]);
    assert_eq!(report.removed_parameters(), &[parameter_key!("p_remove")]);
    assert_eq!(
        report.reset_parameters(),
        &[parameter_key!("p_reset"), parameter_key!("p_choice")]
    );
    assert_eq!(instance.parameters().definition(), &new_definition);
    assert_eq!(
        instance
            .parameters()
            .definition()
            .keys()
            .map(ParameterKey::as_str)
            .collect::<Vec<_>>(),
        ["p_add", "p_counter", "p_reset", "p_label", "p_choice"]
    );
    assert_eq!(instance.gui_metadata().rotation(), Rotation::Degrees90);
    assert_eq!(instance.gui_metadata().dimensions().width(), 80);
    assert_eq!(instance.position(), Position::new(-20, 50));
    assert_eq!(instance.id(), id);
    assert_eq!(instance.source(), source);
    assert_eq!(model.connections(), [connection]);
    let parameters = instance.parameters();
    let Some(ItemFrozen::String(label)) = parameters.get("p_label") else {
        panic!("expected label");
    };
    assert_eq!(label.value().as_str(), "user label");
    assert_eq!(
        label.definition().description_ref().as_str(),
        "New description"
    );
    let Some(ItemFrozen::Integer(counter)) = parameters.get("p_counter") else {
        panic!("expected counter");
    };
    assert_eq!(counter.value().as_str(), "p_base + 1");
    let Some(ItemFrozen::Integer(reset)) = parameters.get("p_reset") else {
        panic!("expected reset");
    };
    assert_eq!(reset.value().as_str(), "42");
    let Some(ItemFrozen::String(added)) = parameters.get("p_add") else {
        panic!("expected added");
    };
    assert_eq!(added.value().as_str(), "new default");
    let Some(ItemFrozen::Choice(choice)) = parameters.get("p_choice") else {
        panic!("expected choice");
    };
    assert_eq!(choice.value().as_str(), "new");
    assert!(parameters.get("p_remove").is_none());
    for actual in [
        label.value(),
        label.definition().description(),
        added.value(),
        counter.value(),
    ] {
        let interned = test.project.shared_string_store().get(actual.as_str());
        assert!(std::ptr::eq(
            actual.as_str().as_ptr(),
            interned.as_str().as_ptr()
        ));
    }
    assert!(!model.is_component_interface_stale(&id).unwrap());
    let repeated = model.refresh_component(&id).unwrap();
    assert!(!repeated.changed());
    let empty: &[ParameterKey] = &[];
    assert_eq!(repeated.added_parameters(), empty);
    assert_eq!(repeated.removed_parameters(), empty);
    assert_eq!(repeated.reset_parameters(), empty);
    model.save().unwrap();
    let saved = read_model_file(&model.file_path()).unwrap();
    assert_eq!(
        saved.components()[0].interface_hash(),
        instance.saved_interface_hash()
    );
    assert_eq!(saved.components()[0].parameters(), &parameters);
    assert_eq!(
        test.project
            .open_model("model.hxm")
            .unwrap()
            .component(&id)
            .unwrap()
            .saved_interface_hash(),
        instance.saved_interface_hash()
    );
}

#[test]
fn metadata_variable_definition_and_source_rotation_changes_do_not_make_instances_stale() {
    let test = TestProject::new();
    let path = test.root.join("components/device.hxc");
    let component = test.source(&path, "Initial");
    let source = file_source(&path, None);
    let initial = test
        .project
        .resolve_component_source(&source, Utf8Path::new(""))
        .unwrap();
    let instance = initial.instantiate(
        test.project.shared_string_store(),
        component_key!("device").into(),
        Position::new(0, 0),
    );
    component.with_metadata_mut(|metadata| metadata.set_display_name("Metadata-only edit"));
    component.set_variable_definition(
        VariableObjectDefinitionBuilder::new()
            .with(variable_key!("v_output"), StringDefinition::new("Output"))
            .finish(),
    );
    component.set_gui_metadata(gui(40, Rotation::Degrees270));
    component.save().unwrap();
    let updated = test
        .project
        .reload_component_source(&source, Utf8Path::new(""))
        .unwrap();
    assert_ne!(
        initial.file().unwrap().interface_hash(),
        updated.file().unwrap().interface_hash()
    );
    assert!(!instance.is_interface_stale(&updated).unwrap());
    assert!(!instance.refresh_interface(&updated).unwrap().changed());
    assert_eq!(instance.gui_metadata().rotation(), Rotation::Degrees0);
}

#[test]
fn nested_user_components_refresh_relative_sources_without_recursing_into_cycles() {
    let test = TestProject::new();
    let device = test.source(&test.root.join("components/device.hxc"), "Device");
    let parent = test.source(&test.root.join("components/parent.hxc"), "Parent");
    let source = file_source(Utf8Path::new("device.hxc"), None);
    let resolved = test
        .project
        .resolve_component_source(&source, &parent.file_path())
        .unwrap();
    let id: ComponentKey = component_key!("device").into();
    assert!(parent.add_component(resolved.instantiate(
        test.project.shared_string_store(),
        id.clone(),
        Position::new(5, 6)
    )));
    parent.save().unwrap();
    assert_eq!(
        parent
            .resolve_component(&id)
            .unwrap()
            .metadata()
            .display_name()
            .as_str(),
        "Device"
    );
    device.set_gui_metadata(gui(70, Rotation::Degrees180));
    device.save().unwrap();
    assert!(parent.is_component_interface_stale(&id).unwrap());
    assert!(parent.refresh_component(&id).unwrap().changed());
    assert_eq!(
        parent
            .component(&id)
            .unwrap()
            .gui_metadata()
            .dimensions()
            .width(),
        70
    );
    assert!(!parent.is_component_interface_stale(&id).unwrap());
    parent.save().unwrap();

    let self_source = file_source(Utf8Path::new("parent.hxc"), None);
    let resolved_self = test
        .project
        .resolve_component_source(&self_source, &parent.file_path())
        .unwrap();
    assert!(parent.add_component(resolved_self.instantiate(
        test.project.shared_string_store(),
        component_key!("self_reference").into(),
        Position::new(0, 0)
    )));
    parent.save().unwrap();
    let shallow = test
        .project
        .reload_component_source(&self_source, &parent.file_path())
        .unwrap();
    assert_eq!(shallow.file().unwrap().components().len(), 2);
}

#[test]
fn failed_refresh_and_mismatched_sources_leave_instance_state_unchanged() {
    let test = TestProject::new();
    let path = test.root.join("components/device.hxc");
    test.source(&path, "Device");
    let model = test
        .project
        .create_model("model.hxm", Metadata::new("Model", "", "", 1, 0));
    let source = file_source(&path, None);
    let resolved = test
        .project
        .resolve_component_source(&source, &model.file_path())
        .unwrap();
    let id: ComponentKey = component_key!("device").into();
    let instance = resolved.instantiate(
        test.project.shared_string_store(),
        id.clone(),
        Position::new(10, 20),
    );
    assert!(model.add_component(instance.clone()));
    let original = model.to_model_file();
    let hash = instance.saved_interface_hash();
    fs::write(&path, "invalid JSON").unwrap();
    assert!(model.refresh_component(&id).is_err());
    assert!(model.is_component_interface_stale(&id).is_err());
    assert_eq!(model.to_model_file(), original);
    assert_eq!(instance.saved_interface_hash(), hash);

    assert!(test.project.register_built_in(built_in(1, 100)).is_none());
    let other_source = ComponentSource::Builtin {
        name: "builtin.device".into(),
        version: 1,
    };
    let other = test
        .project
        .resolve_component_source(&other_source, &model.file_path())
        .unwrap();
    for error in [
        instance.is_interface_stale(&other).unwrap_err(),
        instance.refresh_interface(&other).unwrap_err(),
    ] {
        assert_error(&test.project, &error, "design_component_source_mismatch");
    }
    assert_eq!(model.to_model_file(), original);
    assert_eq!(instance.saved_interface_hash(), hash);

    let missing: ComponentKey = component_key!("missing").into();
    let parent = test.project.create_user_component(
        "parent.hxc",
        Metadata::new("Parent", "", "", 1, 0),
        gui(40, Rotation::Degrees0),
    );
    for error in [
        model.resolve_component(&missing).unwrap_err(),
        model.is_component_interface_stale(&missing).unwrap_err(),
        model.refresh_component(&missing).unwrap_err(),
        parent.resolve_component(&missing).unwrap_err(),
        parent.is_component_interface_stale(&missing).unwrap_err(),
        parent.refresh_component(&missing).unwrap_err(),
    ] {
        assert_error(&test.project, &error, "design_component_not_found");
    }
}
