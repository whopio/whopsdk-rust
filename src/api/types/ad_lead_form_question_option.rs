pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdLeadFormQuestionOption {
    /// Stable identifier the choice's answers are stored under. Absent for simple choices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Where the form goes when this choice is selected. Absent when the form just continues to the next question.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic: Option<AdLeadFormOptionLogic>,
    /// Choice text shown to the person.
    #[serde(default)]
    pub value: String,
}

impl AdLeadFormQuestionOption {
    pub fn builder() -> AdLeadFormQuestionOptionBuilder {
        <AdLeadFormQuestionOptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdLeadFormQuestionOptionBuilder {
    key: Option<String>,
    logic: Option<AdLeadFormOptionLogic>,
    value: Option<String>,
}

impl AdLeadFormQuestionOptionBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn logic(mut self, value: AdLeadFormOptionLogic) -> Self {
        self.logic = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdLeadFormQuestionOption`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](AdLeadFormQuestionOptionBuilder::value)
    pub fn build(self) -> Result<AdLeadFormQuestionOption, BuildError> {
        Ok(AdLeadFormQuestionOption {
            key: self.key,
            logic: self.logic,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
