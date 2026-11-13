pub use crate::prelude::*;

/// How far proving verification has got.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainVerificationState {
    Pending,
    Ready,
    ActionRequired,
    Releasing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DomainVerificationState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Ready => serializer.serialize_str("ready"),
            Self::ActionRequired => serializer.serialize_str("action_required"),
            Self::Releasing => serializer.serialize_str("releasing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DomainVerificationState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "ready" => Ok(Self::Ready),
            "action_required" => Ok(Self::ActionRequired),
            "releasing" => Ok(Self::Releasing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DomainVerificationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Ready => write!(f, "ready"),
            Self::ActionRequired => write!(f, "action_required"),
            Self::Releasing => write!(f, "releasing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
