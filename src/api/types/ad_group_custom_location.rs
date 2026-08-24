pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdGroupCustomLocation {
    /// Unit for `radius`.
    pub distance_unit: AdGroupCustomLocationDistanceUnit,
    /// Latitude of the center point.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub latitude: f64,
    /// Longitude of the center point.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub longitude: f64,
    /// Label for the location, such as a city or address. Absent when the location has no label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Radius around the center point, in `distance_unit`.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub radius: f64,
}

impl AdGroupCustomLocation {
    pub fn builder() -> AdGroupCustomLocationBuilder {
        <AdGroupCustomLocationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupCustomLocationBuilder {
    distance_unit: Option<AdGroupCustomLocationDistanceUnit>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    name: Option<String>,
    radius: Option<f64>,
}

impl AdGroupCustomLocationBuilder {
    pub fn distance_unit(mut self, value: AdGroupCustomLocationDistanceUnit) -> Self {
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

    /// Consumes the builder and constructs a [`AdGroupCustomLocation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`distance_unit`](AdGroupCustomLocationBuilder::distance_unit)
    /// - [`latitude`](AdGroupCustomLocationBuilder::latitude)
    /// - [`longitude`](AdGroupCustomLocationBuilder::longitude)
    /// - [`radius`](AdGroupCustomLocationBuilder::radius)
    pub fn build(self) -> Result<AdGroupCustomLocation, BuildError> {
        Ok(AdGroupCustomLocation {
            distance_unit: self
                .distance_unit
                .ok_or_else(|| BuildError::missing_field("distance_unit"))?,
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
