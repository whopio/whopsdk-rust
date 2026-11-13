pub use crate::prelude::*;

/// How closely a search has to match: `broad` also reaches related searches, `phrase` needs the meaning of the phrase, and `exact` needs the same meaning as the term. Defaults to `broad`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CreateAdGroupsRequestKeywordsItemMatchType {
    Broad,
    Phrase,
    Exact,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CreateAdGroupsRequestKeywordsItemMatchType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Broad => serializer.serialize_str("broad"),
            Self::Phrase => serializer.serialize_str("phrase"),
            Self::Exact => serializer.serialize_str("exact"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CreateAdGroupsRequestKeywordsItemMatchType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "broad" => Ok(Self::Broad),
            "phrase" => Ok(Self::Phrase),
            "exact" => Ok(Self::Exact),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CreateAdGroupsRequestKeywordsItemMatchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Broad => write!(f, "broad"),
            Self::Phrase => write!(f, "phrase"),
            Self::Exact => write!(f, "exact"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
