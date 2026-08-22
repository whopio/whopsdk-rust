pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupDevices {
    #[serde(default)]
    pub operating_systems: Vec<AdGroupOperatingSystem>,
    #[serde(default)]
    pub platforms: Vec<AdGroupDevicesPlatformsItem>,
}

impl AdGroupDevices {
    pub fn builder() -> AdGroupDevicesBuilder {
        <AdGroupDevicesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupDevicesBuilder {
    operating_systems: Option<Vec<AdGroupOperatingSystem>>,
    platforms: Option<Vec<AdGroupDevicesPlatformsItem>>,
}

impl AdGroupDevicesBuilder {
    pub fn operating_systems(mut self, value: Vec<AdGroupOperatingSystem>) -> Self {
        self.operating_systems = Some(value);
        self
    }

    pub fn platforms(mut self, value: Vec<AdGroupDevicesPlatformsItem>) -> Self {
        self.platforms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupDevices`].
    /// This method will fail if any of the following fields are not set:
    /// - [`operating_systems`](AdGroupDevicesBuilder::operating_systems)
    /// - [`platforms`](AdGroupDevicesBuilder::platforms)
    pub fn build(self) -> Result<AdGroupDevices, BuildError> {
        Ok(AdGroupDevices {
            operating_systems: self
                .operating_systems
                .ok_or_else(|| BuildError::missing_field("operating_systems"))?,
            platforms: self
                .platforms
                .ok_or_else(|| BuildError::missing_field("platforms"))?,
        })
    }
}
