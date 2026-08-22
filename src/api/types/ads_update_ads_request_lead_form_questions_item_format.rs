pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAdsRequestLeadFormQuestionsItemFormat {
    ShortAnswer,
    MultipleChoice,
    Appointment,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAdsRequestLeadFormQuestionsItemFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ShortAnswer => serializer.serialize_str("short_answer"),
            Self::MultipleChoice => serializer.serialize_str("multiple_choice"),
            Self::Appointment => serializer.serialize_str("appointment"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAdsRequestLeadFormQuestionsItemFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "short_answer" => Ok(Self::ShortAnswer),
            "multiple_choice" => Ok(Self::MultipleChoice),
            "appointment" => Ok(Self::Appointment),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAdsRequestLeadFormQuestionsItemFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ShortAnswer => write!(f, "short_answer"),
            Self::MultipleChoice => write!(f, "multiple_choice"),
            Self::Appointment => write!(f, "appointment"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
