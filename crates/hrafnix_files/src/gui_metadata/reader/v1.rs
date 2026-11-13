use crate::common::deserialize::deserialize_object;
use crate::graphics::reader::v1::DimensionReaderV1;
use crate::gui_metadata::data::GuiMetadata;
use serde::Deserialize;

/// The version 1 JSON representation of GUI metadata.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct GuiMetadataReaderV1 {
    /// Dimensions in drawing coordinates.
    #[serde(deserialize_with = "deserialize_object")]
    dimensions: DimensionReaderV1,
    /// Icon path, stored unchanged rather than opened or resolved during file I/O.
    icon: String,
}

impl From<GuiMetadataReaderV1> for GuiMetadata {
    fn from(metadata: GuiMetadataReaderV1) -> Self {
        Self {
            dimensions: metadata.dimensions.into(),
            icon: metadata.icon.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GuiMetadataReaderV1;
    use crate::common::deserialize::RootObject;
    use crate::gui_metadata::data::GuiMetadata;
    use serde_json::{Value, json};

    /// Reads GUI metadata through its object-only envelope.
    fn read(value: &Value) -> Result<GuiMetadata, serde_json::Error> {
        let reader: RootObject<GuiMetadataReaderV1> =
            serde_json::from_str(&serde_json::to_string(value)?)?;
        Ok(reader.document.into())
    }

    /// Creates GUI metadata with explicit dimensions and an unresolved icon path.
    fn metadata_json() -> Value {
        json!({
            "dimensions": {"width": 100, "height": 50},
            "icon": "../icons/missing.svg",
        })
    }

    #[test]
    fn dimensions_and_icon_paths_are_preserved() {
        for path in [
            "",
            "../icons/missing \u{3b1}.svg",
            " leading and trailing spaces ",
            r"C:\icons\missing.png",
        ] {
            let mut value = metadata_json();
            value["icon"] = json!(path);
            let metadata = read(&value).unwrap();
            assert_eq!(metadata.dimensions.width(), 100);
            assert_eq!(metadata.dimensions.height(), 50);
            assert_eq!(metadata.icon.as_str(), path);
        }
    }

    #[test]
    fn icon_rejects_non_string_values() {
        for invalid in [Value::Null, json!(123), json!(false), json!([]), json!({})] {
            let mut value = metadata_json();
            value["icon"] = invalid;
            let error = read(&value).unwrap_err();
            assert!(error.is_data(), "{error}");
            assert!(error.line() > 0);
            assert!(error.column() > 0);
        }
    }

    #[test]
    fn dimensions_require_unsigned_16_bit_integers() {
        for size in [0, u16::MAX] {
            let mut value = metadata_json();
            value["dimensions"] = json!({"width": size, "height": size});
            let metadata = read(&value).unwrap();
            assert_eq!(metadata.dimensions.width(), size);
            assert_eq!(metadata.dimensions.height(), size);
        }
        for field in ["width", "height"] {
            for invalid in [
                json!(-1),
                json!(65536),
                json!(1.5),
                json!(100.0),
                json!("100"),
                Value::Null,
                json!(true),
                json!([]),
                json!({}),
            ] {
                let mut value = metadata_json();
                value["dimensions"][field] = invalid;
                assert!(read(&value).is_err(), "{value}");
            }
        }
    }

    #[test]
    fn fields_are_required_and_unknown_fields_are_rejected() {
        for field in ["dimensions", "icon"] {
            let mut value = metadata_json();
            value.as_object_mut().unwrap().remove(field);
            assert!(read(&value).is_err(), "{value}");
        }
        for field in ["width", "height"] {
            let mut value = metadata_json();
            value["dimensions"].as_object_mut().unwrap().remove(field);
            assert!(read(&value).is_err(), "{value}");
        }
        let mut value = metadata_json();
        value["extra"] = json!(1);
        assert!(read(&value).is_err());
        let mut value = metadata_json();
        value["dimensions"]["extra"] = json!(1);
        assert!(read(&value).is_err());
    }

    #[test]
    fn structs_require_json_objects() {
        for invalid in [
            json!([{"width": 100, "height": 50}, "icon.svg"]),
            Value::Null,
            json!(false),
            json!(100),
            json!("metadata"),
        ] {
            assert!(read(&invalid).is_err(), "{invalid}");
        }
        for invalid in [
            json!([100, 50]),
            Value::Null,
            json!(false),
            json!(100),
            json!("dimensions"),
        ] {
            let mut value = metadata_json();
            value["dimensions"] = invalid;
            assert!(read(&value).is_err(), "{value}");
        }
    }

    #[test]
    fn duplicate_fields_are_rejected() {
        for json in [
            r#"{"dimensions":{"width":100,"height":50},"icon":"a.svg","icon":"b.svg"}"#,
            r#"{"dimensions":{"width":100,"height":50},"dimensions":{"width":1,"height":2},"icon":""}"#,
            r#"{"dimensions":{"width":100,"width":1,"height":50},"icon":""}"#,
            r#"{"dimensions":{"width":100,"height":50,"height":1},"icon":""}"#,
        ] {
            assert!(
                serde_json::from_str::<RootObject<GuiMetadataReaderV1>>(json).is_err(),
                "{json}"
            );
        }
    }
}
