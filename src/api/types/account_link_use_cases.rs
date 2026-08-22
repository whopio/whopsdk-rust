pub use crate::prelude::*;

/// The different use cases for generating an account link.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountLinkUseCases {
    AccountOnboarding,
    PayoutsPortal,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountLinkUseCases {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AccountOnboarding => serializer.serialize_str("account_onboarding"),
            Self::PayoutsPortal => serializer.serialize_str("payouts_portal"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountLinkUseCases {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "account_onboarding" => Ok(Self::AccountOnboarding),
            "payouts_portal" => Ok(Self::PayoutsPortal),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountLinkUseCases {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountOnboarding => write!(f, "account_onboarding"),
            Self::PayoutsPortal => write!(f, "payouts_portal"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
