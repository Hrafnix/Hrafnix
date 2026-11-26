use crate::path::Path;
use hrafnix_shareable_string::{ShareableString, SharedStringTranslationMap, TranslateMessage};
use std::collections::HashMap;
use std::fmt;

/// An enumeration representing the level of message.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageLevel {
    /// A message that is for debugging purposes.
    Debug,
    /// A message that is informational in nature.
    Info,
    /// A message that is a warning.
    Warning,
    /// A message that is an error.
    Error,
}

impl fmt::Display for MessageLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MessageLevel::Debug => "Debug",
            MessageLevel::Info => "Info",
            MessageLevel::Warning => "Warning",
            MessageLevel::Error => "Error",
        };
        f.write_str(s)
    }
}

/// A message that is associated with a specific category.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageCategory {
    /// A message related to reading or writing files.
    File,
    /// A message related to the datastore.
    Datastore,
    /// A message related to a component.
    Component,
    /// A message related to expression parsing.
    ExpressionParsing,
    /// A message related to expression evaluation.
    ExpressionEvaluation,
}

impl fmt::Display for MessageCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            MessageCategory::File => "File",
            MessageCategory::Datastore => "Datastore",
            MessageCategory::Component => "Component",
            MessageCategory::ExpressionParsing => "Expression Parsing",
            MessageCategory::ExpressionEvaluation => "Expression Evaluation",
        };
        f.write_str(s)
    }
}

/// A message with detailed information, including its path, level, category, and content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// The path of the object associated with this message.
    object_path: Option<Box<Path>>,
    /// The path of the item associated with this message.
    item_path: Option<Box<Path>>,
    /// The severity level of this message.
    level: MessageLevel,
    /// The category of this message.
    category: MessageCategory,
    /// The key/params for the message.
    translate_data: Box<TranslateMessage>,
    /// The translated message, if available.
    extra_detail: Option<ShareableString>,
}

impl Message {
    /// Creates an error message without parameters.
    #[must_use]
    pub fn error(category: MessageCategory, message_key: impl Into<ShareableString>) -> Self {
        Self::new_with_params(
            MessageLevel::Error,
            category,
            message_key.into(),
            HashMap::new(),
            None,
        )
    }

    /// Creates an error message with one named parameter.
    #[must_use]
    pub fn error_with_param(
        category: MessageCategory,
        message_key: impl Into<ShareableString>,
        parameter: impl Into<ShareableString>,
        value: impl Into<ShareableString>,
    ) -> Self {
        Self::new_with_params(
            MessageLevel::Error,
            category,
            message_key.into(),
            HashMap::from([(parameter.into(), value.into())]),
            None,
        )
    }

    /// Creates a new `Message` with the specified details.
    #[must_use]
    pub fn new(
        level: MessageLevel,
        category: MessageCategory,
        translate_data: TranslateMessage,
        extra_detail: Option<ShareableString>,
    ) -> Self {
        Self {
            object_path: None,
            item_path: None,
            level,
            category,
            translate_data: Box::new(translate_data),
            extra_detail,
        }
    }

    /// Creates a new `Message` with the specified details, using a message key and parameters.
    #[must_use]
    pub fn new_with_params(
        level: MessageLevel,
        category: MessageCategory,
        message_key: ShareableString,
        message_params: HashMap<ShareableString, ShareableString>,
        extra_detail: Option<ShareableString>,
    ) -> Self {
        let translate_data = TranslateMessage::new(message_key, message_params);
        Self::new(level, category, translate_data, extra_detail)
    }

    /// Overrides the object path of the message with a new path.
    pub fn override_object_path(&mut self, new_object_path: Option<Path>) {
        self.object_path = new_object_path.map(Box::new);
    }

    /// Overrides the item path of the message with a new path.
    pub fn override_item_path(&mut self, new_item_path: Option<Path>) {
        self.item_path = new_item_path.map(Box::new);
    }

    /// Returns the path of the object associated with this message.
    #[must_use]
    pub const fn object_path(&self) -> Option<&Path> {
        match &self.object_path {
            Some(path) => Some(path),
            None => None,
        }
    }

    /// Returns the path of the item associated with this message.
    #[must_use]
    pub const fn item_path(&self) -> Option<&Path> {
        match &self.item_path {
            Some(path) => Some(path),
            None => None,
        }
    }

    /// Returns the level of this message.
    #[must_use]
    pub const fn level(&self) -> MessageLevel {
        self.level
    }

    /// Returns the category of this message.
    #[must_use]
    pub const fn category(&self) -> MessageCategory {
        self.category
    }

    /// Translates the message using the provided translation map and language.
    #[must_use]
    pub fn translated_message(
        &self,
        translation_map: &SharedStringTranslationMap,
        lang: &str,
    ) -> Option<ShareableString> {
        self.translate_data.translate(translation_map, lang)
    }

    /// Returns the translation message associated with this message.
    #[must_use]
    pub const fn translate_data(&self) -> &TranslateMessage {
        &self.translate_data
    }

    /// Returns the extra detail associated with this message, if available.
    #[must_use]
    pub const fn extra_detail(&self) -> Option<&ShareableString> {
        self.extra_detail.as_ref()
    }

    /// Determines if this message should be displayed based on the provided minimum level.
    #[must_use]
    pub const fn should_display(&self, min_level: MessageLevel) -> bool {
        matches!(
            (self.level, min_level),
            (
                MessageLevel::Debug | MessageLevel::Info | MessageLevel::Warning,
                MessageLevel::Debug
            ) | (
                MessageLevel::Info | MessageLevel::Warning,
                MessageLevel::Info
            ) | (MessageLevel::Warning, MessageLevel::Warning)
                | (
                    MessageLevel::Error,
                    MessageLevel::Debug
                        | MessageLevel::Info
                        | MessageLevel::Warning
                        | MessageLevel::Error
                )
        )
    }

    /// Interns this value's strings in `store` in place, preserving its contents.
    pub fn launder(&mut self, store: &hrafnix_shareable_string::SharedStringStore) {
        if let Some(path) = &mut self.object_path {
            path.launder(store);
        }
        if let Some(path) = &mut self.item_path {
            path.launder(store);
        }
        self.translate_data.launder(store);
        if let Some(detail) = &mut self.extra_detail {
            *detail = store.launder(&*detail);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hrafnix_shareable_string::SharedStringStore;

    #[test]
    fn laundering_interns_message_paths_parameters_and_detail_in_place() {
        let store = SharedStringStore::new();
        for text in [
            "message",
            "parameter",
            "value",
            "detail",
            "object",
            "item",
            "segment",
        ] {
            store.get(text);
        }
        let mut message = Message::new_with_params(
            MessageLevel::Error,
            MessageCategory::Datastore,
            ShareableString::new("message"),
            HashMap::from([(
                ShareableString::new("parameter"),
                ShareableString::new("value"),
            )]),
            Some(ShareableString::new("detail")),
        );
        message.override_object_path(Some(Path::new("object").with_segment("segment")));
        message.override_item_path(Some(Path::new("item").with_segment("segment")));
        let original = message.clone();
        message.launder(&store);
        assert_eq!(message, original);
        assert!(
            message
                .translate_data()
                .message_key()
                .ptr_eq(&store.get("message"))
        );
        for (key, value) in message.translate_data().message_params() {
            assert!(key.ptr_eq(&store.get("parameter")));
            assert!(value.ptr_eq(&store.get("value")));
        }
        assert!(message.extra_detail().unwrap().ptr_eq(&store.get("detail")));
        for path in [message.object_path().unwrap(), message.item_path().unwrap()] {
            assert!(path.object_key().ptr_eq(&store.get(path.object_key())));
            for segment in path.segments() {
                assert!(segment.ptr_eq(&store.get("segment")));
            }
        }
        assert!(
            !original
                .translate_data()
                .message_key()
                .ptr_eq(&store.get("message"))
        );
        message.launder(&store);
        assert_eq!(message, original);

        let mut empty = Message::error(MessageCategory::File, "message");
        let original = empty.clone();
        empty.launder(&store);
        assert_eq!(empty, original);
    }
}
