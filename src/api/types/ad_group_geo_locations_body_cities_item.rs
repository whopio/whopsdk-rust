pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdGroupGeoLocationsBodyCitiesItem {
    /// The ad platform's key for the city in its location taxonomy.
    #[serde(default)]
    pub key: String,
    /// City name, such as `Austin`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl AdGroupGeoLocationsBodyCitiesItem {
    pub fn builder() -> AdGroupGeoLocationsBodyCitiesItemBuilder {
        <AdGroupGeoLocationsBodyCitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupGeoLocationsBodyCitiesItemBuilder {
    key: Option<String>,
    name: Option<String>,
}

impl AdGroupGeoLocationsBodyCitiesItemBuilder {
    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdGroupGeoLocationsBodyCitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`key`](AdGroupGeoLocationsBodyCitiesItemBuilder::key)
    pub fn build(self) -> Result<AdGroupGeoLocationsBodyCitiesItem, BuildError> {
        Ok(AdGroupGeoLocationsBodyCitiesItem {
            key: self.key.ok_or_else(|| BuildError::missing_field("key"))?,
            name: self.name,
        })
    }
}
