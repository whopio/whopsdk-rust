pub use crate::prelude::*;

/// Optional completion screen shown after submission; url sets the follow-up website button.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestLeadFormCompletion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl CreateAdsRequestLeadFormCompletion {
    pub fn builder() -> CreateAdsRequestLeadFormCompletionBuilder {
        <CreateAdsRequestLeadFormCompletionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestLeadFormCompletionBuilder {
    button_text: Option<String>,
    description: Option<String>,
    headline: Option<String>,
    url: Option<String>,
}

impl CreateAdsRequestLeadFormCompletionBuilder {
    pub fn button_text(mut self, value: impl Into<String>) -> Self {
        self.button_text = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn headline(mut self, value: impl Into<String>) -> Self {
        self.headline = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestLeadFormCompletion`].
    pub fn build(self) -> Result<CreateAdsRequestLeadFormCompletion, BuildError> {
        Ok(CreateAdsRequestLeadFormCompletion {
            button_text: self.button_text,
            description: self.description,
            headline: self.headline,
            url: self.url,
        })
    }
}
