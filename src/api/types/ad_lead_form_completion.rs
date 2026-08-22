pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdLeadFormCompletion {
    /// Text of the follow-up button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text: Option<String>,
    /// Body text under the headline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Headline of the completion screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    /// Website the follow-up button opens. `null` when the screen has no button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AdLeadFormCompletion {
    pub fn builder() -> AdLeadFormCompletionBuilder {
        <AdLeadFormCompletionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdLeadFormCompletionBuilder {
    button_text: Option<String>,
    description: Option<String>,
    headline: Option<String>,
    url: Option<String>,
}

impl AdLeadFormCompletionBuilder {
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

    /// Consumes the builder and constructs a [`AdLeadFormCompletion`].
    pub fn build(self) -> Result<AdLeadFormCompletion, BuildError> {
        Ok(AdLeadFormCompletion {
            button_text: self.button_text,
            description: self.description,
            headline: self.headline,
            url: self.url,
        })
    }
}
