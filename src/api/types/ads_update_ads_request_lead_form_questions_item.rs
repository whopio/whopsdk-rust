pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdsRequestLeadFormQuestionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<UpdateAdsRequestLeadFormQuestionsItemFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<UpdateAdsRequestLeadFormQuestionsItemOptionsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<UpdateAdsRequestLeadFormQuestionsItemType>,
}

impl UpdateAdsRequestLeadFormQuestionsItem {
    pub fn builder() -> UpdateAdsRequestLeadFormQuestionsItemBuilder {
        <UpdateAdsRequestLeadFormQuestionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestLeadFormQuestionsItemBuilder {
    format: Option<UpdateAdsRequestLeadFormQuestionsItemFormat>,
    label: Option<String>,
    options: Option<Vec<UpdateAdsRequestLeadFormQuestionsItemOptionsItem>>,
    r#type: Option<UpdateAdsRequestLeadFormQuestionsItemType>,
}

impl UpdateAdsRequestLeadFormQuestionsItemBuilder {
    pub fn format(mut self, value: UpdateAdsRequestLeadFormQuestionsItemFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn options(mut self, value: Vec<UpdateAdsRequestLeadFormQuestionsItemOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn r#type(mut self, value: UpdateAdsRequestLeadFormQuestionsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestLeadFormQuestionsItem`].
    pub fn build(self) -> Result<UpdateAdsRequestLeadFormQuestionsItem, BuildError> {
        Ok(UpdateAdsRequestLeadFormQuestionsItem {
            format: self.format,
            label: self.label,
            options: self.options,
            r#type: self.r#type,
        })
    }
}
