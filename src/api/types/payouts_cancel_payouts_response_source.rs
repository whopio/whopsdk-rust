pub use crate::prelude::*;

/// How the payout was created. `automatic` means a scheduled auto-payout; `null` on payouts created before source tracking or through internal tooling.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CancelPayoutsResponseSource {
    Api,
    Dashboard,
    Automatic,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CancelPayoutsResponseSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Api => serializer.serialize_str("api"),
            Self::Dashboard => serializer.serialize_str("dashboard"),
            Self::Automatic => serializer.serialize_str("automatic"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CancelPayoutsResponseSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "api" => Ok(Self::Api),
            "dashboard" => Ok(Self::Dashboard),
            "automatic" => Ok(Self::Automatic),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CancelPayoutsResponseSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Api => write!(f, "api"),
            Self::Dashboard => write!(f, "dashboard"),
            Self::Automatic => write!(f, "automatic"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
