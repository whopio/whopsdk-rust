pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListCandlesRequestInterval {
    OneH,
    TwoH,
    FourH,
    SixH,
    TwelveH,
    OneD,
    OneW,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListCandlesRequestInterval {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::OneH => serializer.serialize_str("1h"),
            Self::TwoH => serializer.serialize_str("2h"),
            Self::FourH => serializer.serialize_str("4h"),
            Self::SixH => serializer.serialize_str("6h"),
            Self::TwelveH => serializer.serialize_str("12h"),
            Self::OneD => serializer.serialize_str("1d"),
            Self::OneW => serializer.serialize_str("1w"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListCandlesRequestInterval {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "1h" => Ok(Self::OneH),
            "2h" => Ok(Self::TwoH),
            "4h" => Ok(Self::FourH),
            "6h" => Ok(Self::SixH),
            "12h" => Ok(Self::TwelveH),
            "1d" => Ok(Self::OneD),
            "1w" => Ok(Self::OneW),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListCandlesRequestInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OneH => write!(f, "1h"),
            Self::TwoH => write!(f, "2h"),
            Self::FourH => write!(f, "4h"),
            Self::SixH => write!(f, "6h"),
            Self::TwelveH => write!(f, "12h"),
            Self::OneD => write!(f, "1d"),
            Self::OneW => write!(f, "1w"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
