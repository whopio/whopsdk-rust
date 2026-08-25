pub use crate::prelude::*;

/// Which row this is. Render a kind you know from `detail` in your own wording; render one you do not from `label` and `text` verbatim — never drop it.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionBreakdownDisplayRowKind {
    ThenStarting,
    ThenAfterTrial,
    DueToday,
    Period,
    OneTimePayment,
    AccessWindow,
    AccessUntil,
    TrialWindow,
    WaitlistNotice,
    WaitlistTrialNotice,
    FreeForever,
    TaxIncluded,
    PromoFreePeriod,
    InitialFee,
    RenewalFee,
    TaxToday,
    TotalDueToday,
    TotalIfAccepted,
    TotalDueStarting,
    TotalAfterTrial,
    PostPromo,
    ItemSubtotal,
    ServiceFee,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionBreakdownDisplayRowKind {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::ThenStarting => serializer.serialize_str("then_starting"),
            Self::ThenAfterTrial => serializer.serialize_str("then_after_trial"),
            Self::DueToday => serializer.serialize_str("due_today"),
            Self::Period => serializer.serialize_str("period"),
            Self::OneTimePayment => serializer.serialize_str("one_time_payment"),
            Self::AccessWindow => serializer.serialize_str("access_window"),
            Self::AccessUntil => serializer.serialize_str("access_until"),
            Self::TrialWindow => serializer.serialize_str("trial_window"),
            Self::WaitlistNotice => serializer.serialize_str("waitlist_notice"),
            Self::WaitlistTrialNotice => serializer.serialize_str("waitlist_trial_notice"),
            Self::FreeForever => serializer.serialize_str("free_forever"),
            Self::TaxIncluded => serializer.serialize_str("tax_included"),
            Self::PromoFreePeriod => serializer.serialize_str("promo_free_period"),
            Self::InitialFee => serializer.serialize_str("initial_fee"),
            Self::RenewalFee => serializer.serialize_str("renewal_fee"),
            Self::TaxToday => serializer.serialize_str("tax_today"),
            Self::TotalDueToday => serializer.serialize_str("total_due_today"),
            Self::TotalIfAccepted => serializer.serialize_str("total_if_accepted"),
            Self::TotalDueStarting => serializer.serialize_str("total_due_starting"),
            Self::TotalAfterTrial => serializer.serialize_str("total_after_trial"),
            Self::PostPromo => serializer.serialize_str("post_promo"),
            Self::ItemSubtotal => serializer.serialize_str("item_subtotal"),
            Self::ServiceFee => serializer.serialize_str("service_fee"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionBreakdownDisplayRowKind {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "then_starting" => Ok(Self::ThenStarting),
            "then_after_trial" => Ok(Self::ThenAfterTrial),
            "due_today" => Ok(Self::DueToday),
            "period" => Ok(Self::Period),
            "one_time_payment" => Ok(Self::OneTimePayment),
            "access_window" => Ok(Self::AccessWindow),
            "access_until" => Ok(Self::AccessUntil),
            "trial_window" => Ok(Self::TrialWindow),
            "waitlist_notice" => Ok(Self::WaitlistNotice),
            "waitlist_trial_notice" => Ok(Self::WaitlistTrialNotice),
            "free_forever" => Ok(Self::FreeForever),
            "tax_included" => Ok(Self::TaxIncluded),
            "promo_free_period" => Ok(Self::PromoFreePeriod),
            "initial_fee" => Ok(Self::InitialFee),
            "renewal_fee" => Ok(Self::RenewalFee),
            "tax_today" => Ok(Self::TaxToday),
            "total_due_today" => Ok(Self::TotalDueToday),
            "total_if_accepted" => Ok(Self::TotalIfAccepted),
            "total_due_starting" => Ok(Self::TotalDueStarting),
            "total_after_trial" => Ok(Self::TotalAfterTrial),
            "post_promo" => Ok(Self::PostPromo),
            "item_subtotal" => Ok(Self::ItemSubtotal),
            "service_fee" => Ok(Self::ServiceFee),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionBreakdownDisplayRowKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ThenStarting => write!(f, "then_starting"),
            Self::ThenAfterTrial => write!(f, "then_after_trial"),
            Self::DueToday => write!(f, "due_today"),
            Self::Period => write!(f, "period"),
            Self::OneTimePayment => write!(f, "one_time_payment"),
            Self::AccessWindow => write!(f, "access_window"),
            Self::AccessUntil => write!(f, "access_until"),
            Self::TrialWindow => write!(f, "trial_window"),
            Self::WaitlistNotice => write!(f, "waitlist_notice"),
            Self::WaitlistTrialNotice => write!(f, "waitlist_trial_notice"),
            Self::FreeForever => write!(f, "free_forever"),
            Self::TaxIncluded => write!(f, "tax_included"),
            Self::PromoFreePeriod => write!(f, "promo_free_period"),
            Self::InitialFee => write!(f, "initial_fee"),
            Self::RenewalFee => write!(f, "renewal_fee"),
            Self::TaxToday => write!(f, "tax_today"),
            Self::TotalDueToday => write!(f, "total_due_today"),
            Self::TotalIfAccepted => write!(f, "total_if_accepted"),
            Self::TotalDueStarting => write!(f, "total_due_starting"),
            Self::TotalAfterTrial => write!(f, "total_after_trial"),
            Self::PostPromo => write!(f, "post_promo"),
            Self::ItemSubtotal => write!(f, "item_subtotal"),
            Self::ServiceFee => write!(f, "service_fee"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
