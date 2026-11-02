pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListPaymentsRequestStatus {
    Open,
    Authorized,
    Paid,
    Pending,
    Uncollectible,
    Unresolved,
    Void,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListPaymentsRequestStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Open => serializer.serialize_str("open"),
            Self::Authorized => serializer.serialize_str("authorized"),
            Self::Paid => serializer.serialize_str("paid"),
            Self::Pending => serializer.serialize_str("pending"),
            Self::Uncollectible => serializer.serialize_str("uncollectible"),
            Self::Unresolved => serializer.serialize_str("unresolved"),
            Self::Void => serializer.serialize_str("void"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListPaymentsRequestStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "open" => Ok(Self::Open),
            "authorized" => Ok(Self::Authorized),
            "paid" => Ok(Self::Paid),
            "pending" => Ok(Self::Pending),
            "uncollectible" => Ok(Self::Uncollectible),
            "unresolved" => Ok(Self::Unresolved),
            "void" => Ok(Self::Void),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListPaymentsRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Open => write!(f, "open"),
            Self::Authorized => write!(f, "authorized"),
            Self::Paid => write!(f, "paid"),
            Self::Pending => write!(f, "pending"),
            Self::Uncollectible => write!(f, "uncollectible"),
            Self::Unresolved => write!(f, "unresolved"),
            Self::Void => write!(f, "void"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
