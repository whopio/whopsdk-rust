pub use crate::prelude::*;

/// The headline's face: `amount` headlines the figure in `amount`; `trial` headlines the free-trial length off `trial_days`; `free` headlines the word — the served free signal or a membership transfer collecting nothing today (a renewing transfer states when its free stretch ends in `free_until`), never derived from zero prices (a promo covering the whole first charge is not a free plan).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionBreakdownDisplayHeadlineKind {
    Amount,
    Trial,
    Free,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionBreakdownDisplayHeadlineKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Amount => serializer.serialize_str("amount"),
            Self::Trial => serializer.serialize_str("trial"),
            Self::Free => serializer.serialize_str("free"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionBreakdownDisplayHeadlineKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "amount" => Ok(Self::Amount),
            "trial" => Ok(Self::Trial),
            "free" => Ok(Self::Free),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionBreakdownDisplayHeadlineKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Amount => write!(f, "amount"),
            Self::Trial => write!(f, "trial"),
            Self::Free => write!(f, "free"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
