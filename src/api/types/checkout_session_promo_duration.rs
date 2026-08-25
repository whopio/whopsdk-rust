pub use crate::prelude::*;

/// Which charges the discount covers: `forever` discounts every charge; `once` covers only the charge at purchase — the code is spent then even when it made that charge free, except on a free trial with nothing due today, where it holds until the trial's first real charge; `repeating` covers every charge landing within `number_of_intervals` calendar months of purchase.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionPromoDuration {
    Forever,
    Once,
    Repeating,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionPromoDuration {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Forever => serializer.serialize_str("forever"),
            Self::Once => serializer.serialize_str("once"),
            Self::Repeating => serializer.serialize_str("repeating"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionPromoDuration {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "forever" => Ok(Self::Forever),
            "once" => Ok(Self::Once),
            "repeating" => Ok(Self::Repeating),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionPromoDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Forever => write!(f, "forever"),
            Self::Once => write!(f, "once"),
            Self::Repeating => write!(f, "repeating"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
