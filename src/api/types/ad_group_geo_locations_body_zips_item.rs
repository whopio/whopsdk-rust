pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AdGroupGeoLocationsBodyZipsItem {
    String(String),

    AdGroupGeoLocationsBodyZipsItemKey(AdGroupGeoLocationsBodyZipsItemKey),
}

impl AdGroupGeoLocationsBodyZipsItem {
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn is_ad_group_geo_locations_body_zips_item_key(&self) -> bool {
        matches!(self, Self::AdGroupGeoLocationsBodyZipsItemKey(_))
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_ad_group_geo_locations_body_zips_item_key(
        &self,
    ) -> Option<&AdGroupGeoLocationsBodyZipsItemKey> {
        match self {
            Self::AdGroupGeoLocationsBodyZipsItemKey(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ad_group_geo_locations_body_zips_item_key(
        self,
    ) -> Option<AdGroupGeoLocationsBodyZipsItemKey> {
        match self {
            Self::AdGroupGeoLocationsBodyZipsItemKey(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AdGroupGeoLocationsBodyZipsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::String(value) => write!(f, "{}", value),
            Self::AdGroupGeoLocationsBodyZipsItemKey(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
