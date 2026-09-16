pub use crate::prelude::*;

/// The operators this field accepts. Any other operator is rejected.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentRuleFieldOperatorsItem {
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    In,
    NotIn,
    Contains,
    StartsWith,
    EndsWith,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentRuleFieldOperatorsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Eq => serializer.serialize_str("eq"),
            Self::Neq => serializer.serialize_str("neq"),
            Self::Gt => serializer.serialize_str("gt"),
            Self::Gte => serializer.serialize_str("gte"),
            Self::Lt => serializer.serialize_str("lt"),
            Self::Lte => serializer.serialize_str("lte"),
            Self::In => serializer.serialize_str("in"),
            Self::NotIn => serializer.serialize_str("not_in"),
            Self::Contains => serializer.serialize_str("contains"),
            Self::StartsWith => serializer.serialize_str("starts_with"),
            Self::EndsWith => serializer.serialize_str("ends_with"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentRuleFieldOperatorsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "eq" => Ok(Self::Eq),
            "neq" => Ok(Self::Neq),
            "gt" => Ok(Self::Gt),
            "gte" => Ok(Self::Gte),
            "lt" => Ok(Self::Lt),
            "lte" => Ok(Self::Lte),
            "in" => Ok(Self::In),
            "not_in" => Ok(Self::NotIn),
            "contains" => Ok(Self::Contains),
            "starts_with" => Ok(Self::StartsWith),
            "ends_with" => Ok(Self::EndsWith),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentRuleFieldOperatorsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eq => write!(f, "eq"),
            Self::Neq => write!(f, "neq"),
            Self::Gt => write!(f, "gt"),
            Self::Gte => write!(f, "gte"),
            Self::Lt => write!(f, "lt"),
            Self::Lte => write!(f, "lte"),
            Self::In => write!(f, "in"),
            Self::NotIn => write!(f, "not_in"),
            Self::Contains => write!(f, "contains"),
            Self::StartsWith => write!(f, "starts_with"),
            Self::EndsWith => write!(f, "ends_with"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
