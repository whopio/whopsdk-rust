pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestLeadFormQuestionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<CreateAdsRequestLeadFormQuestionsItemFormat>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CreateAdsRequestLeadFormQuestionsItemOptionsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<CreateAdsRequestLeadFormQuestionsItemType>,
}

impl CreateAdsRequestLeadFormQuestionsItem {
    pub fn builder() -> CreateAdsRequestLeadFormQuestionsItemBuilder {
        <CreateAdsRequestLeadFormQuestionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestLeadFormQuestionsItemBuilder {
    format: Option<CreateAdsRequestLeadFormQuestionsItemFormat>,
    label: Option<String>,
    options: Option<Vec<CreateAdsRequestLeadFormQuestionsItemOptionsItem>>,
    r#type: Option<CreateAdsRequestLeadFormQuestionsItemType>,
}

impl CreateAdsRequestLeadFormQuestionsItemBuilder {
    pub fn format(mut self, value: CreateAdsRequestLeadFormQuestionsItemFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn options(mut self, value: Vec<CreateAdsRequestLeadFormQuestionsItemOptionsItem>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn r#type(mut self, value: CreateAdsRequestLeadFormQuestionsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestLeadFormQuestionsItem`].
    pub fn build(self) -> Result<CreateAdsRequestLeadFormQuestionsItem, BuildError> {
        Ok(CreateAdsRequestLeadFormQuestionsItem {
            format: self.format,
            label: self.label,
            options: self.options,
            r#type: self.r#type,
        })
    }
}
