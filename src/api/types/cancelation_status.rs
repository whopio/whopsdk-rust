pub use crate::prelude::*;

/// The state of a membership after a customer provides a cancelation reason.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CancelationStatus {
    WonBack,
    Left,
    Canceling,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CancelationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::WonBack => serializer.serialize_str("won_back"),
            Self::Left => serializer.serialize_str("left"),
            Self::Canceling => serializer.serialize_str("canceling"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CancelationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "won_back" => Ok(Self::WonBack),
            "left" => Ok(Self::Left),
            "canceling" => Ok(Self::Canceling),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CancelationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WonBack => write!(f, "won_back"),
            Self::Left => write!(f, "left"),
            Self::Canceling => write!(f, "canceling"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
