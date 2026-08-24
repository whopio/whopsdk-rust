pub use crate::prelude::*;

/// Controls where this plan can be seen. When `hidden`, the plan is reachable only by its direct link.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PlanVisibility {
    Visible,
    Hidden,
    Archived,
    QuickLink,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PlanVisibility {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Visible => serializer.serialize_str("visible"),
            Self::Hidden => serializer.serialize_str("hidden"),
            Self::Archived => serializer.serialize_str("archived"),
            Self::QuickLink => serializer.serialize_str("quick_link"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PlanVisibility {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "visible" => Ok(Self::Visible),
            "hidden" => Ok(Self::Hidden),
            "archived" => Ok(Self::Archived),
            "quick_link" => Ok(Self::QuickLink),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PlanVisibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visible => write!(f, "visible"),
            Self::Hidden => write!(f, "hidden"),
            Self::Archived => write!(f, "archived"),
            Self::QuickLink => write!(f, "quick_link"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
