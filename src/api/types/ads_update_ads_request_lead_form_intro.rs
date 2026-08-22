pub use crate::prelude::*;

/// Optional intro screen shown before the questions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdsRequestLeadFormIntro {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
}

impl UpdateAdsRequestLeadFormIntro {
    pub fn builder() -> UpdateAdsRequestLeadFormIntroBuilder {
        <UpdateAdsRequestLeadFormIntroBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestLeadFormIntroBuilder {
    description: Option<String>,
    headline: Option<String>,
}

impl UpdateAdsRequestLeadFormIntroBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn headline(mut self, value: impl Into<String>) -> Self {
        self.headline = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestLeadFormIntro`].
    pub fn build(self) -> Result<UpdateAdsRequestLeadFormIntro, BuildError> {
        Ok(UpdateAdsRequestLeadFormIntro {
            description: self.description,
            headline: self.headline,
        })
    }
}
