pub use crate::prelude::*;

/// Where members come from. `csv_upload` = an uploaded customer list; `people_filter` = built from saved People filters. See `auto_refresh` for whether a `people_filter` audience keeps updating.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudienceSourceType {
    CsvUpload,
    PeopleFilter,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AudienceSourceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CsvUpload => serializer.serialize_str("csv_upload"),
            Self::PeopleFilter => serializer.serialize_str("people_filter"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AudienceSourceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "csv_upload" => Ok(Self::CsvUpload),
            "people_filter" => Ok(Self::PeopleFilter),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AudienceSourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CsvUpload => write!(f, "csv_upload"),
            Self::PeopleFilter => write!(f, "people_filter"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
