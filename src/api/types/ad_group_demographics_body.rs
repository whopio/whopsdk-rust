pub use crate::prelude::*;

/// Age, gender, and automatic-audience targeting.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupDemographicsBody {
    /// Turn on automatic audience targeting (Advantage+ on Meta): the platform can deliver beyond the ages, genders, and detailed targeting you set, treating them as suggestions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic: Option<bool>,
    /// Gender to target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<AdGroupDemographicsBodyGender>,
    /// Oldest age to target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_age: Option<i64>,
    /// Youngest age to target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_age: Option<i64>,
}

impl AdGroupDemographicsBody {
    pub fn builder() -> AdGroupDemographicsBodyBuilder {
        <AdGroupDemographicsBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupDemographicsBodyBuilder {
    automatic: Option<bool>,
    gender: Option<AdGroupDemographicsBodyGender>,
    maximum_age: Option<i64>,
    minimum_age: Option<i64>,
}

impl AdGroupDemographicsBodyBuilder {
    pub fn automatic(mut self, value: bool) -> Self {
        self.automatic = Some(value);
        self
    }

    pub fn gender(mut self, value: AdGroupDemographicsBodyGender) -> Self {
        self.gender = Some(value);
        self
    }

    pub fn maximum_age(mut self, value: i64) -> Self {
        self.maximum_age = Some(value);
        self
    }

    pub fn minimum_age(mut self, value: i64) -> Self {
        self.minimum_age = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupDemographicsBody`].
    pub fn build(self) -> Result<AdGroupDemographicsBody, BuildError> {
        Ok(AdGroupDemographicsBody {
            automatic: self.automatic,
            gender: self.gender,
            maximum_age: self.maximum_age,
            minimum_age: self.minimum_age,
        })
    }
}
