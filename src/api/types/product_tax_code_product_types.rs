pub use crate::prelude::*;

/// The product_type of the ProductTaxCode
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProductTaxCodeProductTypes {
    Physical,
    Digital,
    Services,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ProductTaxCodeProductTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Physical => serializer.serialize_str("physical"),
            Self::Digital => serializer.serialize_str("digital"),
            Self::Services => serializer.serialize_str("services"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ProductTaxCodeProductTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "physical" => Ok(Self::Physical),
            "digital" => Ok(Self::Digital),
            "services" => Ok(Self::Services),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ProductTaxCodeProductTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Physical => write!(f, "physical"),
            Self::Digital => write!(f, "digital"),
            Self::Services => write!(f, "services"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
