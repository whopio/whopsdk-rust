pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdGroupGeoLocations {
    #[serde(default)]
    pub cities: Vec<AdGroupCity>,
    #[serde(default)]
    pub countries: Vec<String>,
    #[serde(default)]
    pub country_groups: Vec<String>,
    #[serde(default)]
    pub custom_locations: Vec<AdGroupCustomLocation>,
    #[serde(default)]
    pub regions: Vec<String>,
    #[serde(default)]
    pub zips: Vec<String>,
}

impl AdGroupGeoLocations {
    pub fn builder() -> AdGroupGeoLocationsBuilder {
        <AdGroupGeoLocationsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupGeoLocationsBuilder {
    cities: Option<Vec<AdGroupCity>>,
    countries: Option<Vec<String>>,
    country_groups: Option<Vec<String>>,
    custom_locations: Option<Vec<AdGroupCustomLocation>>,
    regions: Option<Vec<String>>,
    zips: Option<Vec<String>>,
}

impl AdGroupGeoLocationsBuilder {
    pub fn cities(mut self, value: Vec<AdGroupCity>) -> Self {
        self.cities = Some(value);
        self
    }

    pub fn countries(mut self, value: Vec<String>) -> Self {
        self.countries = Some(value);
        self
    }

    pub fn country_groups(mut self, value: Vec<String>) -> Self {
        self.country_groups = Some(value);
        self
    }

    pub fn custom_locations(mut self, value: Vec<AdGroupCustomLocation>) -> Self {
        self.custom_locations = Some(value);
        self
    }

    pub fn regions(mut self, value: Vec<String>) -> Self {
        self.regions = Some(value);
        self
    }

    pub fn zips(mut self, value: Vec<String>) -> Self {
        self.zips = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupGeoLocations`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cities`](AdGroupGeoLocationsBuilder::cities)
    /// - [`countries`](AdGroupGeoLocationsBuilder::countries)
    /// - [`country_groups`](AdGroupGeoLocationsBuilder::country_groups)
    /// - [`custom_locations`](AdGroupGeoLocationsBuilder::custom_locations)
    /// - [`regions`](AdGroupGeoLocationsBuilder::regions)
    /// - [`zips`](AdGroupGeoLocationsBuilder::zips)
    pub fn build(self) -> Result<AdGroupGeoLocations, BuildError> {
        Ok(AdGroupGeoLocations {
            cities: self
                .cities
                .ok_or_else(|| BuildError::missing_field("cities"))?,
            countries: self
                .countries
                .ok_or_else(|| BuildError::missing_field("countries"))?,
            country_groups: self
                .country_groups
                .ok_or_else(|| BuildError::missing_field("country_groups"))?,
            custom_locations: self
                .custom_locations
                .ok_or_else(|| BuildError::missing_field("custom_locations"))?,
            regions: self
                .regions
                .ok_or_else(|| BuildError::missing_field("regions"))?,
            zips: self.zips.ok_or_else(|| BuildError::missing_field("zips"))?,
        })
    }
}
