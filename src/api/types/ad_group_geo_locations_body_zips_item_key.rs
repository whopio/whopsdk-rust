pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct AdGroupGeoLocationsBodyZipsItemKey {
    /// The ad platform's key for the ZIP or postal code.
    pub key: String,
}

impl AdGroupGeoLocationsBodyZipsItemKey {
    pub fn builder() -> AdGroupGeoLocationsBodyZipsItemKeyBuilder {
        <AdGroupGeoLocationsBodyZipsItemKeyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupGeoLocationsBodyZipsItemKeyBuilder {
    key: Option<String>,
}

impl AdGroupGeoLocationsBodyZipsItemKeyBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdGroupGeoLocationsBodyZipsItemKey`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](AdGroupGeoLocationsBodyZipsItemKeyBuilder::key)
    pub fn build(self) -> Result<AdGroupGeoLocationsBodyZipsItemKey, BuildError> {
        Ok(AdGroupGeoLocationsBodyZipsItemKey {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
        })
    }
}
