pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdGroupDetailedTargeting {
    #[serde(default)]
    pub behaviors: Vec<AdGroupBehaviorCategory>,
    #[serde(default)]
    pub demographics: Vec<AdGroupDemographicCategory>,
    #[serde(default)]
    pub interests: Vec<AdGroupTargetingCategory>,
}

impl AdGroupDetailedTargeting {
    pub fn builder() -> AdGroupDetailedTargetingBuilder {
        <AdGroupDetailedTargetingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupDetailedTargetingBuilder {
    behaviors: Option<Vec<AdGroupBehaviorCategory>>,
    demographics: Option<Vec<AdGroupDemographicCategory>>,
    interests: Option<Vec<AdGroupTargetingCategory>>,
}

impl AdGroupDetailedTargetingBuilder {
    pub fn behaviors(mut self, value: Vec<AdGroupBehaviorCategory>) -> Self {
        self.behaviors = Some(value);
        self
    }

    pub fn demographics(mut self, value: Vec<AdGroupDemographicCategory>) -> Self {
        self.demographics = Some(value);
        self
    }

    pub fn interests(mut self, value: Vec<AdGroupTargetingCategory>) -> Self {
        self.interests = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupDetailedTargeting`].
    /// This method will fail if any of the following fields are not set:
    /// - [`behaviors`](AdGroupDetailedTargetingBuilder::behaviors)
    /// - [`demographics`](AdGroupDetailedTargetingBuilder::demographics)
    /// - [`interests`](AdGroupDetailedTargetingBuilder::interests)
    pub fn build(self) -> Result<AdGroupDetailedTargeting, BuildError> {
        Ok(AdGroupDetailedTargeting {
            behaviors: self
                .behaviors
                .ok_or_else(|| BuildError::missing_field("behaviors"))?,
            demographics: self
                .demographics
                .ok_or_else(|| BuildError::missing_field("demographics"))?,
            interests: self
                .interests
                .ok_or_else(|| BuildError::missing_field("interests"))?,
        })
    }
}
