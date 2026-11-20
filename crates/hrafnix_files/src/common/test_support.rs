use crate::common::deserialize::RootObject;
use hrafnix_message::message::Message;
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt;

/// The stage at which reading a document failed.
#[derive(Debug)]
pub(crate) enum ReadError {
    /// Structural JSON parsing failed.
    Json(serde_json::Error),
    /// Post-parse conversion returned a localized diagnostic.
    Conversion(Message),
}

impl ReadError {
    /// Returns the JSON error, panicking if conversion failed instead.
    pub(crate) fn json(self) -> serde_json::Error {
        match self {
            Self::Json(error) => error,
            Self::Conversion(message) => panic!("expected a JSON error, got {message:?}"),
        }
    }

    /// Returns the localized diagnostic, panicking if parsing failed instead.
    pub(crate) fn message(self) -> Message {
        match self {
            Self::Json(error) => panic!("expected a conversion error, got {error}"),
            Self::Conversion(message) => message,
        }
    }

    /// Returns the localized diagnostic key, panicking if parsing failed instead.
    pub(crate) fn message_key(self) -> String {
        self.message()
            .translate_data()
            .message_key()
            .as_str()
            .to_owned()
    }
}

impl fmt::Display for ReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(error) => write!(f, "{error}"),
            Self::Conversion(message) => write!(f, "{message:?}"),
        }
    }
}

/// Parses `json` through the object-only envelope, then converts it.
pub(crate) fn read_str<R: DeserializeOwned, T: TryFrom<R, Error = Message>>(
    json: &str,
) -> Result<T, ReadError> {
    let reader: RootObject<R> = serde_json::from_str(json).map_err(ReadError::Json)?;
    T::try_from(reader.document).map_err(ReadError::Conversion)
}

/// Serializes `value` to text so JSON errors keep positions, then reads it.
pub(crate) fn read_value<R: DeserializeOwned, T: TryFrom<R, Error = Message>>(
    value: &Value,
) -> Result<T, ReadError> {
    read_str::<R, T>(&serde_json::to_string(value).unwrap())
}

/// Reads an owned `value` directly; JSON errors have no text positions.
pub(crate) fn read_owned_value<R: DeserializeOwned, T: TryFrom<R, Error = Message>>(
    value: Value,
) -> Result<T, ReadError> {
    let reader: RootObject<R> = serde_json::from_value(value).map_err(ReadError::Json)?;
    T::try_from(reader.document).map_err(ReadError::Conversion)
}
