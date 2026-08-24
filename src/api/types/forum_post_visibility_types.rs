pub use crate::prelude::*;

/// The visibility types for forum posts
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForumPostVisibilityTypes {
    MembersOnly,
    GloballyVisible,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ForumPostVisibilityTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::MembersOnly => serializer.serialize_str("members_only"),
            Self::GloballyVisible => serializer.serialize_str("globally_visible"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ForumPostVisibilityTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "members_only" => Ok(Self::MembersOnly),
            "globally_visible" => Ok(Self::GloballyVisible),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ForumPostVisibilityTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MembersOnly => write!(f, "members_only"),
            Self::GloballyVisible => write!(f, "globally_visible"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
