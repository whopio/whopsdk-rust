pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdsRequestLeadFormQuestionsItemOptionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logic: Option<UpdateAdsRequestLeadFormQuestionsItemOptionsItemLogic>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl UpdateAdsRequestLeadFormQuestionsItemOptionsItem {
    pub fn builder() -> UpdateAdsRequestLeadFormQuestionsItemOptionsItemBuilder {
        <UpdateAdsRequestLeadFormQuestionsItemOptionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestLeadFormQuestionsItemOptionsItemBuilder {
    key: Option<String>,
    logic: Option<UpdateAdsRequestLeadFormQuestionsItemOptionsItemLogic>,
    value: Option<String>,
}

impl UpdateAdsRequestLeadFormQuestionsItemOptionsItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn logic(mut self, value: UpdateAdsRequestLeadFormQuestionsItemOptionsItemLogic) -> Self {
        self.logic = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestLeadFormQuestionsItemOptionsItem`].
    pub fn build(self) -> Result<UpdateAdsRequestLeadFormQuestionsItemOptionsItem, BuildError> {
        Ok(UpdateAdsRequestLeadFormQuestionsItemOptionsItem {
            key: self.key,
            logic: self.logic,
            value: self.value,
        })
    }
}
