pub use crate::prelude::*;

/// The income stream these terms describe.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeeOptionsPartnersResponseStreamsItemIncomeStream {
    Sales,
    Transfer,
    CardInterchange,
    AdSpend,
    Withdrawal,
    OnboardingReward,
    PartnerReward,
    VerifiedPartnerReferralPayback,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FeeOptionsPartnersResponseStreamsItemIncomeStream {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Sales => serializer.serialize_str("sales"),
            Self::Transfer => serializer.serialize_str("transfer"),
            Self::CardInterchange => serializer.serialize_str("card_interchange"),
            Self::AdSpend => serializer.serialize_str("ad_spend"),
            Self::Withdrawal => serializer.serialize_str("withdrawal"),
            Self::OnboardingReward => serializer.serialize_str("onboarding_reward"),
            Self::PartnerReward => serializer.serialize_str("partner_reward"),
            Self::VerifiedPartnerReferralPayback => {
                serializer.serialize_str("verified_partner_referral_payback")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FeeOptionsPartnersResponseStreamsItemIncomeStream {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "sales" => Ok(Self::Sales),
            "transfer" => Ok(Self::Transfer),
            "card_interchange" => Ok(Self::CardInterchange),
            "ad_spend" => Ok(Self::AdSpend),
            "withdrawal" => Ok(Self::Withdrawal),
            "onboarding_reward" => Ok(Self::OnboardingReward),
            "partner_reward" => Ok(Self::PartnerReward),
            "verified_partner_referral_payback" => Ok(Self::VerifiedPartnerReferralPayback),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FeeOptionsPartnersResponseStreamsItemIncomeStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sales => write!(f, "sales"),
            Self::Transfer => write!(f, "transfer"),
            Self::CardInterchange => write!(f, "card_interchange"),
            Self::AdSpend => write!(f, "ad_spend"),
            Self::Withdrawal => write!(f, "withdrawal"),
            Self::OnboardingReward => write!(f, "onboarding_reward"),
            Self::PartnerReward => write!(f, "partner_reward"),
            Self::VerifiedPartnerReferralPayback => write!(f, "verified_partner_referral_payback"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
