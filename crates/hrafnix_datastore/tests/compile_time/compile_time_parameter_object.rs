use hrafnix_datastore::prelude::*;

#[test]
fn parameter_object_compile_time_supports_empty_objects() {
    const FROM_LITERALS: ParameterObjectCompileTime = const_parameter_object!([]);
    const FROM_SLICE: ParameterObjectCompileTime = const_parameter_object!(&[],);

    assert_eq!(FROM_LITERALS, FROM_SLICE);
    assert_eq!(FROM_SLICE.count(), 0);
    assert_eq!(
        FROM_SLICE.into_definition(),
        ParameterObjectDefinition::default()
    );
}

#[test]
fn parameter_object_compile_time_converts_both_macro_forms() {
    const ITEMS: &[(ConstParameterKey, ItemCompileTime)] = &[(
        parameter_key!("p_name"),
        const_item!(string = const_string!("Name")),
    )];
    const FROM_SLICE: ParameterObjectCompileTime = const_parameter_object!(ITEMS);
    const FROM_LITERALS: ParameterObjectCompileTime = const_parameter_object!([(
        "p_enabled",
        const_item!(boolean = const_boolean!("Enabled")),
    )],);

    assert_eq!(FROM_SLICE.items(), ITEMS);
    assert_eq!(FROM_SLICE.count(), 1);
    assert!(FROM_SLICE.contains("p_name"));
    assert!(!FROM_SLICE.contains("p_missing"));
    assert!(matches!(
        FROM_SLICE.get("p_name"),
        Some(ItemCompileTime::String(_))
    ));
    assert_eq!(FROM_SLICE.get("p_missing"), None);
    assert_eq!(FROM_SLICE.iter().count(), 1);
    assert_eq!(
        FROM_SLICE
            .keys()
            .map(|key| key.to_string())
            .collect::<Vec<_>>(),
        ["p_name"]
    );
    assert_eq!(
        FROM_LITERALS
            .into_definition()
            .keys()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        ["p_enabled"]
    );
}
