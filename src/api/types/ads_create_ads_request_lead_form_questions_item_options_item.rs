pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestLeadFormQuestionsItemOptionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic: Option<CreateAdsRequestLeadFormQuestionsItemOptionsItemLogic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl CreateAdsRequestLeadFormQuestionsItemOptionsItem {
    pub fn builder() -> CreateAdsRequestLeadFormQuestionsItemOptionsItemBuilder {
        <CreateAdsRequestLeadFormQuestionsItemOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestLeadFormQuestionsItemOptionsItemBuilder {
    key: Option<String>,
    logic: Option<CreateAdsRequestLeadFormQuestionsItemOptionsItemLogic>,
    value: Option<String>,
}

impl CreateAdsRequestLeadFormQuestionsItemOptionsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn logic(mut self, value: CreateAdsRequestLeadFormQuestionsItemOptionsItemLogic) -> Self {
        self.logic = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestLeadFormQuestionsItemOptionsItem`].
    pub fn build(self) -> Result<CreateAdsRequestLeadFormQuestionsItemOptionsItem, BuildError> {
        Ok(CreateAdsRequestLeadFormQuestionsItemOptionsItem {
            key: self.key,
            logic: self.logic,
            value: self.value,
        })
    }
}
