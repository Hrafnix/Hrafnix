use crate::data::metadata_general::MetadataFile;
use serde::Deserialize;

/// The version 1 JSON representation of the `metadata` entry, separate from the
/// application's metadata.
///
/// Every field is required and unknown fields are rejected so nothing is silently
/// dropped when the file is written back.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
pub(crate) struct MetadataReaderV1 {
    /// Name shown to users.
    display_name: String,
    /// Free-form description.
    description: String,
    /// Author of the file.
    author: String,
    /// Major part of the minimum Hrafnix version.
    minimum_version_major: u16,
    /// Minor part of the minimum Hrafnix version.
    minimum_version_minor: u16,
}

impl From<MetadataReaderV1> for MetadataFile {
    fn from(metadata: MetadataReaderV1) -> Self {
        Self::new(
            metadata.display_name.into(),
            metadata.description.into(),
            metadata.author.into(),
            metadata.minimum_version_major,
            metadata.minimum_version_minor,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::MetadataReaderV1;
    use crate::common::deserialize::RootObject;
    use crate::data::metadata_general::MetadataFile;
    use serde_json::{Value, json};

    /// Metadata with distinct values for each field.
    fn metadata_json() -> Value {
        json!({
            "display-name": "Example", "description": "Description", "author": "Author",
            "minimum-version-major": 1, "minimum-version-minor": 2
        })
    }

    #[test]
    fn preserves_all_metadata_fields() {
        let reader: RootObject<MetadataReaderV1> = serde_json::from_value(metadata_json()).unwrap();
        let metadata = MetadataFile::from(reader.document);
        assert_eq!(metadata.display_name(), "Example");
        assert_eq!(metadata.description(), "Description");
        assert_eq!(metadata.author(), "Author");
        assert_eq!(metadata.minimum_version_major(), 1);
        assert_eq!(metadata.minimum_version_minor(), 2);
    }

    #[test]
    fn requires_fields_and_unsigned_versions() {
        let value = metadata_json();
        for field in value.as_object().unwrap().keys() {
            let mut invalid = value.clone();
            invalid.as_object_mut().unwrap().remove(field);
            assert!(
                serde_json::from_value::<RootObject<MetadataReaderV1>>(invalid).is_err(),
                "{field}"
            );
        }
        for field in ["minimum-version-major", "minimum-version-minor"] {
            for version in [json!(-1), json!(65536), json!("1")] {
                let mut invalid = value.clone();
                invalid[field] = version;
                assert!(serde_json::from_value::<RootObject<MetadataReaderV1>>(invalid).is_err());
            }
        }
        let mut invalid = value;
        invalid["unknown"] = json!(true);
        assert!(serde_json::from_value::<RootObject<MetadataReaderV1>>(invalid).is_err());
    }
}
