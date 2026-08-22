pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdGroupOperatingSystem {
    /// Lowest OS version targeted, such as `18.0`. Absent when any version qualifies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_version: Option<String>,
    /// Operating system targeted.
    pub os: AdGroupOperatingSystemOs,
}

impl AdGroupOperatingSystem {
    pub fn builder() -> AdGroupOperatingSystemBuilder {
        <AdGroupOperatingSystemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupOperatingSystemBuilder {
    minimum_version: Option<String>,
    os: Option<AdGroupOperatingSystemOs>,
}

impl AdGroupOperatingSystemBuilder {
    pub fn minimum_version(mut self, value: impl Into<String>) -> Self {
        self.minimum_version = Some(value.into());
        self
    }

    pub fn os(mut self, value: AdGroupOperatingSystemOs) -> Self {
        self.os = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupOperatingSystem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`os`](AdGroupOperatingSystemBuilder::os)
    pub fn build(self) -> Result<AdGroupOperatingSystem, BuildError> {
        Ok(AdGroupOperatingSystem {
            minimum_version: self.minimum_version,
            os: self.os.ok_or_else(|| BuildError::missing_field("os"))?,
        })
    }
}
