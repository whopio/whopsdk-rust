pub use crate::prelude::*;

/// Optional custom consent disclaimer with checkboxes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestLeadFormDisclaimer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkboxes: Option<Vec<CreateAdsRequestLeadFormDisclaimerCheckboxesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl CreateAdsRequestLeadFormDisclaimer {
    pub fn builder() -> CreateAdsRequestLeadFormDisclaimerBuilder {
        <CreateAdsRequestLeadFormDisclaimerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestLeadFormDisclaimerBuilder {
    body: Option<String>,
    checkboxes: Option<Vec<CreateAdsRequestLeadFormDisclaimerCheckboxesItem>>,
    title: Option<String>,
}

impl CreateAdsRequestLeadFormDisclaimerBuilder {
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn checkboxes(
        mut self,
        value: Vec<CreateAdsRequestLeadFormDisclaimerCheckboxesItem>,
    ) -> Self {
        self.checkboxes = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestLeadFormDisclaimer`].
    pub fn build(self) -> Result<CreateAdsRequestLeadFormDisclaimer, BuildError> {
        Ok(CreateAdsRequestLeadFormDisclaimer {
            body: self.body,
            checkboxes: self.checkboxes,
            title: self.title,
        })
    }
}
