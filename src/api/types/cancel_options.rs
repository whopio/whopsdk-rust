pub use crate::prelude::*;

/// The different reasons a user can choose for why they are canceling their membership.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CancelOptions {
    TooExpensive,
    Switching,
    MissingFeatures,
    TechnicalIssues,
    BadExperience,
    Other,
    Testing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CancelOptions {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TooExpensive => serializer.serialize_str("too_expensive"),
            Self::Switching => serializer.serialize_str("switching"),
            Self::MissingFeatures => serializer.serialize_str("missing_features"),
            Self::TechnicalIssues => serializer.serialize_str("technical_issues"),
            Self::BadExperience => serializer.serialize_str("bad_experience"),
            Self::Other => serializer.serialize_str("other"),
            Self::Testing => serializer.serialize_str("testing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CancelOptions {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "too_expensive" => Ok(Self::TooExpensive),
            "switching" => Ok(Self::Switching),
            "missing_features" => Ok(Self::MissingFeatures),
            "technical_issues" => Ok(Self::TechnicalIssues),
            "bad_experience" => Ok(Self::BadExperience),
            "other" => Ok(Self::Other),
            "testing" => Ok(Self::Testing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CancelOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooExpensive => write!(f, "too_expensive"),
            Self::Switching => write!(f, "switching"),
            Self::MissingFeatures => write!(f, "missing_features"),
            Self::TechnicalIssues => write!(f, "technical_issues"),
            Self::BadExperience => write!(f, "bad_experience"),
            Self::Other => write!(f, "other"),
            Self::Testing => write!(f, "testing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
