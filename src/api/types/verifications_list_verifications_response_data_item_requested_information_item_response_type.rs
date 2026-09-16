pub use crate::prelude::*;

/// Optional native input format for a text response.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListVerificationsResponseDataItemRequestedInformationItemResponseType {
    YesNo,
    YesNoNa,
    Date,
    Url,
    Number,
    Tel,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListVerificationsResponseDataItemRequestedInformationItemResponseType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::YesNo => serializer.serialize_str("yes_no"),
            Self::YesNoNa => serializer.serialize_str("yes_no_na"),
            Self::Date => serializer.serialize_str("date"),
            Self::Url => serializer.serialize_str("url"),
            Self::Number => serializer.serialize_str("number"),
            Self::Tel => serializer.serialize_str("tel"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de>
    for ListVerificationsResponseDataItemRequestedInformationItemResponseType
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "yes_no" => Ok(Self::YesNo),
            "yes_no_na" => Ok(Self::YesNoNa),
            "date" => Ok(Self::Date),
            "url" => Ok(Self::Url),
            "number" => Ok(Self::Number),
            "tel" => Ok(Self::Tel),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListVerificationsResponseDataItemRequestedInformationItemResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::YesNo => write!(f, "yes_no"),
            Self::YesNoNa => write!(f, "yes_no_na"),
            Self::Date => write!(f, "date"),
            Self::Url => write!(f, "url"),
            Self::Number => write!(f, "number"),
            Self::Tel => write!(f, "tel"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
