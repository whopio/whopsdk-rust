pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogsAppsResponseDataItemSource {
    Console,
    Exception,
    Request,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LogsAppsResponseDataItemSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Console => serializer.serialize_str("console"),
            Self::Exception => serializer.serialize_str("exception"),
            Self::Request => serializer.serialize_str("request"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LogsAppsResponseDataItemSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "console" => Ok(Self::Console),
            "exception" => Ok(Self::Exception),
            "request" => Ok(Self::Request),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LogsAppsResponseDataItemSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Console => write!(f, "console"),
            Self::Exception => write!(f, "exception"),
            Self::Request => write!(f, "request"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
