pub use crate::prelude::*;

/// Device platforms and operating systems to target.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupDevicesBody {
    /// Operating systems to target. Empty targets all operating systems.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operating_systems: Option<Vec<AdGroupDevicesBodyOperatingSystemsItem>>,
    /// Device types to target. Empty targets all devices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platforms: Option<Vec<AdGroupDevicesBodyPlatformsItem>>,
}

impl AdGroupDevicesBody {
    pub fn builder() -> AdGroupDevicesBodyBuilder {
        <AdGroupDevicesBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupDevicesBodyBuilder {
    operating_systems: Option<Vec<AdGroupDevicesBodyOperatingSystemsItem>>,
    platforms: Option<Vec<AdGroupDevicesBodyPlatformsItem>>,
}

impl AdGroupDevicesBodyBuilder {
    pub fn operating_systems(mut self, value: Vec<AdGroupDevicesBodyOperatingSystemsItem>) -> Self {
        self.operating_systems = Some(value);
        self
    }

    pub fn platforms(mut self, value: Vec<AdGroupDevicesBodyPlatformsItem>) -> Self {
        self.platforms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupDevicesBody`].
    pub fn build(self) -> Result<AdGroupDevicesBody, BuildError> {
        Ok(AdGroupDevicesBody {
            operating_systems: self.operating_systems,
            platforms: self.platforms,
        })
    }
}
