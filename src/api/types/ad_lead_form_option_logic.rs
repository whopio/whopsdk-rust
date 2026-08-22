pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdLeadFormOptionLogic {
    /// What happens when the choice is selected.
    pub action: AdLeadFormOptionLogicAction,
    /// Zero-based index of the ending screen to jump to.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub target_end_page_index: Option<f64>,
    /// Zero-based index of the question to jump to, for `go_to_question`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub target_question_index: Option<f64>,
}

impl AdLeadFormOptionLogic {
    pub fn builder() -> AdLeadFormOptionLogicBuilder {
        <AdLeadFormOptionLogicBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdLeadFormOptionLogicBuilder {
    action: Option<AdLeadFormOptionLogicAction>,
    target_end_page_index: Option<f64>,
    target_question_index: Option<f64>,
}

impl AdLeadFormOptionLogicBuilder {
    pub fn action(mut self, value: AdLeadFormOptionLogicAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn target_end_page_index(mut self, value: f64) -> Self {
        self.target_end_page_index = Some(value);
        self
    }

    pub fn target_question_index(mut self, value: f64) -> Self {
        self.target_question_index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdLeadFormOptionLogic`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](AdLeadFormOptionLogicBuilder::action)
    pub fn build(self) -> Result<AdLeadFormOptionLogic, BuildError> {
        Ok(AdLeadFormOptionLogic {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            target_end_page_index: self.target_end_page_index,
            target_question_index: self.target_question_index,
        })
    }
}
