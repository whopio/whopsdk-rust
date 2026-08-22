pub use crate::prelude::*;

/// How a payment method will be charged after the buyer leaves — the same vocabulary as a confirmation token's setup_future_usage.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FutureUsageTypes {
    OffSession,
    OnSession,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FutureUsageTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OffSession => serializer.serialize_str("off_session"),
            Self::OnSession => serializer.serialize_str("on_session"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FutureUsageTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "off_session" => Ok(Self::OffSession),
            "on_session" => Ok(Self::OnSession),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FutureUsageTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OffSession => write!(f, "off_session"),
            Self::OnSession => write!(f, "on_session"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
