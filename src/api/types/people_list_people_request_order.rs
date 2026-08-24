pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListPeopleRequestOrder {
    FirstSeenAt,
    LastSeenAt,
    FirstPurchaseAt,
    LastPurchaseAt,
    PurchaseCount,
    EventCount,
    Ltv,
    Aov,
    Name,
    Email,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListPeopleRequestOrder {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::FirstSeenAt => serializer.serialize_str("first_seen_at"),
            Self::LastSeenAt => serializer.serialize_str("last_seen_at"),
            Self::FirstPurchaseAt => serializer.serialize_str("first_purchase_at"),
            Self::LastPurchaseAt => serializer.serialize_str("last_purchase_at"),
            Self::PurchaseCount => serializer.serialize_str("purchase_count"),
            Self::EventCount => serializer.serialize_str("event_count"),
            Self::Ltv => serializer.serialize_str("ltv"),
            Self::Aov => serializer.serialize_str("aov"),
            Self::Name => serializer.serialize_str("name"),
            Self::Email => serializer.serialize_str("email"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListPeopleRequestOrder {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "first_seen_at" => Ok(Self::FirstSeenAt),
            "last_seen_at" => Ok(Self::LastSeenAt),
            "first_purchase_at" => Ok(Self::FirstPurchaseAt),
            "last_purchase_at" => Ok(Self::LastPurchaseAt),
            "purchase_count" => Ok(Self::PurchaseCount),
            "event_count" => Ok(Self::EventCount),
            "ltv" => Ok(Self::Ltv),
            "aov" => Ok(Self::Aov),
            "name" => Ok(Self::Name),
            "email" => Ok(Self::Email),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListPeopleRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FirstSeenAt => write!(f, "first_seen_at"),
            Self::LastSeenAt => write!(f, "last_seen_at"),
            Self::FirstPurchaseAt => write!(f, "first_purchase_at"),
            Self::LastPurchaseAt => write!(f, "last_purchase_at"),
            Self::PurchaseCount => write!(f, "purchase_count"),
            Self::EventCount => write!(f, "event_count"),
            Self::Ltv => write!(f, "ltv"),
            Self::Aov => write!(f, "aov"),
            Self::Name => write!(f, "name"),
            Self::Email => write!(f, "email"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
