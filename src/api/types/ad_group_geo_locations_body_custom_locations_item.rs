pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdGroupGeoLocationsBodyCustomLocationsItem {
    /// Unit for `radius`. Defaults to `mile`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub distance_unit: Option<AdGroupGeoLocationsBodyCustomLocationsItemDistanceUnit>,
    /// Latitude of the center point.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub latitude: f64,
    /// Longitude of the center point.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub longitude: f64,
    /// Label for the location, such as a city or address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Radius around the center point: 1-50 miles or 1-80 kilometers.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub radius: f64,
}

impl AdGroupGeoLocationsBodyCustomLocationsItem {
    pub fn builder() -> AdGroupGeoLocationsBodyCustomLocationsItemBuilder {
        <AdGroupGeoLocationsBodyCustomLocationsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupGeoLocationsBodyCustomLocationsItemBuilder {
    distance_unit: Option<AdGroupGeoLocationsBodyCustomLocationsItemDistanceUnit>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    name: Option<String>,
    radius: Option<f64>,
}

impl AdGroupGeoLocationsBodyCustomLocationsItemBuilder {
    pub fn distance_unit(
        mut self,
        value: AdGroupGeoLocationsBodyCustomLocationsItemDistanceUnit,
    ) -> Self {
        self.distance_unit = Some(value);
        self
    }

    pub fn latitude(mut self, value: f64) -> Self {
        self.latitude = Some(value);
        self
    }

    pub fn longitude(mut self, value: f64) -> Self {
        self.longitude = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn radius(mut self, value: f64) -> Self {
        self.radius = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupGeoLocationsBodyCustomLocationsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`latitude`](AdGroupGeoLocationsBodyCustomLocationsItemBuilder::latitude)
    /// - [`longitude`](AdGroupGeoLocationsBodyCustomLocationsItemBuilder::longitude)
    /// - [`radius`](AdGroupGeoLocationsBodyCustomLocationsItemBuilder::radius)
    pub fn build(self) -> Result<AdGroupGeoLocationsBodyCustomLocationsItem, BuildError> {
        Ok(AdGroupGeoLocationsBodyCustomLocationsItem {
            distance_unit: self.distance_unit,
            latitude: self
                .latitude
                .ok_or_else(|| BuildError::missing_field("latitude"))?,
            longitude: self
                .longitude
                .ok_or_else(|| BuildError::missing_field("longitude"))?,
            name: self.name,
            radius: self
                .radius
                .ok_or_else(|| BuildError::missing_field("radius"))?,
        })
    }
}
