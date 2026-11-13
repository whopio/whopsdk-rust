pub use crate::prelude::*;

/// Why the provider has this status, when known. On `active` it is `dispute_rate_near_limit` when the provider is close to being turned off. On `inactive` it names what keeps it off, including `not_included` when the account's approval doesn't cover it, `turned_off` when it was switched off after setup, `setup_failed`, and `declined` when the provider turned the account down without a reason Whop recognizes. More reasons may be added.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FinancingProviderReason {
    AccountNotSupported,
    PayinAccountRequired,
    PaymentAccountRequired,
    ProcessorConnectionRequired,
    MinimumVolumeNotMet,
    TurnedOffInCheckoutSettings,
    ProviderReview,
    DisputeRateNearLimit,
    DisputeRateExceeded,
    WhopRiskControl,
    NotIncluded,
    TurnedOff,
    Declined,
    SetupFailed,
    PayoutAccountCountryUnsupported,
    SellerCountryUnsupported,
    ApprovedDisputeRateTooHigh,
    RestrictedIndustry,
    IncomeClaims,
    MissingStoreDetails,
    DuplicateAccount,
    InvalidWebsite,
    Unspecified,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for FinancingProviderReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AccountNotSupported => serializer.serialize_str("account_not_supported"),
            Self::PayinAccountRequired => serializer.serialize_str("payin_account_required"),
            Self::PaymentAccountRequired => serializer.serialize_str("payment_account_required"),
            Self::ProcessorConnectionRequired => {
                serializer.serialize_str("processor_connection_required")
            }
            Self::MinimumVolumeNotMet => serializer.serialize_str("minimum_volume_not_met"),
            Self::TurnedOffInCheckoutSettings => {
                serializer.serialize_str("turned_off_in_checkout_settings")
            }
            Self::ProviderReview => serializer.serialize_str("provider_review"),
            Self::DisputeRateNearLimit => serializer.serialize_str("dispute_rate_near_limit"),
            Self::DisputeRateExceeded => serializer.serialize_str("dispute_rate_exceeded"),
            Self::WhopRiskControl => serializer.serialize_str("whop_risk_control"),
            Self::NotIncluded => serializer.serialize_str("not_included"),
            Self::TurnedOff => serializer.serialize_str("turned_off"),
            Self::Declined => serializer.serialize_str("declined"),
            Self::SetupFailed => serializer.serialize_str("setup_failed"),
            Self::PayoutAccountCountryUnsupported => {
                serializer.serialize_str("payout_account_country_unsupported")
            }
            Self::SellerCountryUnsupported => {
                serializer.serialize_str("seller_country_unsupported")
            }
            Self::ApprovedDisputeRateTooHigh => {
                serializer.serialize_str("approved_dispute_rate_too_high")
            }
            Self::RestrictedIndustry => serializer.serialize_str("restricted_industry"),
            Self::IncomeClaims => serializer.serialize_str("income_claims"),
            Self::MissingStoreDetails => serializer.serialize_str("missing_store_details"),
            Self::DuplicateAccount => serializer.serialize_str("duplicate_account"),
            Self::InvalidWebsite => serializer.serialize_str("invalid_website"),
            Self::Unspecified => serializer.serialize_str("unspecified"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for FinancingProviderReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "account_not_supported" => Ok(Self::AccountNotSupported),
            "payin_account_required" => Ok(Self::PayinAccountRequired),
            "payment_account_required" => Ok(Self::PaymentAccountRequired),
            "processor_connection_required" => Ok(Self::ProcessorConnectionRequired),
            "minimum_volume_not_met" => Ok(Self::MinimumVolumeNotMet),
            "turned_off_in_checkout_settings" => Ok(Self::TurnedOffInCheckoutSettings),
            "provider_review" => Ok(Self::ProviderReview),
            "dispute_rate_near_limit" => Ok(Self::DisputeRateNearLimit),
            "dispute_rate_exceeded" => Ok(Self::DisputeRateExceeded),
            "whop_risk_control" => Ok(Self::WhopRiskControl),
            "not_included" => Ok(Self::NotIncluded),
            "turned_off" => Ok(Self::TurnedOff),
            "declined" => Ok(Self::Declined),
            "setup_failed" => Ok(Self::SetupFailed),
            "payout_account_country_unsupported" => Ok(Self::PayoutAccountCountryUnsupported),
            "seller_country_unsupported" => Ok(Self::SellerCountryUnsupported),
            "approved_dispute_rate_too_high" => Ok(Self::ApprovedDisputeRateTooHigh),
            "restricted_industry" => Ok(Self::RestrictedIndustry),
            "income_claims" => Ok(Self::IncomeClaims),
            "missing_store_details" => Ok(Self::MissingStoreDetails),
            "duplicate_account" => Ok(Self::DuplicateAccount),
            "invalid_website" => Ok(Self::InvalidWebsite),
            "unspecified" => Ok(Self::Unspecified),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for FinancingProviderReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AccountNotSupported => write!(f, "account_not_supported"),
            Self::PayinAccountRequired => write!(f, "payin_account_required"),
            Self::PaymentAccountRequired => write!(f, "payment_account_required"),
            Self::ProcessorConnectionRequired => write!(f, "processor_connection_required"),
            Self::MinimumVolumeNotMet => write!(f, "minimum_volume_not_met"),
            Self::TurnedOffInCheckoutSettings => write!(f, "turned_off_in_checkout_settings"),
            Self::ProviderReview => write!(f, "provider_review"),
            Self::DisputeRateNearLimit => write!(f, "dispute_rate_near_limit"),
            Self::DisputeRateExceeded => write!(f, "dispute_rate_exceeded"),
            Self::WhopRiskControl => write!(f, "whop_risk_control"),
            Self::NotIncluded => write!(f, "not_included"),
            Self::TurnedOff => write!(f, "turned_off"),
            Self::Declined => write!(f, "declined"),
            Self::SetupFailed => write!(f, "setup_failed"),
            Self::PayoutAccountCountryUnsupported => {
                write!(f, "payout_account_country_unsupported")
            }
            Self::SellerCountryUnsupported => write!(f, "seller_country_unsupported"),
            Self::ApprovedDisputeRateTooHigh => write!(f, "approved_dispute_rate_too_high"),
            Self::RestrictedIndustry => write!(f, "restricted_industry"),
            Self::IncomeClaims => write!(f, "income_claims"),
            Self::MissingStoreDetails => write!(f, "missing_store_details"),
            Self::DuplicateAccount => write!(f, "duplicate_account"),
            Self::InvalidWebsite => write!(f, "invalid_website"),
            Self::Unspecified => write!(f, "unspecified"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
