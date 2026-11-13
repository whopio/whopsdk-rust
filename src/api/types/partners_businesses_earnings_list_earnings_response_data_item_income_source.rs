pub use crate::prelude::*;

/// Which income source the commission is on: product-sales gross profit, Whop Ads spend billed to the business, platform balance transfer fees, Whop Card interchange, withdrawal gross profit, or a fixed onboarding or referral link reward paid to the partner when a referred business qualifies.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ListEarningsResponseDataItemIncomeSource {
    Sales,
    AdSpend,
    Transfer,
    CardInterchange,
    Withdrawal,
    BusinessReferral,
    OnboardingReward,
    PartnerReward,
    VerifiedPartnerReferralPayback,
    CommissionAdvance,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ListEarningsResponseDataItemIncomeSource {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sales => serializer.serialize_str("sales"),
            Self::AdSpend => serializer.serialize_str("ad_spend"),
            Self::Transfer => serializer.serialize_str("transfer"),
            Self::CardInterchange => serializer.serialize_str("card_interchange"),
            Self::Withdrawal => serializer.serialize_str("withdrawal"),
            Self::BusinessReferral => serializer.serialize_str("business_referral"),
            Self::OnboardingReward => serializer.serialize_str("onboarding_reward"),
            Self::PartnerReward => serializer.serialize_str("partner_reward"),
            Self::VerifiedPartnerReferralPayback => {
                serializer.serialize_str("verified_partner_referral_payback")
            }
            Self::CommissionAdvance => serializer.serialize_str("commission_advance"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ListEarningsResponseDataItemIncomeSource {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sales" => Ok(Self::Sales),
            "ad_spend" => Ok(Self::AdSpend),
            "transfer" => Ok(Self::Transfer),
            "card_interchange" => Ok(Self::CardInterchange),
            "withdrawal" => Ok(Self::Withdrawal),
            "business_referral" => Ok(Self::BusinessReferral),
            "onboarding_reward" => Ok(Self::OnboardingReward),
            "partner_reward" => Ok(Self::PartnerReward),
            "verified_partner_referral_payback" => Ok(Self::VerifiedPartnerReferralPayback),
            "commission_advance" => Ok(Self::CommissionAdvance),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ListEarningsResponseDataItemIncomeSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sales => write!(f, "sales"),
            Self::AdSpend => write!(f, "ad_spend"),
            Self::Transfer => write!(f, "transfer"),
            Self::CardInterchange => write!(f, "card_interchange"),
            Self::Withdrawal => write!(f, "withdrawal"),
            Self::BusinessReferral => write!(f, "business_referral"),
            Self::OnboardingReward => write!(f, "onboarding_reward"),
            Self::PartnerReward => write!(f, "partner_reward"),
            Self::VerifiedPartnerReferralPayback => write!(f, "verified_partner_referral_payback"),
            Self::CommissionAdvance => write!(f, "commission_advance"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
