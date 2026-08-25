pub use crate::prelude::*;

/// The font the seller chose.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionBrandingFontFamily {
    System,
    Roboto,
    OpenSans,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionBrandingFontFamily {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::System => serializer.serialize_str("system"),
            Self::Roboto => serializer.serialize_str("roboto"),
            Self::OpenSans => serializer.serialize_str("open_sans"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionBrandingFontFamily {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "system" => Ok(Self::System),
            "roboto" => Ok(Self::Roboto),
            "open_sans" => Ok(Self::OpenSans),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionBrandingFontFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::System => write!(f, "system"),
            Self::Roboto => write!(f, "roboto"),
            Self::OpenSans => write!(f, "open_sans"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
