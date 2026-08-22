pub use crate::prelude::*;

/// The mode of cancellation for a membership
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MembershipCancellationModes {
    AtPeriodEnd,
    Immediate,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MembershipCancellationModes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AtPeriodEnd => serializer.serialize_str("at_period_end"),
            Self::Immediate => serializer.serialize_str("immediate"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MembershipCancellationModes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "at_period_end" => Ok(Self::AtPeriodEnd),
            "immediate" => Ok(Self::Immediate),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MembershipCancellationModes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AtPeriodEnd => write!(f, "at_period_end"),
            Self::Immediate => write!(f, "immediate"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
