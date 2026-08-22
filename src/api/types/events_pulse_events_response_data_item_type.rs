pub use crate::prelude::*;

/// What moved: a purchase, an affiliate commission, Whop card spend, ad spend, app revenue, an off-platform sale, a wallet deposit, a card load, a claimed drop, a transfer between accounts, or a referral bonus.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PulseEventsResponseDataItemType {
    Purchase,
    AffiliateCommission,
    CardSpend,
    AdSpend,
    AppRevenue,
    OffPlatformSale,
    Deposit,
    CardLoad,
    AirdropClaim,
    Transfer,
    ReferralBonus,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PulseEventsResponseDataItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Purchase => serializer.serialize_str("purchase"),
            Self::AffiliateCommission => serializer.serialize_str("affiliate_commission"),
            Self::CardSpend => serializer.serialize_str("card_spend"),
            Self::AdSpend => serializer.serialize_str("ad_spend"),
            Self::AppRevenue => serializer.serialize_str("app_revenue"),
            Self::OffPlatformSale => serializer.serialize_str("off_platform_sale"),
            Self::Deposit => serializer.serialize_str("deposit"),
            Self::CardLoad => serializer.serialize_str("card_load"),
            Self::AirdropClaim => serializer.serialize_str("airdrop_claim"),
            Self::Transfer => serializer.serialize_str("transfer"),
            Self::ReferralBonus => serializer.serialize_str("referral_bonus"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PulseEventsResponseDataItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "purchase" => Ok(Self::Purchase),
            "affiliate_commission" => Ok(Self::AffiliateCommission),
            "card_spend" => Ok(Self::CardSpend),
            "ad_spend" => Ok(Self::AdSpend),
            "app_revenue" => Ok(Self::AppRevenue),
            "off_platform_sale" => Ok(Self::OffPlatformSale),
            "deposit" => Ok(Self::Deposit),
            "card_load" => Ok(Self::CardLoad),
            "airdrop_claim" => Ok(Self::AirdropClaim),
            "transfer" => Ok(Self::Transfer),
            "referral_bonus" => Ok(Self::ReferralBonus),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PulseEventsResponseDataItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Purchase => write!(f, "purchase"),
            Self::AffiliateCommission => write!(f, "affiliate_commission"),
            Self::CardSpend => write!(f, "card_spend"),
            Self::AdSpend => write!(f, "ad_spend"),
            Self::AppRevenue => write!(f, "app_revenue"),
            Self::OffPlatformSale => write!(f, "off_platform_sale"),
            Self::Deposit => write!(f, "deposit"),
            Self::CardLoad => write!(f, "card_load"),
            Self::AirdropClaim => write!(f, "airdrop_claim"),
            Self::Transfer => write!(f, "transfer"),
            Self::ReferralBonus => write!(f, "referral_bonus"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
