pub use crate::prelude::*;

/// The corner style the seller chose.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionBrandingBorderStyle {
    Rounded,
    Pill,
    Rectangular,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionBrandingBorderStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Rounded => serializer.serialize_str("rounded"),
            Self::Pill => serializer.serialize_str("pill"),
            Self::Rectangular => serializer.serialize_str("rectangular"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionBrandingBorderStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "rounded" => Ok(Self::Rounded),
            "pill" => Ok(Self::Pill),
            "rectangular" => Ok(Self::Rectangular),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionBrandingBorderStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rounded => write!(f, "rounded"),
            Self::Pill => write!(f, "pill"),
            Self::Rectangular => write!(f, "rectangular"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
