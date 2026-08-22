pub use crate::prelude::*;

/// The different levels of visibility for resources
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VisibilityFilter {
    Visible,
    Hidden,
    Archived,
    QuickLink,
    All,
    NotQuickLink,
    NotArchived,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for VisibilityFilter {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Visible => serializer.serialize_str("visible"),
            Self::Hidden => serializer.serialize_str("hidden"),
            Self::Archived => serializer.serialize_str("archived"),
            Self::QuickLink => serializer.serialize_str("quick_link"),
            Self::All => serializer.serialize_str("all"),
            Self::NotQuickLink => serializer.serialize_str("not_quick_link"),
            Self::NotArchived => serializer.serialize_str("not_archived"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for VisibilityFilter {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "visible" => Ok(Self::Visible),
            "hidden" => Ok(Self::Hidden),
            "archived" => Ok(Self::Archived),
            "quick_link" => Ok(Self::QuickLink),
            "all" => Ok(Self::All),
            "not_quick_link" => Ok(Self::NotQuickLink),
            "not_archived" => Ok(Self::NotArchived),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for VisibilityFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Visible => write!(f, "visible"),
            Self::Hidden => write!(f, "hidden"),
            Self::Archived => write!(f, "archived"),
            Self::QuickLink => write!(f, "quick_link"),
            Self::All => write!(f, "all"),
            Self::NotQuickLink => write!(f, "not_quick_link"),
            Self::NotArchived => write!(f, "not_archived"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
