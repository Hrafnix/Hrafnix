use crate::common::deserialize::deserialize_object;
use crate::foreground_decoration::data::{
    ForegroundDecoration, ForegroundDecorationKind, ImageDecoration, ImageFit, TextDecoration,
};
use crate::graphics::reader::v1::{
    ColorReaderV1, DimensionReaderV1, FontSizeReaderV1, PositionReaderV1,
};
use serde::Deserialize;

/// The version 1 JSON representation of a foreground decoration.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ForegroundDecorationReaderV1 {
    /// Identifier of the decoration.
    id: String,
    /// Position in drawing coordinates.
    #[serde(deserialize_with = "deserialize_object")]
    position: PositionReaderV1,
    /// Dimensions in drawing coordinates.
    #[serde(deserialize_with = "deserialize_object")]
    dimensions: DimensionReaderV1,
    /// The applicable text or image payload.
    kind: ForegroundDecorationKindReaderV1,
}

impl From<ForegroundDecorationReaderV1> for ForegroundDecoration {
    fn from(decoration: ForegroundDecorationReaderV1) -> Self {
        Self {
            id: decoration.id.into(),
            position: decoration.position.into(),
            dimensions: decoration.dimensions.into(),
            kind: decoration.kind.into(),
        }
    }
}

/// The content allowed in a foreground layer.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) enum ForegroundDecorationKindReaderV1 {
    /// Text displayed within the decoration's bounds.
    Text(#[serde(deserialize_with = "deserialize_object")] TextDecorationReaderV1),
    /// An image displayed within the decoration's bounds.
    Image(#[serde(deserialize_with = "deserialize_object")] ImageDecorationReaderV1),
}

impl From<ForegroundDecorationKindReaderV1> for ForegroundDecorationKind {
    fn from(kind: ForegroundDecorationKindReaderV1) -> Self {
        match kind {
            ForegroundDecorationKindReaderV1::Text(text) => Self::Text(text.into()),
            ForegroundDecorationKindReaderV1::Image(image) => Self::Image(image.into()),
        }
    }
}

/// Text content and styling for a foreground decoration.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct TextDecorationReaderV1 {
    /// The displayed text.
    pub text: String,
    /// Font size in hundredths of a drawing unit: `2000` represents `20.00`.
    pub font_size: FontSizeReaderV1,
    /// Text color, using the theme's foreground color for
    /// [`ThemeDefault`](crate::graphics::data::Color::ThemeDefault).
    pub text_color: ColorReaderV1,
}

impl From<TextDecorationReaderV1> for TextDecoration {
    fn from(component: TextDecorationReaderV1) -> Self {
        Self {
            text: component.text.into(),
            font_size: component.font_size.into(),
            text_color: component.text_color.into(),
        }
    }
}

/// Image content and fitting for a foreground decoration.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ImageDecorationReaderV1 {
    /// Image path, stored unchanged rather than opened or resolved during file I/O.
    pub path: String,
    /// How the image fits within the decoration's bounds.
    pub fit: ImageFitReaderV1,
}

impl From<ImageDecorationReaderV1> for ImageDecoration {
    fn from(component: ImageDecorationReaderV1) -> Self {
        Self {
            path: component.path.into(),
            fit: component.fit.into(),
        }
    }
}

/// How an image fits within its decoration's bounds.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) enum ImageFitReaderV1 {
    /// Preserve the aspect ratio and show the whole image within the bounds.
    Contain,
    /// Preserve the aspect ratio and cover the bounds, cropping if necessary.
    Cover,
    /// Stretch the image to fill the bounds.
    Fill,
}

impl From<ImageFitReaderV1> for ImageFit {
    fn from(component: ImageFitReaderV1) -> Self {
        match component {
            ImageFitReaderV1::Contain => Self::Contain,
            ImageFitReaderV1::Cover => Self::Cover,
            ImageFitReaderV1::Fill => Self::Fill,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ForegroundDecorationReaderV1;
    use crate::common::deserialize::RootObject;
    use crate::foreground_decoration::data::{
        ForegroundDecoration, ForegroundDecorationKind, ImageFit,
    };
    use crate::graphics::data::{Color, Rgba};
    use serde_json::{Value, json};

    /// Reads the current nested reader schema through its object-only envelope.
    fn read(value: &Value) -> Result<ForegroundDecoration, serde_json::Error> {
        let reader: RootObject<ForegroundDecorationReaderV1> =
            serde_json::from_str(&serde_json::to_string(value)?)?;
        Ok(reader.document.into())
    }

    /// Creates text with explicit font and color fields.
    fn text_json() -> Value {
        json!({
            "id": " label \u{3b1} ",
            "position": {"x": -10, "y": 20},
            "dimensions": {"width": 100, "height": 50},
            "kind": {
                "Text": {
                    "text": " Title \u{3b1}\n ",
                    "font_size": 2050,
                    "text_color": "#AABBCC80",
                },
            },
        })
    }

    /// Creates an image without reading or resolving the supplied path.
    fn image_json(path: &str, fit: &str) -> Value {
        json!({
            "id": "image",
            "position": {"x": 0, "y": 0},
            "dimensions": {"width": 100, "height": 50},
            "kind": {"Image": {"path": path, "fit": fit}},
        })
    }

    #[test]
    fn shared_strings_preserve_their_contents() {
        let decoration = read(&text_json()).unwrap();
        assert_eq!(decoration.id.as_str(), " label \u{3b1} ");
        assert_eq!(decoration.position.x(), -10);
        assert_eq!(decoration.position.y(), 20);
        let ForegroundDecorationKind::Text(text) = decoration.kind else {
            panic!("expected text");
        };
        assert_eq!(text.text.as_str(), " Title \u{3b1}\n ");
        assert_eq!(text.font_size.size(), 2050);
        assert_eq!(
            text.text_color,
            Color::Custom(Rgba::new(170, 187, 204, 128))
        );
    }

    #[test]
    fn paths_are_preserved_for_all_image_fits() {
        for path in [
            "",
            "../images/missing \u{3b1}.svg",
            " leading and trailing spaces ",
            r"C:\images\missing.png",
        ] {
            for (fit, expected) in [
                ("Contain", ImageFit::Contain),
                ("Cover", ImageFit::Cover),
                ("Fill", ImageFit::Fill),
            ] {
                let decoration = read(&image_json(path, fit)).unwrap();
                let ForegroundDecorationKind::Image(image) = decoration.kind else {
                    panic!("expected image");
                };
                assert_eq!(image.path.as_str(), path);
                assert_eq!(image.fit, expected);
            }
        }
    }

    #[test]
    fn string_fields_reject_non_string_values() {
        for invalid in [Value::Null, json!(123), json!(false), json!([]), json!({})] {
            let mut id = text_json();
            id["id"] = invalid.clone();
            let mut text = text_json();
            text["kind"]["Text"]["text"] = invalid.clone();
            let mut image = image_json("image.png", "Contain");
            image["kind"]["Image"]["path"] = invalid;

            for value in [id, text, image] {
                let error = read(&value).unwrap_err();
                assert!(error.is_data(), "{error}");
                assert!(error.line() > 0);
                assert!(error.column() > 0);
            }
        }
    }

    #[test]
    fn required_fields_and_unknown_field_checks_are_preserved() {
        for field in ["text", "font_size", "text_color"] {
            let mut value = text_json();
            value["kind"]["Text"].as_object_mut().unwrap().remove(field);
            assert!(read(&value).is_err());
        }
        let mut image = image_json("image.png", "Contain");
        image["kind"]["Image"]
            .as_object_mut()
            .unwrap()
            .remove("path");
        assert!(read(&image).is_err());

        let mut text = text_json();
        text["kind"]["Text"]["extra"] = json!(1);
        assert!(read(&text).is_err());
        let mut image = image_json("image.png", "Contain");
        image["kind"]["Image"]["extra"] = json!(1);
        assert!(read(&image).is_err());
    }

    #[test]
    fn nested_structs_reject_positional_arrays() {
        for (field, array) in [
            ("position", json!([-10, 20])),
            ("dimensions", json!([100, 50])),
        ] {
            let mut value = text_json();
            value[field] = array;
            assert!(read(&value).is_err());
        }
        let mut text = text_json();
        text["kind"]["Text"]["font_size"] = json!([2050]);
        assert!(read(&text).is_err());
        text["kind"]["Text"] = json!(["Title", 2050, "theme-default"]);
        assert!(read(&text).is_err());
        let mut image = image_json("image.png", "Contain");
        image["kind"]["Image"] = json!(["image.png", "Contain"]);
        assert!(read(&image).is_err());
    }
}
