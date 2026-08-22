pub use crate::prelude::*;

/// The recommendation; new values may be added, so handle unknown actions gracefully
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UserRecommendedActionAction {
    CreateBusiness,
    BecomeAffiliate,
    BecomeWhopPartner,
    ThemeBusiness,
    CreateProduct,
    CreatePlan,
    VerifyIdentity,
    ConnectAffiliateProgram,
    CreatePromotion,
    MigrateFromStripe,
    AcceptFirstPayment,
    LaunchFirstAd,
    LaunchDraftCampaign,
    IncreaseAdBudget,
    RefreshAdCreatives,
    FixAdBilling,
    ExcludeCustomersFromAds,
    RetargetAbandonedCheckouts,
    FixFunnelDropoff,
    InviteTeamMember,
    EnableTaxCollection,
    CreateCard,
    ApplyForFinancing,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UserRecommendedActionAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::CreateBusiness => serializer.serialize_str("create_business"),
            Self::BecomeAffiliate => serializer.serialize_str("become_affiliate"),
            Self::BecomeWhopPartner => serializer.serialize_str("become_whop_partner"),
            Self::ThemeBusiness => serializer.serialize_str("theme_business"),
            Self::CreateProduct => serializer.serialize_str("create_product"),
            Self::CreatePlan => serializer.serialize_str("create_plan"),
            Self::VerifyIdentity => serializer.serialize_str("verify_identity"),
            Self::ConnectAffiliateProgram => serializer.serialize_str("connect_affiliate_program"),
            Self::CreatePromotion => serializer.serialize_str("create_promotion"),
            Self::MigrateFromStripe => serializer.serialize_str("migrate_from_stripe"),
            Self::AcceptFirstPayment => serializer.serialize_str("accept_first_payment"),
            Self::LaunchFirstAd => serializer.serialize_str("launch_first_ad"),
            Self::LaunchDraftCampaign => serializer.serialize_str("launch_draft_campaign"),
            Self::IncreaseAdBudget => serializer.serialize_str("increase_ad_budget"),
            Self::RefreshAdCreatives => serializer.serialize_str("refresh_ad_creatives"),
            Self::FixAdBilling => serializer.serialize_str("fix_ad_billing"),
            Self::ExcludeCustomersFromAds => serializer.serialize_str("exclude_customers_from_ads"),
            Self::RetargetAbandonedCheckouts => {
                serializer.serialize_str("retarget_abandoned_checkouts")
            }
            Self::FixFunnelDropoff => serializer.serialize_str("fix_funnel_dropoff"),
            Self::InviteTeamMember => serializer.serialize_str("invite_team_member"),
            Self::EnableTaxCollection => serializer.serialize_str("enable_tax_collection"),
            Self::CreateCard => serializer.serialize_str("create_card"),
            Self::ApplyForFinancing => serializer.serialize_str("apply_for_financing"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UserRecommendedActionAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "create_business" => Ok(Self::CreateBusiness),
            "become_affiliate" => Ok(Self::BecomeAffiliate),
            "become_whop_partner" => Ok(Self::BecomeWhopPartner),
            "theme_business" => Ok(Self::ThemeBusiness),
            "create_product" => Ok(Self::CreateProduct),
            "create_plan" => Ok(Self::CreatePlan),
            "verify_identity" => Ok(Self::VerifyIdentity),
            "connect_affiliate_program" => Ok(Self::ConnectAffiliateProgram),
            "create_promotion" => Ok(Self::CreatePromotion),
            "migrate_from_stripe" => Ok(Self::MigrateFromStripe),
            "accept_first_payment" => Ok(Self::AcceptFirstPayment),
            "launch_first_ad" => Ok(Self::LaunchFirstAd),
            "launch_draft_campaign" => Ok(Self::LaunchDraftCampaign),
            "increase_ad_budget" => Ok(Self::IncreaseAdBudget),
            "refresh_ad_creatives" => Ok(Self::RefreshAdCreatives),
            "fix_ad_billing" => Ok(Self::FixAdBilling),
            "exclude_customers_from_ads" => Ok(Self::ExcludeCustomersFromAds),
            "retarget_abandoned_checkouts" => Ok(Self::RetargetAbandonedCheckouts),
            "fix_funnel_dropoff" => Ok(Self::FixFunnelDropoff),
            "invite_team_member" => Ok(Self::InviteTeamMember),
            "enable_tax_collection" => Ok(Self::EnableTaxCollection),
            "create_card" => Ok(Self::CreateCard),
            "apply_for_financing" => Ok(Self::ApplyForFinancing),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UserRecommendedActionAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CreateBusiness => write!(f, "create_business"),
            Self::BecomeAffiliate => write!(f, "become_affiliate"),
            Self::BecomeWhopPartner => write!(f, "become_whop_partner"),
            Self::ThemeBusiness => write!(f, "theme_business"),
            Self::CreateProduct => write!(f, "create_product"),
            Self::CreatePlan => write!(f, "create_plan"),
            Self::VerifyIdentity => write!(f, "verify_identity"),
            Self::ConnectAffiliateProgram => write!(f, "connect_affiliate_program"),
            Self::CreatePromotion => write!(f, "create_promotion"),
            Self::MigrateFromStripe => write!(f, "migrate_from_stripe"),
            Self::AcceptFirstPayment => write!(f, "accept_first_payment"),
            Self::LaunchFirstAd => write!(f, "launch_first_ad"),
            Self::LaunchDraftCampaign => write!(f, "launch_draft_campaign"),
            Self::IncreaseAdBudget => write!(f, "increase_ad_budget"),
            Self::RefreshAdCreatives => write!(f, "refresh_ad_creatives"),
            Self::FixAdBilling => write!(f, "fix_ad_billing"),
            Self::ExcludeCustomersFromAds => write!(f, "exclude_customers_from_ads"),
            Self::RetargetAbandonedCheckouts => write!(f, "retarget_abandoned_checkouts"),
            Self::FixFunnelDropoff => write!(f, "fix_funnel_dropoff"),
            Self::InviteTeamMember => write!(f, "invite_team_member"),
            Self::EnableTaxCollection => write!(f, "enable_tax_collection"),
            Self::CreateCard => write!(f, "create_card"),
            Self::ApplyForFinancing => write!(f, "apply_for_financing"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
