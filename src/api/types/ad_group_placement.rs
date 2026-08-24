pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdGroupPlacement {
    /// Publisher platform where the ad is eligible to appear.
    pub platform: AdGroupPlacementPlatform,
    #[serde(default)]
    pub positions: Vec<String>,
}

impl AdGroupPlacement {
    pub fn builder() -> AdGroupPlacementBuilder {
        <AdGroupPlacementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupPlacementBuilder {
    platform: Option<AdGroupPlacementPlatform>,
    positions: Option<Vec<String>>,
}

impl AdGroupPlacementBuilder {
    pub fn platform(mut self, value: AdGroupPlacementPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn positions(mut self, value: Vec<String>) -> Self {
        self.positions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupPlacement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](AdGroupPlacementBuilder::platform)
    /// - [`positions`](AdGroupPlacementBuilder::positions)
    pub fn build(self) -> Result<AdGroupPlacement, BuildError> {
        Ok(AdGroupPlacement {
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            positions: self
                .positions
                .ok_or_else(|| BuildError::missing_field("positions"))?,
        })
    }
}
