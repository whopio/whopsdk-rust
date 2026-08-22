pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdateAdsRequestLeadFormQuestionsItemOptionsItemLogicAction {
    GoToQuestion,
    SubmitForm,
    CloseForm,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdateAdsRequestLeadFormQuestionsItemOptionsItemLogicAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::GoToQuestion => serializer.serialize_str("go_to_question"),
            Self::SubmitForm => serializer.serialize_str("submit_form"),
            Self::CloseForm => serializer.serialize_str("close_form"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdateAdsRequestLeadFormQuestionsItemOptionsItemLogicAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "go_to_question" => Ok(Self::GoToQuestion),
            "submit_form" => Ok(Self::SubmitForm),
            "close_form" => Ok(Self::CloseForm),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdateAdsRequestLeadFormQuestionsItemOptionsItemLogicAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GoToQuestion => write!(f, "go_to_question"),
            Self::SubmitForm => write!(f, "submit_form"),
            Self::CloseForm => write!(f, "close_form"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
