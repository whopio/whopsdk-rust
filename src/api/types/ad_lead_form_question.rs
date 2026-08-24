pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdLeadFormQuestion {
    /// Answer format for `custom` questions: `short_answer`, `multiple_choice`, or `appointment`. Absent otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Question text for `custom` questions. Absent for standard prefill questions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<AdLeadFormQuestionOption>>,
    /// Question type: a standard prefill type such as `email`, `phone`, or `full_name`, or `custom` for your own question.
    #[serde(default)]
    pub r#type: String,
}

impl AdLeadFormQuestion {
    pub fn builder() -> AdLeadFormQuestionBuilder {
        <AdLeadFormQuestionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdLeadFormQuestionBuilder {
    format: Option<String>,
    label: Option<String>,
    options: Option<Vec<AdLeadFormQuestionOption>>,
    r#type: Option<String>,
}

impl AdLeadFormQuestionBuilder {
    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.format = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn options(mut self, value: Vec<AdLeadFormQuestionOption>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdLeadFormQuestion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](AdLeadFormQuestionBuilder::r#type)
    pub fn build(self) -> Result<AdLeadFormQuestion, BuildError> {
        Ok(AdLeadFormQuestion {
            format: self.format,
            label: self.label,
            options: self.options,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
