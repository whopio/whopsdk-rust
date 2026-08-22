pub use crate::prelude::*;

/// The different statuses an invoice can be in
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InvoiceStatuses {
    Draft,
    Open,
    Paid,
    PastDue,
    Uncollectible,
    Void,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for InvoiceStatuses {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Draft => serializer.serialize_str("draft"),
            Self::Open => serializer.serialize_str("open"),
            Self::Paid => serializer.serialize_str("paid"),
            Self::PastDue => serializer.serialize_str("past_due"),
            Self::Uncollectible => serializer.serialize_str("uncollectible"),
            Self::Void => serializer.serialize_str("void"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for InvoiceStatuses {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "draft" => Ok(Self::Draft),
            "open" => Ok(Self::Open),
            "paid" => Ok(Self::Paid),
            "past_due" => Ok(Self::PastDue),
            "uncollectible" => Ok(Self::Uncollectible),
            "void" => Ok(Self::Void),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for InvoiceStatuses {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Draft => write!(f, "draft"),
            Self::Open => write!(f, "open"),
            Self::Paid => write!(f, "paid"),
            Self::PastDue => write!(f, "past_due"),
            Self::Uncollectible => write!(f, "uncollectible"),
            Self::Void => write!(f, "void"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
