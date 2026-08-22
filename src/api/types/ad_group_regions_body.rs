pub use crate::prelude::*;

/// Locations to target and exclude.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdGroupRegionsBody {
    /// Locations excluded from targeting. Country groups can't be excluded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<AdGroupGeoLocationsBody>,
    /// Locations the ad group targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<AdGroupGeoLocationsBody>,
}

impl AdGroupRegionsBody {
    pub fn builder() -> AdGroupRegionsBodyBuilder {
        <AdGroupRegionsBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupRegionsBodyBuilder {
    exclude: Option<AdGroupGeoLocationsBody>,
    include: Option<AdGroupGeoLocationsBody>,
}

impl AdGroupRegionsBodyBuilder {
    pub fn exclude(mut self, value: AdGroupGeoLocationsBody) -> Self {
        self.exclude = Some(value);
        self
    }

    pub fn include(mut self, value: AdGroupGeoLocationsBody) -> Self {
        self.include = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupRegionsBody`].
    pub fn build(self) -> Result<AdGroupRegionsBody, BuildError> {
        Ok(AdGroupRegionsBody {
            exclude: self.exclude,
            include: self.include,
        })
    }
}
