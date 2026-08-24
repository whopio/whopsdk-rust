pub use crate::prelude::*;

/// Optional custom consent disclaimer with checkboxes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdsRequestLeadFormDisclaimer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkboxes: Option<Vec<UpdateAdsRequestLeadFormDisclaimerCheckboxesItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl UpdateAdsRequestLeadFormDisclaimer {
    pub fn builder() -> UpdateAdsRequestLeadFormDisclaimerBuilder {
        <UpdateAdsRequestLeadFormDisclaimerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestLeadFormDisclaimerBuilder {
    body: Option<String>,
    checkboxes: Option<Vec<UpdateAdsRequestLeadFormDisclaimerCheckboxesItem>>,
    title: Option<String>,
}

impl UpdateAdsRequestLeadFormDisclaimerBuilder {
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn checkboxes(
        mut self,
        value: Vec<UpdateAdsRequestLeadFormDisclaimerCheckboxesItem>,
    ) -> Self {
        self.checkboxes = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestLeadFormDisclaimer`].
    pub fn build(self) -> Result<UpdateAdsRequestLeadFormDisclaimer, BuildError> {
        Ok(UpdateAdsRequestLeadFormDisclaimer {
            body: self.body,
            checkboxes: self.checkboxes,
            title: self.title,
        })
    }
}
