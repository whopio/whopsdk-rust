pub use crate::prelude::*;

/// What the step does for this member: `sent` its message, `skipped` it and moved on to the next step, or `exited` the automation.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AutomationStepPreviewOutcome {
    Sent,
    Skipped,
    Exited,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AutomationStepPreviewOutcome {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sent => serializer.serialize_str("sent"),
            Self::Skipped => serializer.serialize_str("skipped"),
            Self::Exited => serializer.serialize_str("exited"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AutomationStepPreviewOutcome {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sent" => Ok(Self::Sent),
            "skipped" => Ok(Self::Skipped),
            "exited" => Ok(Self::Exited),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AutomationStepPreviewOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sent => write!(f, "sent"),
            Self::Skipped => write!(f, "skipped"),
            Self::Exited => write!(f, "exited"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
