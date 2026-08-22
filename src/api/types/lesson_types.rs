pub use crate::prelude::*;

/// The available types for a lesson
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LessonTypes {
    Text,
    Video,
    Pdf,
    Multi,
    Quiz,
    KnowledgeCheck,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LessonTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Text => serializer.serialize_str("text"),
            Self::Video => serializer.serialize_str("video"),
            Self::Pdf => serializer.serialize_str("pdf"),
            Self::Multi => serializer.serialize_str("multi"),
            Self::Quiz => serializer.serialize_str("quiz"),
            Self::KnowledgeCheck => serializer.serialize_str("knowledge_check"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LessonTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "text" => Ok(Self::Text),
            "video" => Ok(Self::Video),
            "pdf" => Ok(Self::Pdf),
            "multi" => Ok(Self::Multi),
            "quiz" => Ok(Self::Quiz),
            "knowledge_check" => Ok(Self::KnowledgeCheck),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LessonTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Video => write!(f, "video"),
            Self::Pdf => write!(f, "pdf"),
            Self::Multi => write!(f, "multi"),
            Self::Quiz => write!(f, "quiz"),
            Self::KnowledgeCheck => write!(f, "knowledge_check"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
