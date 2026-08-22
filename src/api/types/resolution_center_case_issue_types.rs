pub use crate::prelude::*;

/// The different types of issues a resolution can be
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResolutionCenterCaseIssueTypes {
    ForgotToCancel,
    ItemNotReceived,
    SignificantlyNotAsDescribed,
    UnauthorizedTransaction,
    ProductUnacceptable,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResolutionCenterCaseIssueTypes {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ForgotToCancel => serializer.serialize_str("forgot_to_cancel"),
            Self::ItemNotReceived => serializer.serialize_str("item_not_received"),
            Self::SignificantlyNotAsDescribed => {
                serializer.serialize_str("significantly_not_as_described")
            }
            Self::UnauthorizedTransaction => serializer.serialize_str("unauthorized_transaction"),
            Self::ProductUnacceptable => serializer.serialize_str("product_unacceptable"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResolutionCenterCaseIssueTypes {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "forgot_to_cancel" => Ok(Self::ForgotToCancel),
            "item_not_received" => Ok(Self::ItemNotReceived),
            "significantly_not_as_described" => Ok(Self::SignificantlyNotAsDescribed),
            "unauthorized_transaction" => Ok(Self::UnauthorizedTransaction),
            "product_unacceptable" => Ok(Self::ProductUnacceptable),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResolutionCenterCaseIssueTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ForgotToCancel => write!(f, "forgot_to_cancel"),
            Self::ItemNotReceived => write!(f, "item_not_received"),
            Self::SignificantlyNotAsDescribed => write!(f, "significantly_not_as_described"),
            Self::UnauthorizedTransaction => write!(f, "unauthorized_transaction"),
            Self::ProductUnacceptable => write!(f, "product_unacceptable"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
