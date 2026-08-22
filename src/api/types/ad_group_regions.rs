pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdGroupRegions {
    /// Locations excluded from targeting. Country groups can't be excluded.
    #[serde(default)]
    pub exclude: AdGroupGeoLocations,
    /// Locations the ad group targets.
    #[serde(default)]
    pub include: AdGroupGeoLocations,
}

impl AdGroupRegions {
    pub fn builder() -> AdGroupRegionsBuilder {
        <AdGroupRegionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupRegionsBuilder {
    exclude: Option<AdGroupGeoLocations>,
    include: Option<AdGroupGeoLocations>,
}

impl AdGroupRegionsBuilder {
    pub fn exclude(mut self, value: AdGroupGeoLocations) -> Self {
        self.exclude = Some(value);
        self
    }

    pub fn include(mut self, value: AdGroupGeoLocations) -> Self {
        self.include = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupRegions`].
    /// This method will fail if any of the following fields are not set:
    /// - [`exclude`](AdGroupRegionsBuilder::exclude)
    /// - [`include`](AdGroupRegionsBuilder::include)
    pub fn build(self) -> Result<AdGroupRegions, BuildError> {
        Ok(AdGroupRegions {
            exclude: self
                .exclude
                .ok_or_else(|| BuildError::missing_field("exclude"))?,
            include: self
                .include
                .ok_or_else(|| BuildError::missing_field("include"))?,
        })
    }
}
