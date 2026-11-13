pub use crate::prelude::*;

/// Expected text format. max_product_price uses money instead of values.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinancingRequirementTextFormat {
    Plain,
    Url,
    Email,
    Phone,
    Number,
    Checkbox,
    Dropdown,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FinancingRequirementTextFormat {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Plain => serializer.serialize_str("plain"),
            Self::Url => serializer.serialize_str("url"),
            Self::Email => serializer.serialize_str("email"),
            Self::Phone => serializer.serialize_str("phone"),
            Self::Number => serializer.serialize_str("number"),
            Self::Checkbox => serializer.serialize_str("checkbox"),
            Self::Dropdown => serializer.serialize_str("dropdown"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FinancingRequirementTextFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "plain" => Ok(Self::Plain),
            "url" => Ok(Self::Url),
            "email" => Ok(Self::Email),
            "phone" => Ok(Self::Phone),
            "number" => Ok(Self::Number),
            "checkbox" => Ok(Self::Checkbox),
            "dropdown" => Ok(Self::Dropdown),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FinancingRequirementTextFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plain => write!(f, "plain"),
            Self::Url => write!(f, "url"),
            Self::Email => write!(f, "email"),
            Self::Phone => write!(f, "phone"),
            Self::Number => write!(f, "number"),
            Self::Checkbox => write!(f, "checkbox"),
            Self::Dropdown => write!(f, "dropdown"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
