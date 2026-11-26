use hrafnix_datastore::definition::{FolderDefinition, SeparatorDefinition, TabDefinition};
use hrafnix_datastore::prelude::*;
use hrafnix_units::{UnitFamilyId, UnitId};
use std::collections::BTreeMap;

macro_rules! assert_stable_laundering {
    ($value:expr, $store:expr) => {{
        let value = &mut $value;
        let original = value.clone();
        let original_debug = format!("{value:?}");
        let original_hash = value.hash();
        value.launder($store);
        assert_eq!(*value, original);
        assert_eq!(format!("{value:?}"), original_debug);
        assert_eq!(value.hash(), original_hash);
        value.launder($store);
        assert_eq!(format!("{value:?}"), original_debug);
    }};
}

fn table_definition() -> TableDefinition {
    TableDefinition::new_with_column_requirements_and_default(
        "Table".to_owned(),
        vec![
            (
                store_key!("z"),
                NumberDefinition::new_with_default("Last".to_owned(), "7".to_owned()),
                TableColumnRequirement::Increasing,
            ),
            (
                store_key!("a"),
                NumberDefinition::new_with_default("First".to_owned(), "8".to_owned()),
                TableColumnRequirement::Decreasing,
            ),
        ],
        vec![vec!["11".to_owned(), "12".to_owned()]],
    )
}

fn unit_table_definition() -> TableWithUnitsDefinition {
    TableWithUnitsDefinition::new_with_column_requirements_and_default(
        "Unit table".to_owned(),
        vec![(
            store_key!("length"),
            NumberWithUnitsDefinition::new_with_default(
                "Length".to_owned(),
                "3".to_owned(),
                UnitId::Length_Meter,
            ),
            TableColumnRequirement::Increasing,
        )],
        vec![vec!["13".to_owned()]],
    )
}

fn map_definition() -> MapDefinition {
    let items = vec![
        (
            store_key!("z"),
            MapItemDefinition::Boolean(BooleanDefinition::new_with_descriptions_and_default(
                "Boolean", "On", "Off", "true",
            )),
        ),
        (
            store_key!("choice"),
            ChoiceDefinition::new_with_default(
                "Choice",
                vec![ChoiceItemDefinition::new(
                    store_key!("selected"),
                    "Selected",
                )],
                "selected",
            )
            .into(),
        ),
        (
            store_key!("file"),
            FileDefinition::new_with_default("File", "*.csv", true, "data.csv").into(),
        ),
        (
            store_key!("integer"),
            IntegerDefinition::new_with_default("Integer", "2").into(),
        ),
        (
            store_key!("number"),
            NumberDefinition::new_with_default("Number", "3").into(),
        ),
        (
            store_key!("number_with_units"),
            MapItemDefinition::NumberWithUnits(NumberWithUnitsDefinition::new_with_default(
                "Length",
                "4",
                UnitId::Length_Meter,
            )),
        ),
        (
            store_key!("string"),
            StringDefinition::new_with_default("String", "text").into(),
        ),
        (store_key!("table"), table_definition().into()),
        (store_key!("unit_table"), unit_table_definition().into()),
        (
            store_key!("a"),
            UnitDefinition::new_with_default("Unit", UnitFamilyId::Length, "length_meter").into(),
        ),
    ];
    let defaults = BTreeMap::from([(
        StoreKey::new("entry".to_owned().into()).unwrap(),
        vec![
            MapItemDefault::scalar("false".to_owned()),
            MapItemDefault::scalar("selected".to_owned()),
            MapItemDefault::scalar("other.csv".to_owned()),
            MapItemDefault::scalar("5".to_owned()),
            MapItemDefault::scalar("6".to_owned()),
            MapItemDefault::scalar("7".to_owned()),
            MapItemDefault::scalar("entry text".to_owned()),
            MapItemDefault::table(vec![vec!["21".to_owned(), "22".to_owned()]]),
            MapItemDefault::table(vec![vec!["23".to_owned()]]),
            MapItemDefault::scalar("length_centimeter".to_owned()),
        ],
    )]);
    MapDefinition::new_with_default("Map".to_owned(), items, defaults)
}

#[test]
fn all_definition_and_frozen_variants_launder_recursively() {
    let store = SharedStringStore::new();
    let definitions: Vec<ItemDefinitionType> = vec![
        BooleanDefinition::new_with_descriptions_and_default("Boolean", "On", "Off", "true").into(),
        ChoiceDefinition::new_with_default(
            "Choice",
            vec![ChoiceItemDefinition::new(
                store_key!("selected"),
                "Selected",
            )],
            "selected",
        )
        .into(),
        FileDefinition::new_with_default("File", "*.csv", true, "data.csv").into(),
        FolderDefinition::new_with_default("Folder", true, "folder").into(),
        IntegerDefinition::new_with_default("Integer", "2").into(),
        map_definition().into(),
        NumberDefinition::new_with_default("Number", "3").into(),
        NumberWithUnitsDefinition::new_with_default("Length", "4", UnitId::Length_Meter).into(),
        StringDefinition::new_with_default("String", "text").into(),
        table_definition().into(),
        unit_table_definition().into(),
        UnitDefinition::new_with_default("Unit", UnitFamilyId::Length, "length_meter").into(),
        TabDefinition::new("Tab").into(),
        SeparatorDefinition::new("Separator").into(),
    ];
    let mut global = GlobalObjectDefinition::builder();
    let mut parameter = ParameterObjectDefinition::builder();
    let mut variable = VariableObjectDefinition::builder();
    for (index, definition) in definitions.into_iter().enumerate() {
        global.insert(
            GlobalKey::new(format!("g_item_{index}").into()).unwrap(),
            definition.clone(),
        );
        parameter.insert(
            ParameterKey::new(format!("p_item_{index}").into()).unwrap(),
            definition.clone(),
        );
        variable.insert(
            VariableKey::new(format!("v_item_{index}").into()).unwrap(),
            definition,
        );
    }
    let mut global = global.finish();
    let mut parameter = parameter.finish();
    let mut variable = variable.finish();
    let mut global_frozen = GlobalObjectFrozen::new(global.clone());
    let mut parameter_frozen = ParameterObjectFrozen::new(parameter.clone());
    let mut variable_frozen = VariableObjectFrozen::new(variable.clone());
    assert_stable_laundering!(global, &store);
    assert_stable_laundering!(parameter, &store);
    assert_stable_laundering!(variable, &store);
    assert_stable_laundering!(global_frozen, &store);
    assert_stable_laundering!(parameter_frozen, &store);
    assert_stable_laundering!(variable_frozen, &store);
    assert_eq!(global_frozen.thaw().freeze(), global_frozen);
    assert_eq!(parameter_frozen.thaw().freeze(), parameter_frozen);
    assert_eq!(variable_frozen.thaw().freeze(), variable_frozen);
}

#[test]
fn shared_table_columns_use_copy_on_write() {
    let store = SharedStringStore::new();
    let interned = store.launder("Last".to_owned());
    let mut table = table_definition();
    let original = table.clone();
    let original_column = original.iter().next().unwrap().1;
    let original_string_ptr = original_column.description_ref().as_ref().as_ptr();
    assert_ne!(original_string_ptr, interned.as_ref().as_ptr());
    let rows_ptr = table.default_table().unwrap().as_ptr();
    let row_ptr = table.default_table().unwrap()[0].as_ptr();

    assert_stable_laundering!(table, &store);

    let column = table.iter().next().unwrap().1;
    assert_eq!(
        column.description_ref().as_ref().as_ptr(),
        interned.as_ref().as_ptr()
    );
    assert_eq!(
        original_column.description_ref().as_ref().as_ptr(),
        original_string_ptr
    );
    assert!(!std::ptr::eq(column, original_column));
    assert_eq!(table.default_table().unwrap().as_ptr(), rows_ptr);
    assert_eq!(table.default_table().unwrap()[0].as_ptr(), row_ptr);
    assert_eq!(
        table
            .iter()
            .map(|(key, _)| key.as_str())
            .collect::<Vec<_>>(),
        vec!["z", "a"]
    );
    for (key, requirement) in table.column_requirements() {
        assert_eq!(Some(requirement), table.column_requirement(key.as_str()));
    }

    let interned = store.launder("Length".to_owned());
    let mut table = unit_table_definition();
    let original = table.clone();
    let original_column = original.iter().next().unwrap().1;
    let original_string_ptr = original_column.description_ref().as_ref().as_ptr();
    assert_stable_laundering!(table, &store);
    let column = table.iter().next().unwrap().1;
    assert_eq!(
        column.description_ref().as_ref().as_ptr(),
        interned.as_ref().as_ptr()
    );
    assert_eq!(
        original_column.description_ref().as_ref().as_ptr(),
        original_string_ptr
    );
    assert!(!std::ptr::eq(column, original_column));
}

#[test]
fn map_defaults_and_frozen_entries_are_interned() {
    let store = SharedStringStore::new();
    let interned_key = store.launder("entry".to_owned());
    let interned_scalar = store.launder("entry text".to_owned());
    let interned_cell = store.launder("21".to_owned());
    let mut definition = map_definition();
    let original = definition.clone();
    assert!(std::ptr::eq(definition.item_type(), original.item_type()));
    assert_stable_laundering!(definition, &store);
    assert!(!std::ptr::eq(definition.item_type(), original.item_type()));
    let (key, defaults) = definition.default_map().unwrap().first_key_value().unwrap();
    assert_eq!(key.as_str().as_ptr(), interned_key.as_ref().as_ptr());
    let MapItemDefault::Scalar(value) = &defaults[6] else {
        panic!("expected scalar default");
    };
    assert_eq!(value.as_ref().as_ptr(), interned_scalar.as_ref().as_ptr());
    let MapItemDefault::Table(rows) = &defaults[7] else {
        panic!("expected table default");
    };
    assert_eq!(
        rows[0][0].as_ref().as_ptr(),
        interned_cell.as_ref().as_ptr()
    );

    let mut frozen = MapFrozen::new(original);
    assert_stable_laundering!(frozen, &store);
    let (key, entry) = frozen.iter().next().unwrap();
    assert_eq!(key.as_str().as_ptr(), interned_key.as_ref().as_ptr());
    let item = entry
        .iter()
        .find(|(key, _)| key.as_str() == "string")
        .unwrap()
        .1;
    let MapItemFrozen::String(value) = item else {
        panic!("expected frozen string");
    };
    assert_eq!(
        value.value().as_ref().as_ptr(),
        interned_scalar.as_ref().as_ptr()
    );
    let item = entry
        .iter()
        .find(|(key, _)| key.as_str() == "table")
        .unwrap()
        .1;
    let MapItemFrozen::Table(table) = item else {
        panic!("expected frozen table");
    };
    assert_eq!(
        table.rows()[0][0].as_ref().as_ptr(),
        interned_cell.as_ref().as_ptr()
    );
    assert_eq!(frozen.thaw().freeze(), frozen);
}

#[test]
fn frozen_tables_preserve_buffers_and_intern_parameter_units_and_cells() {
    let store = SharedStringStore::new();
    let parameter = store.launder("p_source".to_owned());
    let cell = store.launder("31".to_owned());
    let units = store.launder("length_centimeter".to_owned());
    let mut table = TableFrozen::new_from_rows_with_parameter(
        table_definition(),
        vec![vec!["31".to_owned().into(), "32".to_owned().into()]],
        "p_source".to_owned(),
    );
    let rows_ptr = table.rows().as_ptr();
    let row_ptr = table.rows()[0].as_ptr();
    assert_stable_laundering!(table, &store);
    assert_eq!(table.rows().as_ptr(), rows_ptr);
    assert_eq!(table.rows()[0].as_ptr(), row_ptr);
    assert_eq!(
        table.parameter().as_ref().as_ptr(),
        parameter.as_ref().as_ptr()
    );
    assert_eq!(table.rows()[0][0].as_ref().as_ptr(), cell.as_ref().as_ptr());

    let mut table = TableWithUnitsFrozen::new_from_rows_with_parameter(
        unit_table_definition(),
        vec![vec!["31".to_owned().into()]],
        vec!["length_centimeter".to_owned().into()],
        "p_source".to_owned(),
    );
    let rows_ptr = table.rows().as_ptr();
    let row_ptr = table.rows()[0].as_ptr();
    let units_ptr = table.units().as_ptr();
    assert_stable_laundering!(table, &store);
    assert_eq!(table.rows().as_ptr(), rows_ptr);
    assert_eq!(table.rows()[0].as_ptr(), row_ptr);
    assert_eq!(table.units().as_ptr(), units_ptr);
    assert_eq!(
        table.parameter().as_ref().as_ptr(),
        parameter.as_ref().as_ptr()
    );
    assert_eq!(table.rows()[0][0].as_ref().as_ptr(), cell.as_ref().as_ptr());
    assert_eq!(table.units()[0].as_ref().as_ptr(), units.as_ref().as_ptr());
}
