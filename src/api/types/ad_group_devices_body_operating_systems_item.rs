pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdGroupDevicesBodyOperatingSystemsItem {
    /// Lowest OS version to target, such as `18.0`. Omit to target any version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_version: Option<String>,
    /// Operating system to target.
    pub os: AdGroupDevicesBodyOperatingSystemsItemOs,
}

impl AdGroupDevicesBodyOperatingSystemsItem {
    pub fn builder() -> AdGroupDevicesBodyOperatingSystemsItemBuilder {
        <AdGroupDevicesBodyOperatingSystemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupDevicesBodyOperatingSystemsItemBuilder {
    minimum_version: Option<String>,
    os: Option<AdGroupDevicesBodyOperatingSystemsItemOs>,
}

impl AdGroupDevicesBodyOperatingSystemsItemBuilder {
    pub fn minimum_version(mut self, value: impl Into<String>) -> Self {
        self.minimum_version = Some(value.into());
        self
    }

    pub fn os(mut self, value: AdGroupDevicesBodyOperatingSystemsItemOs) -> Self {
        self.os = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupDevicesBodyOperatingSystemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`os`](AdGroupDevicesBodyOperatingSystemsItemBuilder::os)
    pub fn build(self) -> Result<AdGroupDevicesBodyOperatingSystemsItem, BuildError> {
        Ok(AdGroupDevicesBodyOperatingSystemsItem {
            minimum_version: self.minimum_version,
            os: self.os.ok_or_else(|| BuildError::missing_field("os"))?,
        })
    }
}
