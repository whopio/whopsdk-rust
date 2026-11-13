pub use crate::prelude::*;

/// How far the setup has got. **A 201 or 200 means we answered, not that the method was saved — always branch on this.** `requires_action` — the buyer has a step outstanding; hand `client_secret` to the elements or poll Retrieve setup status. `processing` — the processor is deciding. `succeeded` — the method is saved, and only this one means saved. `canceled` — abandoned or refused; see `last_setup_error`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SetupIntentStatus {
    Processing,
    Succeeded,
    Canceled,
    RequiresAction,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SetupIntentStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Processing => serializer.serialize_str("processing"),
            Self::Succeeded => serializer.serialize_str("succeeded"),
            Self::Canceled => serializer.serialize_str("canceled"),
            Self::RequiresAction => serializer.serialize_str("requires_action"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SetupIntentStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "processing" => Ok(Self::Processing),
            "succeeded" => Ok(Self::Succeeded),
            "canceled" => Ok(Self::Canceled),
            "requires_action" => Ok(Self::RequiresAction),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SetupIntentStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Processing => write!(f, "processing"),
            Self::Succeeded => write!(f, "succeeded"),
            Self::Canceled => write!(f, "canceled"),
            Self::RequiresAction => write!(f, "requires_action"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
