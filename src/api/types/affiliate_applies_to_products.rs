pub use crate::prelude::*;

/// Whether a rev-share override applies to a single product or all products
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AffiliateAppliesToProducts {
    SingleProduct,
    AllProducts,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AffiliateAppliesToProducts {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SingleProduct => serializer.serialize_str("single_product"),
            Self::AllProducts => serializer.serialize_str("all_products"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AffiliateAppliesToProducts {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "single_product" => Ok(Self::SingleProduct),
            "all_products" => Ok(Self::AllProducts),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AffiliateAppliesToProducts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SingleProduct => write!(f, "single_product"),
            Self::AllProducts => write!(f, "all_products"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
