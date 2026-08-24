pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateDepositsResponseMethodsBankCurrenciesItemRailsItem {
    Ach,
    Wire,
    Sepa,
    Fps,
    Chaps,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateDepositsResponseMethodsBankCurrenciesItemRailsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Ach => serializer.serialize_str("ach"),
            Self::Wire => serializer.serialize_str("wire"),
            Self::Sepa => serializer.serialize_str("sepa"),
            Self::Fps => serializer.serialize_str("fps"),
            Self::Chaps => serializer.serialize_str("chaps"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateDepositsResponseMethodsBankCurrenciesItemRailsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ach" => Ok(Self::Ach),
            "wire" => Ok(Self::Wire),
            "sepa" => Ok(Self::Sepa),
            "fps" => Ok(Self::Fps),
            "chaps" => Ok(Self::Chaps),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateDepositsResponseMethodsBankCurrenciesItemRailsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ach => write!(f, "ach"),
            Self::Wire => write!(f, "wire"),
            Self::Sepa => write!(f, "sepa"),
            Self::Fps => write!(f, "fps"),
            Self::Chaps => write!(f, "chaps"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
