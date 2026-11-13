pub use crate::prelude::*;

/// The BNPL provider whose dispute rate triggered enforcement. Other payment methods may also lose access.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinancingAccessChangeProvider {
    Klarna,
    Splitit,
    AfterpayClearpay,
    Zip,
    Sezzle,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FinancingAccessChangeProvider {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Klarna => serializer.serialize_str("klarna"),
            Self::Splitit => serializer.serialize_str("splitit"),
            Self::AfterpayClearpay => serializer.serialize_str("afterpay_clearpay"),
            Self::Zip => serializer.serialize_str("zip"),
            Self::Sezzle => serializer.serialize_str("sezzle"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FinancingAccessChangeProvider {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "klarna" => Ok(Self::Klarna),
            "splitit" => Ok(Self::Splitit),
            "afterpay_clearpay" => Ok(Self::AfterpayClearpay),
            "zip" => Ok(Self::Zip),
            "sezzle" => Ok(Self::Sezzle),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FinancingAccessChangeProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Klarna => write!(f, "klarna"),
            Self::Splitit => write!(f, "splitit"),
            Self::AfterpayClearpay => write!(f, "afterpay_clearpay"),
            Self::Zip => write!(f, "zip"),
            Self::Sezzle => write!(f, "sezzle"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
