pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdLeadFormIntro {
    /// Body text under the headline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Headline of the intro screen.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
}

impl AdLeadFormIntro {
    pub fn builder() -> AdLeadFormIntroBuilder {
        <AdLeadFormIntroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdLeadFormIntroBuilder {
    description: Option<String>,
    headline: Option<String>,
}

impl AdLeadFormIntroBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn headline(mut self, value: impl Into<String>) -> Self {
        self.headline = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdLeadFormIntro`].
    pub fn build(self) -> Result<AdLeadFormIntro, BuildError> {
        Ok(AdLeadFormIntro {
            description: self.description,
            headline: self.headline,
        })
    }
}
