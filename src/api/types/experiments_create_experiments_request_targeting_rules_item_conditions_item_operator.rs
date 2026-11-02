pub use crate::prelude::*;

/// Comparison to apply: `any`/`none` — inclusion/exclusion list; `eq`/`neq` — equality; `gt`/`gte`/`lt`/`lte` — numeric range.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateExperimentsRequestTargetingRulesItemConditionsItemOperator {
    Any,
    None,
    Eq,
    Neq,
    Gt,
    Gte,
    Lt,
    Lte,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateExperimentsRequestTargetingRulesItemConditionsItemOperator {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Any => serializer.serialize_str("any"),
            Self::None => serializer.serialize_str("none"),
            Self::Eq => serializer.serialize_str("eq"),
            Self::Neq => serializer.serialize_str("neq"),
            Self::Gt => serializer.serialize_str("gt"),
            Self::Gte => serializer.serialize_str("gte"),
            Self::Lt => serializer.serialize_str("lt"),
            Self::Lte => serializer.serialize_str("lte"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateExperimentsRequestTargetingRulesItemConditionsItemOperator {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "any" => Ok(Self::Any),
            "none" => Ok(Self::None),
            "eq" => Ok(Self::Eq),
            "neq" => Ok(Self::Neq),
            "gt" => Ok(Self::Gt),
            "gte" => Ok(Self::Gte),
            "lt" => Ok(Self::Lt),
            "lte" => Ok(Self::Lte),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateExperimentsRequestTargetingRulesItemConditionsItemOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::None => write!(f, "none"),
            Self::Eq => write!(f, "eq"),
            Self::Neq => write!(f, "neq"),
            Self::Gt => write!(f, "gt"),
            Self::Gte => write!(f, "gte"),
            Self::Lt => write!(f, "lt"),
            Self::Lte => write!(f, "lte"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
