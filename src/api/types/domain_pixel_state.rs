pub use crate::prelude::*;

/// `ready` while the Whop pixel recorded a page view on the domain in the last 7 days. `pending` until a pixel you asked for records its first one, and `action_required` once it goes 7 days without one; its issue says what to do.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DomainPixelState {
    Pending,
    Ready,
    ActionRequired,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DomainPixelState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pending => serializer.serialize_str("pending"),
            Self::Ready => serializer.serialize_str("ready"),
            Self::ActionRequired => serializer.serialize_str("action_required"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DomainPixelState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "pending" => Ok(Self::Pending),
            "ready" => Ok(Self::Ready),
            "action_required" => Ok(Self::ActionRequired),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DomainPixelState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "pending"),
            Self::Ready => write!(f, "ready"),
            Self::ActionRequired => write!(f, "action_required"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
