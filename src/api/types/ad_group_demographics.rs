pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdGroupDemographics {
    /// Whether automatic audience targeting is on (Advantage+ on Meta). When `true`, the platform can deliver beyond the ages, genders, and detailed targeting you set, treating them as suggestions.
    #[serde(default)]
    pub automatic: bool,
    /// Gender targeted.
    pub gender: AdGroupDemographicsGender,
    /// Oldest age targeted. `null` when no maximum is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub maximum_age: Option<f64>,
    /// Youngest age targeted. `null` when no minimum is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub minimum_age: Option<f64>,
}

impl AdGroupDemographics {
    pub fn builder() -> AdGroupDemographicsBuilder {
        <AdGroupDemographicsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupDemographicsBuilder {
    automatic: Option<bool>,
    gender: Option<AdGroupDemographicsGender>,
    maximum_age: Option<f64>,
    minimum_age: Option<f64>,
}

impl AdGroupDemographicsBuilder {
    pub fn automatic(mut self, value: bool) -> Self {
        self.automatic = Some(value);
        self
    }

    pub fn gender(mut self, value: AdGroupDemographicsGender) -> Self {
        self.gender = Some(value);
        self
    }

    pub fn maximum_age(mut self, value: f64) -> Self {
        self.maximum_age = Some(value);
        self
    }

    pub fn minimum_age(mut self, value: f64) -> Self {
        self.minimum_age = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupDemographics`].
    /// This method will fail if any of the following fields are not set:
    /// - [`automatic`](AdGroupDemographicsBuilder::automatic)
    /// - [`gender`](AdGroupDemographicsBuilder::gender)
    pub fn build(self) -> Result<AdGroupDemographics, BuildError> {
        Ok(AdGroupDemographics {
            automatic: self
                .automatic
                .ok_or_else(|| BuildError::missing_field("automatic"))?,
            gender: self
                .gender
                .ok_or_else(|| BuildError::missing_field("gender"))?,
            maximum_age: self.maximum_age,
            minimum_age: self.minimum_age,
        })
    }
}
