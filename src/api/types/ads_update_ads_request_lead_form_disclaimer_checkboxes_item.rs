pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdsRequestLeadFormDisclaimerCheckboxesItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_by_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl UpdateAdsRequestLeadFormDisclaimerCheckboxesItem {
    pub fn builder() -> UpdateAdsRequestLeadFormDisclaimerCheckboxesItemBuilder {
        <UpdateAdsRequestLeadFormDisclaimerCheckboxesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestLeadFormDisclaimerCheckboxesItemBuilder {
    checked_by_default: Option<bool>,
    key: Option<String>,
    required: Option<bool>,
    text: Option<String>,
}

impl UpdateAdsRequestLeadFormDisclaimerCheckboxesItemBuilder {
    pub fn checked_by_default(mut self, value: bool) -> Self {
        self.checked_by_default = Some(value);
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestLeadFormDisclaimerCheckboxesItem`].
    pub fn build(self) -> Result<UpdateAdsRequestLeadFormDisclaimerCheckboxesItem, BuildError> {
        Ok(UpdateAdsRequestLeadFormDisclaimerCheckboxesItem {
            checked_by_default: self.checked_by_default,
            key: self.key,
            required: self.required,
            text: self.text,
        })
    }
}
