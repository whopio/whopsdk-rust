pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SearchTargetingOptionsAdGroupsRequestLocationTypesItem {
    Country,
    Region,
    City,
    Zip,
    Neighborhood,
    Subcity,
    MediumGeoArea,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SearchTargetingOptionsAdGroupsRequestLocationTypesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Country => serializer.serialize_str("country"),
            Self::Region => serializer.serialize_str("region"),
            Self::City => serializer.serialize_str("city"),
            Self::Zip => serializer.serialize_str("zip"),
            Self::Neighborhood => serializer.serialize_str("neighborhood"),
            Self::Subcity => serializer.serialize_str("subcity"),
            Self::MediumGeoArea => serializer.serialize_str("medium_geo_area"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SearchTargetingOptionsAdGroupsRequestLocationTypesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "country" => Ok(Self::Country),
            "region" => Ok(Self::Region),
            "city" => Ok(Self::City),
            "zip" => Ok(Self::Zip),
            "neighborhood" => Ok(Self::Neighborhood),
            "subcity" => Ok(Self::Subcity),
            "medium_geo_area" => Ok(Self::MediumGeoArea),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SearchTargetingOptionsAdGroupsRequestLocationTypesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Country => write!(f, "country"),
            Self::Region => write!(f, "region"),
            Self::City => write!(f, "city"),
            Self::Zip => write!(f, "zip"),
            Self::Neighborhood => write!(f, "neighborhood"),
            Self::Subcity => write!(f, "subcity"),
            Self::MediumGeoArea => write!(f, "medium_geo_area"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
