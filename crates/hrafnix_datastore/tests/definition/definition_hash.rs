use hrafnix_datastore::definition::{
    BooleanDefinition, ChoiceDefinition, ChoiceItemDefinition, FileDefinition, FolderDefinition,
    GlobalObjectDefinition, IntegerConstraint, IntegerDefinition, ItemDefinitionType,
    MapDefinition, MapItemDefault, MapItemDefinition, NumberConstraint, NumberDefinition,
    NumberWithUnitsDefinition, ParameterObjectDefinition, SeparatorDefinition, StringDefinition,
    TabDefinition, TableColumnRequirement, TableDefinition, TableWithUnitsDefinition,
    UnitDefinition, VariableObjectDefinition,
};
use hrafnix_keys::{parameter_key, store_key};
use hrafnix_shareable_string::SharedStringStore;
use hrafnix_units::{UnitFamilyId, UnitId};
use std::collections::BTreeMap;

#[test]
fn test_scalar_definition_hashes() {
    let store = SharedStringStore::new();

    // Boolean
    let mut b1 = BooleanDefinition::new_with_descriptions_and_default("Desc", "Yes", "No", "true");
    let b2 = BooleanDefinition::new_with_descriptions_and_default("Desc", "Yes", "No", "true");
    assert_ne!(b1.hash(), [0u8; 32]);
    assert_eq!(b1.hash(), b2.hash());
    let original = b1.clone();
    b1.launder(&store);
    assert_eq!(b1, original);
    assert_eq!(b1.hash(), original.hash());
    assert_ne!(
        b1.hash(),
        BooleanDefinition::new_with_descriptions_and_default("Other", "Yes", "No", "true").hash()
    );
    assert_ne!(
        b1.hash(),
        BooleanDefinition::new_with_descriptions_and_default("Desc", "On", "No", "true").hash()
    );
    assert_ne!(
        b1.hash(),
        BooleanDefinition::new_with_descriptions_and_default("Desc", "Yes", "Off", "true").hash()
    );
    assert_ne!(
        b1.hash(),
        BooleanDefinition::new_with_descriptions_and_default("Desc", "Yes", "No", "false").hash()
    );

    // Choice & ChoiceItem
    let mut ci1 = ChoiceItemDefinition::new(store_key!("a"), "Alpha");
    let ci2 = ChoiceItemDefinition::new(store_key!("b"), "Beta");
    assert_ne!(ci1.hash(), [0u8; 32]);
    assert_eq!(
        ci1.hash(),
        ChoiceItemDefinition::new(store_key!("a"), "Alpha").hash()
    );
    let original = ci1.clone();
    ci1.launder(&store);
    assert_eq!(ci1, original);
    assert_eq!(ci1.hash(), original.hash());
    assert_ne!(
        ci1.hash(),
        ChoiceItemDefinition::new(store_key!("a"), "Other").hash()
    );
    assert_ne!(ci1.hash(), ci2.hash());

    let mut c1 = ChoiceDefinition::new_with_default("Pick", vec![ci1.clone(), ci2.clone()], "a");
    let c2 = ChoiceDefinition::new_with_default("Pick", vec![ci1.clone(), ci2.clone()], "a");
    let c_reordered = ChoiceDefinition::new_with_default("Pick", vec![ci2, ci1], "a");
    assert_ne!(c1.hash(), [0u8; 32]);
    assert_eq!(c1.hash(), c2.hash());
    let original = c1.clone();
    c1.launder(&store);
    assert_eq!(c1, original);
    assert_eq!(c1.hash(), original.hash());
    assert_ne!(c1.hash(), c_reordered.hash());
    assert_ne!(
        c1.hash(),
        ChoiceDefinition::new_with_default("Pick", c1.choices().to_vec(), "b").hash()
    );

    // File
    let mut f1 = FileDefinition::new_with_default("File", "*.csv", true, "a.csv");
    assert_ne!(f1.hash(), [0u8; 32]);
    let original = f1.clone();
    f1.launder(&store);
    assert_eq!(f1, original);
    assert_eq!(f1.hash(), original.hash());
    assert_ne!(
        f1.hash(),
        FileDefinition::new_with_default("File", "*.txt", true, "a.csv").hash()
    );
    assert_ne!(
        f1.hash(),
        FileDefinition::new_with_default("File", "*.csv", false, "a.csv").hash()
    );
    assert_ne!(
        f1.hash(),
        FileDefinition::new_with_default("File", "*.csv", true, "b.csv").hash()
    );

    // Folder
    let mut dir1 = FolderDefinition::new_with_default("Dir", true, "/tmp");
    assert_ne!(dir1.hash(), [0u8; 32]);
    let original = dir1.clone();
    dir1.launder(&store);
    assert_eq!(dir1, original);
    assert_eq!(dir1.hash(), original.hash());
    assert_ne!(
        dir1.hash(),
        FolderDefinition::new_with_default("Dir", false, "/tmp").hash()
    );
    assert_ne!(
        dir1.hash(),
        FolderDefinition::new_with_default("Dir", true, "/var").hash()
    );

    // Integer
    let mut i1 = IntegerDefinition::new_with_constraint_and_default(
        "Count",
        IntegerConstraint::range(0, 10, true, false),
        "5",
    );
    assert_ne!(i1.hash(), [0u8; 32]);
    let original = i1.clone();
    i1.launder(&store);
    assert_eq!(i1, original);
    assert_eq!(i1.hash(), original.hash());
    assert_ne!(
        i1.hash(),
        IntegerDefinition::new_with_constraint_and_default(
            "Count",
            IntegerConstraint::range(0, 10, true, true),
            "5",
        )
        .hash()
    );
    assert_ne!(
        i1.hash(),
        IntegerDefinition::new_with_constraint_and_default(
            "Count",
            IntegerConstraint::min(0, true),
            "5",
        )
        .hash()
    );

    // Number (including -0.0 == +0.0 canonicalization)
    let n_pos_zero = NumberDefinition::new_with_constraint("Val", NumberConstraint::min(0.0, true));
    let n_neg_zero =
        NumberDefinition::new_with_constraint("Val", NumberConstraint::min(-0.0, true));
    assert_eq!(n_pos_zero, n_neg_zero);
    assert_eq!(n_pos_zero.hash(), n_neg_zero.hash());
    assert_ne!(
        n_pos_zero.hash(),
        NumberDefinition::new_with_constraint("Val", NumberConstraint::min(0.0, false)).hash()
    );

    // NumberWithUnits
    let mut nwu1 = NumberWithUnitsDefinition::new_with_constraint_and_default(
        "Len",
        NumberConstraint::max(100.0, true),
        "10",
        UnitId::Length_Meter,
    );
    assert_ne!(nwu1.hash(), [0u8; 32]);
    let original = nwu1.clone();
    nwu1.launder(&store);
    assert_eq!(nwu1, original);
    assert_eq!(nwu1.hash(), original.hash());
    assert_ne!(
        nwu1.hash(),
        NumberWithUnitsDefinition::new_with_constraint_and_default(
            "Len",
            NumberConstraint::max(100.0, true),
            "10",
            UnitId::Length_Foot,
        )
        .hash()
    );

    // Unit
    let mut u1 = UnitDefinition::new_with_default("Unit", UnitFamilyId::Length, "m");
    assert_ne!(u1.hash(), [0u8; 32]);
    let original = u1.clone();
    u1.launder(&store);
    assert_eq!(u1, original);
    assert_eq!(u1.hash(), original.hash());
    assert_ne!(
        u1.hash(),
        UnitDefinition::new_with_default("Unit", UnitFamilyId::Time, "m").hash()
    );

    // String, Tab, Separator + domain separation
    let mut s1 = StringDefinition::new("Same");
    let mut tab1 = TabDefinition::new("Same");
    let mut sep1 = SeparatorDefinition::new("Same");
    let num1 = NumberDefinition::new("Same");
    let int1 = IntegerDefinition::new("Same");
    assert_ne!(s1.hash(), tab1.hash());
    assert_ne!(tab1.hash(), sep1.hash());
    assert_ne!(s1.hash(), num1.hash());
    assert_ne!(num1.hash(), int1.hash());
    let original = s1.clone();
    s1.launder(&store);
    assert_eq!(s1, original);
    assert_eq!(s1.hash(), original.hash());
    let original = tab1.clone();
    tab1.launder(&store);
    assert_eq!(tab1, original);
    assert_eq!(tab1.hash(), original.hash());
    let original = sep1.clone();
    sep1.launder(&store);
    assert_eq!(sep1, original);
    assert_eq!(sep1.hash(), original.hash());
}

#[test]
fn test_table_and_map_definition_hashes() {
    let store = SharedStringStore::new();

    // TableDefinition
    let mut t_no_default = TableDefinition::new_with_column_requirements(
        "Table",
        vec![
            (
                store_key!("x"),
                NumberDefinition::new("X"),
                TableColumnRequirement::Increasing,
            ),
            (
                store_key!("y"),
                NumberDefinition::new("Y"),
                TableColumnRequirement::None,
            ),
        ],
    );
    let t_with_default = TableDefinition::new_with_column_requirements_and_default(
        "Table",
        vec![
            (
                store_key!("x"),
                NumberDefinition::new("X"),
                TableColumnRequirement::Increasing,
            ),
            (
                store_key!("y"),
                NumberDefinition::new("Y"),
                TableColumnRequirement::None,
            ),
        ],
        vec![vec!["1", "2"]],
    );
    let t_diff_req = TableDefinition::new_with_column_requirements(
        "Table",
        vec![
            (
                store_key!("x"),
                NumberDefinition::new("X"),
                TableColumnRequirement::Decreasing,
            ),
            (
                store_key!("y"),
                NumberDefinition::new("Y"),
                TableColumnRequirement::None,
            ),
        ],
    );
    let t_reordered = TableDefinition::new_with_column_requirements(
        "Table",
        vec![
            (
                store_key!("y"),
                NumberDefinition::new("Y"),
                TableColumnRequirement::None,
            ),
            (
                store_key!("x"),
                NumberDefinition::new("X"),
                TableColumnRequirement::Increasing,
            ),
        ],
    );

    assert_ne!(t_no_default.hash(), [0u8; 32]);
    let original = t_no_default.clone();
    t_no_default.launder(&store);
    assert_eq!(t_no_default, original);
    assert_eq!(t_no_default.hash(), original.hash());
    assert_ne!(t_no_default.hash(), t_with_default.hash());
    assert_ne!(t_no_default.hash(), t_diff_req.hash());
    assert_ne!(t_no_default.hash(), t_reordered.hash());

    // TableWithUnitsDefinition
    let mut twu_no_default = TableWithUnitsDefinition::new_with_column_requirements(
        "Table",
        vec![(
            store_key!("x"),
            NumberWithUnitsDefinition::new("X", UnitId::Length_Meter),
            TableColumnRequirement::Increasing,
        )],
    );
    let twu_with_default = TableWithUnitsDefinition::new_with_column_requirements_and_default(
        "Table",
        vec![(
            store_key!("x"),
            NumberWithUnitsDefinition::new("X", UnitId::Length_Meter),
            TableColumnRequirement::Increasing,
        )],
        vec![vec!["1"]],
    );
    assert_ne!(twu_no_default.hash(), [0u8; 32]);
    let original = twu_no_default.clone();
    twu_no_default.launder(&store);
    assert_eq!(twu_no_default, original);
    assert_eq!(twu_no_default.hash(), original.hash());
    assert_ne!(twu_no_default.hash(), twu_with_default.hash());

    // MapDefinition
    let mut m_no_default = MapDefinition::new(
        "Map",
        vec![
            (
                store_key!("a"),
                MapItemDefinition::String(StringDefinition::new("A")),
            ),
            (
                store_key!("b"),
                MapItemDefinition::Number(NumberDefinition::new("B")),
            ),
        ],
    );
    let mut defaults = BTreeMap::new();
    defaults.insert(
        store_key!("entry1").into(),
        vec![MapItemDefault::scalar("hi"), MapItemDefault::scalar("42")],
    );
    let m_with_default = MapDefinition::new_with_default(
        "Map",
        vec![
            (
                store_key!("a"),
                MapItemDefinition::String(StringDefinition::new("A")),
            ),
            (
                store_key!("b"),
                MapItemDefinition::Number(NumberDefinition::new("B")),
            ),
        ],
        defaults,
    );
    assert_ne!(m_no_default.hash(), [0u8; 32]);
    let original = m_no_default.clone();
    m_no_default.launder(&store);
    assert_eq!(m_no_default, original);
    assert_eq!(m_no_default.hash(), original.hash());
    assert_ne!(m_no_default.hash(), m_with_default.hash());

    let map_item = MapItemDefinition::String(StringDefinition::new("A"));
    assert_eq!(map_item.hash(), StringDefinition::new("A").hash());
}

#[test]
fn test_object_definition_hashes_and_defaults() {
    let store = SharedStringStore::new();

    // Default matches empty builder().finish()
    let g_default = GlobalObjectDefinition::default();
    let g_empty = GlobalObjectDefinition::builder().finish();
    assert_eq!(g_default, g_empty);
    assert_eq!(g_default.hash(), g_empty.hash());
    assert_ne!(g_default.hash(), [0u8; 32]);

    let p_default = ParameterObjectDefinition::default();
    let p_empty = ParameterObjectDefinition::builder().finish();
    assert_eq!(p_default, p_empty);
    assert_eq!(p_default.hash(), p_empty.hash());
    assert_ne!(p_default.hash(), [0u8; 32]);

    let v_default = VariableObjectDefinition::default();
    let v_empty = VariableObjectDefinition::builder().finish();
    assert_eq!(v_default, v_empty);
    assert_eq!(v_default.hash(), v_empty.hash());
    assert_ne!(v_default.hash(), [0u8; 32]);

    // Domain separation across object kinds
    assert_ne!(g_default.hash(), p_default.hash());
    assert_ne!(p_default.hash(), v_default.hash());
    assert_ne!(g_default.hash(), v_default.hash());

    // Key order affects hash
    let mut p_ab = ParameterObjectDefinition::builder()
        .with(parameter_key!("p_a"), StringDefinition::new("A"))
        .with(parameter_key!("p_b"), StringDefinition::new("B"))
        .finish();
    let p_ba = ParameterObjectDefinition::builder()
        .with(parameter_key!("p_b"), StringDefinition::new("B"))
        .with(parameter_key!("p_a"), StringDefinition::new("A"))
        .finish();
    assert_ne!(p_ab.hash(), p_ba.hash());
    let original = p_ab.clone();
    p_ab.launder(&store);
    assert_eq!(p_ab, original);
    assert_eq!(p_ab.hash(), original.hash());

    // ItemDefinitionType delegates hash
    let str_def = StringDefinition::new("Test");
    let item_def = ItemDefinitionType::String(str_def.clone());
    assert_eq!(item_def.hash(), str_def.hash());
}
