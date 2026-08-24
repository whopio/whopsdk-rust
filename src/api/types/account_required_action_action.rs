pub use crate::prelude::*;

/// What the holder must do; new values may be added, so handle unknown actions gracefully
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountRequiredActionAction {
    DepositFunds,
    SubmitInformationRequest,
    ReauthorizePayoutMethods,
    UpdatePayoutProfile,
    CardUsageReview,
    VerifyIdentity,
    SignFormationDocuments,
    ConnectFulfillmentTracker,
    SetupApplePayDomains,
    ConfigureTaxRemitter,
    AddVatRegistration,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AccountRequiredActionAction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::DepositFunds => serializer.serialize_str("deposit_funds"),
            Self::SubmitInformationRequest => {
                serializer.serialize_str("submit_information_request")
            }
            Self::ReauthorizePayoutMethods => {
                serializer.serialize_str("reauthorize_payout_methods")
            }
            Self::UpdatePayoutProfile => serializer.serialize_str("update_payout_profile"),
            Self::CardUsageReview => serializer.serialize_str("card_usage_review"),
            Self::VerifyIdentity => serializer.serialize_str("verify_identity"),
            Self::SignFormationDocuments => serializer.serialize_str("sign_formation_documents"),
            Self::ConnectFulfillmentTracker => {
                serializer.serialize_str("connect_fulfillment_tracker")
            }
            Self::SetupApplePayDomains => serializer.serialize_str("setup_apple_pay_domains"),
            Self::ConfigureTaxRemitter => serializer.serialize_str("configure_tax_remitter"),
            Self::AddVatRegistration => serializer.serialize_str("add_vat_registration"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AccountRequiredActionAction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "deposit_funds" => Ok(Self::DepositFunds),
            "submit_information_request" => Ok(Self::SubmitInformationRequest),
            "reauthorize_payout_methods" => Ok(Self::ReauthorizePayoutMethods),
            "update_payout_profile" => Ok(Self::UpdatePayoutProfile),
            "card_usage_review" => Ok(Self::CardUsageReview),
            "verify_identity" => Ok(Self::VerifyIdentity),
            "sign_formation_documents" => Ok(Self::SignFormationDocuments),
            "connect_fulfillment_tracker" => Ok(Self::ConnectFulfillmentTracker),
            "setup_apple_pay_domains" => Ok(Self::SetupApplePayDomains),
            "configure_tax_remitter" => Ok(Self::ConfigureTaxRemitter),
            "add_vat_registration" => Ok(Self::AddVatRegistration),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AccountRequiredActionAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DepositFunds => write!(f, "deposit_funds"),
            Self::SubmitInformationRequest => write!(f, "submit_information_request"),
            Self::ReauthorizePayoutMethods => write!(f, "reauthorize_payout_methods"),
            Self::UpdatePayoutProfile => write!(f, "update_payout_profile"),
            Self::CardUsageReview => write!(f, "card_usage_review"),
            Self::VerifyIdentity => write!(f, "verify_identity"),
            Self::SignFormationDocuments => write!(f, "sign_formation_documents"),
            Self::ConnectFulfillmentTracker => write!(f, "connect_fulfillment_tracker"),
            Self::SetupApplePayDomains => write!(f, "setup_apple_pay_domains"),
            Self::ConfigureTaxRemitter => write!(f, "configure_tax_remitter"),
            Self::AddVatRegistration => write!(f, "add_vat_registration"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
