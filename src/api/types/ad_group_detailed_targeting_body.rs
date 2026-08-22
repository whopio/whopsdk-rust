pub use crate::prelude::*;

/// Interest, behavior, and demographic targeting, using categories from the ad platform's targeting taxonomy. Entries across interests, behaviors, and demographics are OR'd together (anyone matching any entry is reached), matching Ads Manager's detailed-targeting box. At most 100 entries per section. Can't be combined with demographics.automatic, and unavailable to campaigns with special_ad_categories.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupDetailedTargetingBody {
    /// Behavior categories to target, such as frequent travelers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub behaviors: Option<Vec<AdGroupDetailedTargetingBodyBehaviorsItem>>,
    /// Demographic categories to target, such as life events, industries, work employers, job titles, schools, or majors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub demographics: Option<Vec<AdGroupDetailedTargetingBodyDemographicsItem>>,
    /// Interest categories to target, such as an interest in movies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interests: Option<Vec<AdGroupDetailedTargetingBodyInterestsItem>>,
}

impl AdGroupDetailedTargetingBody {
    pub fn builder() -> AdGroupDetailedTargetingBodyBuilder {
        <AdGroupDetailedTargetingBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupDetailedTargetingBodyBuilder {
    behaviors: Option<Vec<AdGroupDetailedTargetingBodyBehaviorsItem>>,
    demographics: Option<Vec<AdGroupDetailedTargetingBodyDemographicsItem>>,
    interests: Option<Vec<AdGroupDetailedTargetingBodyInterestsItem>>,
}

impl AdGroupDetailedTargetingBodyBuilder {
    pub fn behaviors(mut self, value: Vec<AdGroupDetailedTargetingBodyBehaviorsItem>) -> Self {
        self.behaviors = Some(value);
        self
    }

    pub fn demographics(
        mut self,
        value: Vec<AdGroupDetailedTargetingBodyDemographicsItem>,
    ) -> Self {
        self.demographics = Some(value);
        self
    }

    pub fn interests(mut self, value: Vec<AdGroupDetailedTargetingBodyInterestsItem>) -> Self {
        self.interests = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupDetailedTargetingBody`].
    pub fn build(self) -> Result<AdGroupDetailedTargetingBody, BuildError> {
        Ok(AdGroupDetailedTargetingBody {
            behaviors: self.behaviors,
            demographics: self.demographics,
            interests: self.interests,
        })
    }
}
