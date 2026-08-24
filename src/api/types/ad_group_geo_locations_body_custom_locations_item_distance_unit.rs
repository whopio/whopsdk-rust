pub use crate::prelude::*;

/// Unit for `radius`. Defaults to `mile`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdGroupGeoLocationsBodyCustomLocationsItemDistanceUnit {
    Mile,
    Kilometer,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdGroupGeoLocationsBodyCustomLocationsItemDistanceUnit {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Mile => serializer.serialize_str("mile"),
            Self::Kilometer => serializer.serialize_str("kilometer"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdGroupGeoLocationsBodyCustomLocationsItemDistanceUnit {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "mile" => Ok(Self::Mile),
            "kilometer" => Ok(Self::Kilometer),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdGroupGeoLocationsBodyCustomLocationsItemDistanceUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mile => write!(f, "mile"),
            Self::Kilometer => write!(f, "kilometer"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
