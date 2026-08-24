pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdLeadFormDisclaimer {
    /// Disclaimer text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default)]
    pub checkboxes: Vec<AdLeadFormDisclaimerCheckbox>,
    /// Disclaimer title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl AdLeadFormDisclaimer {
    pub fn builder() -> AdLeadFormDisclaimerBuilder {
        <AdLeadFormDisclaimerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdLeadFormDisclaimerBuilder {
    body: Option<String>,
    checkboxes: Option<Vec<AdLeadFormDisclaimerCheckbox>>,
    title: Option<String>,
}

impl AdLeadFormDisclaimerBuilder {
    pub fn body(mut self, value: impl Into<String>) -> Self {
        self.body = Some(value.into());
        self
    }

    pub fn checkboxes(mut self, value: Vec<AdLeadFormDisclaimerCheckbox>) -> Self {
        self.checkboxes = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdLeadFormDisclaimer`].
    /// This method will fail if any of the following fields are not set:
    /// - [`checkboxes`](AdLeadFormDisclaimerBuilder::checkboxes)
    pub fn build(self) -> Result<AdLeadFormDisclaimer, BuildError> {
        Ok(AdLeadFormDisclaimer {
            body: self.body,
            checkboxes: self
                .checkboxes
                .ok_or_else(|| BuildError::missing_field("checkboxes"))?,
            title: self.title,
        })
    }
}
