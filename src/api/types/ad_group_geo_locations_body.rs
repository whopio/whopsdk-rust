pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AdGroupGeoLocationsBody {
    /// Cities, keyed by the ad platform's location taxonomy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cities: Option<Vec<AdGroupGeoLocationsBodyCitiesItem>>,
    /// Countries, as ISO 3166-1 alpha-2 codes such as `US`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries: Option<Vec<String>>,
    /// Multi-country groups such as `worldwide` or `europe`. Include-only — groups can't be excluded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_groups: Option<Vec<String>>,
    /// Circular areas, each a coordinate plus a radius. At most 200 across include and exclude.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_locations: Option<Vec<AdGroupGeoLocationsBodyCustomLocationsItem>>,
    /// US states and DC, as ISO 3166-2 codes such as `US-CA`. US territories (`PR`, `GU`, `VI`, `AS`, `MP`) and everywhere outside the US are targeted through `countries`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<Vec<String>>,
    /// ZIP and postal codes, as bare strings or objects with a key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zips: Option<Vec<AdGroupGeoLocationsBodyZipsItem>>,
}

impl AdGroupGeoLocationsBody {
    pub fn builder() -> AdGroupGeoLocationsBodyBuilder {
        <AdGroupGeoLocationsBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupGeoLocationsBodyBuilder {
    cities: Option<Vec<AdGroupGeoLocationsBodyCitiesItem>>,
    countries: Option<Vec<String>>,
    country_groups: Option<Vec<String>>,
    custom_locations: Option<Vec<AdGroupGeoLocationsBodyCustomLocationsItem>>,
    regions: Option<Vec<String>>,
    zips: Option<Vec<AdGroupGeoLocationsBodyZipsItem>>,
}

impl AdGroupGeoLocationsBodyBuilder {
    pub fn cities(mut self, value: Vec<AdGroupGeoLocationsBodyCitiesItem>) -> Self {
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

    pub fn custom_locations(
        mut self,
        value: Vec<AdGroupGeoLocationsBodyCustomLocationsItem>,
    ) -> Self {
        self.custom_locations = Some(value);
        self
    }

    pub fn regions(mut self, value: Vec<String>) -> Self {
        self.regions = Some(value);
        self
    }

    pub fn zips(mut self, value: Vec<AdGroupGeoLocationsBodyZipsItem>) -> Self {
        self.zips = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdGroupGeoLocationsBody`].
    pub fn build(self) -> Result<AdGroupGeoLocationsBody, BuildError> {
        Ok(AdGroupGeoLocationsBody {
            cities: self.cities,
            countries: self.countries,
            country_groups: self.country_groups,
            custom_locations: self.custom_locations,
            regions: self.regions,
            zips: self.zips,
        })
    }
}
