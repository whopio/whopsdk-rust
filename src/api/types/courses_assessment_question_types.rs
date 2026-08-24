pub use crate::prelude::*;

/// The available types for an assessment question
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CoursesAssessmentQuestionTypes {
    ShortAnswer,
    TrueFalse,
    MultipleChoice,
    MultipleSelect,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CoursesAssessmentQuestionTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ShortAnswer => serializer.serialize_str("short_answer"),
            Self::TrueFalse => serializer.serialize_str("true_false"),
            Self::MultipleChoice => serializer.serialize_str("multiple_choice"),
            Self::MultipleSelect => serializer.serialize_str("multiple_select"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CoursesAssessmentQuestionTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "short_answer" => Ok(Self::ShortAnswer),
            "true_false" => Ok(Self::TrueFalse),
            "multiple_choice" => Ok(Self::MultipleChoice),
            "multiple_select" => Ok(Self::MultipleSelect),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CoursesAssessmentQuestionTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShortAnswer => write!(f, "short_answer"),
            Self::TrueFalse => write!(f, "true_false"),
            Self::MultipleChoice => write!(f, "multiple_choice"),
            Self::MultipleSelect => write!(f, "multiple_select"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
