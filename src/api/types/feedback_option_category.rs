pub use crate::prelude::*;

/// The category saved as the membership's `cancel_option` when a member picks this option.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeedbackOptionCategory {
    TooExpensive,
    Switching,
    MissingFeatures,
    TechnicalIssues,
    BadExperience,
    Unused,
    Other,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FeedbackOptionCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::TooExpensive => serializer.serialize_str("too_expensive"),
            Self::Switching => serializer.serialize_str("switching"),
            Self::MissingFeatures => serializer.serialize_str("missing_features"),
            Self::TechnicalIssues => serializer.serialize_str("technical_issues"),
            Self::BadExperience => serializer.serialize_str("bad_experience"),
            Self::Unused => serializer.serialize_str("unused"),
            Self::Other => serializer.serialize_str("other"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FeedbackOptionCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "too_expensive" => Ok(Self::TooExpensive),
            "switching" => Ok(Self::Switching),
            "missing_features" => Ok(Self::MissingFeatures),
            "technical_issues" => Ok(Self::TechnicalIssues),
            "bad_experience" => Ok(Self::BadExperience),
            "unused" => Ok(Self::Unused),
            "other" => Ok(Self::Other),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FeedbackOptionCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooExpensive => write!(f, "too_expensive"),
            Self::Switching => write!(f, "switching"),
            Self::MissingFeatures => write!(f, "missing_features"),
            Self::TechnicalIssues => write!(f, "technical_issues"),
            Self::BadExperience => write!(f, "bad_experience"),
            Self::Unused => write!(f, "unused"),
            Self::Other => write!(f, "other"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
