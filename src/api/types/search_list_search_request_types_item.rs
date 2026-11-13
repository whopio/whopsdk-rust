pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListSearchRequestTypesItem {
    Account,
    Member,
    Membership,
    Payment,
    Affiliate,
    Review,
    ResolutionCenterCase,
    PromoCode,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListSearchRequestTypesItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Account => serializer.serialize_str("account"),
            Self::Member => serializer.serialize_str("member"),
            Self::Membership => serializer.serialize_str("membership"),
            Self::Payment => serializer.serialize_str("payment"),
            Self::Affiliate => serializer.serialize_str("affiliate"),
            Self::Review => serializer.serialize_str("review"),
            Self::ResolutionCenterCase => serializer.serialize_str("resolution_center_case"),
            Self::PromoCode => serializer.serialize_str("promo_code"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListSearchRequestTypesItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "account" => Ok(Self::Account),
            "member" => Ok(Self::Member),
            "membership" => Ok(Self::Membership),
            "payment" => Ok(Self::Payment),
            "affiliate" => Ok(Self::Affiliate),
            "review" => Ok(Self::Review),
            "resolution_center_case" => Ok(Self::ResolutionCenterCase),
            "promo_code" => Ok(Self::PromoCode),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListSearchRequestTypesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Account => write!(f, "account"),
            Self::Member => write!(f, "member"),
            Self::Membership => write!(f, "membership"),
            Self::Payment => write!(f, "payment"),
            Self::Affiliate => write!(f, "affiliate"),
            Self::Review => write!(f, "review"),
            Self::ResolutionCenterCase => write!(f, "resolution_center_case"),
            Self::PromoCode => write!(f, "promo_code"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
