pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestLeadFormQuestionsItemOptionsItemLogic {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<CreateAdsRequestLeadFormQuestionsItemOptionsItemLogicAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_end_page_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_question_index: Option<i64>,
}

impl CreateAdsRequestLeadFormQuestionsItemOptionsItemLogic {
    pub fn builder() -> CreateAdsRequestLeadFormQuestionsItemOptionsItemLogicBuilder {
        <CreateAdsRequestLeadFormQuestionsItemOptionsItemLogicBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestLeadFormQuestionsItemOptionsItemLogicBuilder {
    action: Option<CreateAdsRequestLeadFormQuestionsItemOptionsItemLogicAction>,
    target_end_page_index: Option<i64>,
    target_question_index: Option<i64>,
}

impl CreateAdsRequestLeadFormQuestionsItemOptionsItemLogicBuilder {
    pub fn action(
        mut self,
        value: CreateAdsRequestLeadFormQuestionsItemOptionsItemLogicAction,
    ) -> Self {
        self.action = Some(value);
        self
    }

    pub fn target_end_page_index(mut self, value: i64) -> Self {
        self.target_end_page_index = Some(value);
        self
    }

    pub fn target_question_index(mut self, value: i64) -> Self {
        self.target_question_index = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestLeadFormQuestionsItemOptionsItemLogic`].
    pub fn build(
        self,
    ) -> Result<CreateAdsRequestLeadFormQuestionsItemOptionsItemLogic, BuildError> {
        Ok(CreateAdsRequestLeadFormQuestionsItemOptionsItemLogic {
            action: self.action,
            target_end_page_index: self.target_end_page_index,
            target_question_index: self.target_question_index,
        })
    }
}
