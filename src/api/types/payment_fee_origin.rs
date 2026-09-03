pub use crate::prelude::*;

/// The specific fee this line is, such as `payment_processing_percentage_fee` or `revshare_percentage_fee`.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PaymentFeeOrigin {
    StripeDomesticProcessingFee,
    StripeInternationalProcessingFee,
    StripeFixedProcessingFee,
    StripeBillingFee,
    StripeRadarFee,
    SalesTaxRemittance,
    SalesTaxRemittanceReversal,
    StripeSalesTaxFee,
    WhopProcessingFee,
    MarketplaceAffiliateFee,
    AffiliateFee,
    CryptoFee,
    StripeStandardProcessingFee,
    PaypalFee,
    StripePayoutFee,
    DisputeFee,
    DisputeAlertFee,
    DisputeRepresentmentFee,
    AppleProcessingFee,
    BuyerFee,
    SezzleProcessingFee,
    SplititProcessingFee,
    PlatformBalanceProcessingFee,
    PaymentProcessingPercentageFee,
    PaymentProcessingFixedFee,
    CrossBorderPercentageFee,
    FxPercentageFee,
    OrchestrationPercentageFee,
    ThreeDsFixedFee,
    BillingPercentageFee,
    RevsharePercentageFee,
    ApplicationFee,
    HighRiskMerchantFee,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PaymentFeeOrigin {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::StripeDomesticProcessingFee => {
                serializer.serialize_str("stripe_domestic_processing_fee")
            }
            Self::StripeInternationalProcessingFee => {
                serializer.serialize_str("stripe_international_processing_fee")
            }
            Self::StripeFixedProcessingFee => {
                serializer.serialize_str("stripe_fixed_processing_fee")
            }
            Self::StripeBillingFee => serializer.serialize_str("stripe_billing_fee"),
            Self::StripeRadarFee => serializer.serialize_str("stripe_radar_fee"),
            Self::SalesTaxRemittance => serializer.serialize_str("sales_tax_remittance"),
            Self::SalesTaxRemittanceReversal => {
                serializer.serialize_str("sales_tax_remittance_reversal")
            }
            Self::StripeSalesTaxFee => serializer.serialize_str("stripe_sales_tax_fee"),
            Self::WhopProcessingFee => serializer.serialize_str("whop_processing_fee"),
            Self::MarketplaceAffiliateFee => serializer.serialize_str("marketplace_affiliate_fee"),
            Self::AffiliateFee => serializer.serialize_str("affiliate_fee"),
            Self::CryptoFee => serializer.serialize_str("crypto_fee"),
            Self::StripeStandardProcessingFee => {
                serializer.serialize_str("stripe_standard_processing_fee")
            }
            Self::PaypalFee => serializer.serialize_str("paypal_fee"),
            Self::StripePayoutFee => serializer.serialize_str("stripe_payout_fee"),
            Self::DisputeFee => serializer.serialize_str("dispute_fee"),
            Self::DisputeAlertFee => serializer.serialize_str("dispute_alert_fee"),
            Self::DisputeRepresentmentFee => serializer.serialize_str("dispute_representment_fee"),
            Self::AppleProcessingFee => serializer.serialize_str("apple_processing_fee"),
            Self::BuyerFee => serializer.serialize_str("buyer_fee"),
            Self::SezzleProcessingFee => serializer.serialize_str("sezzle_processing_fee"),
            Self::SplititProcessingFee => serializer.serialize_str("splitit_processing_fee"),
            Self::PlatformBalanceProcessingFee => {
                serializer.serialize_str("platform_balance_processing_fee")
            }
            Self::PaymentProcessingPercentageFee => {
                serializer.serialize_str("payment_processing_percentage_fee")
            }
            Self::PaymentProcessingFixedFee => {
                serializer.serialize_str("payment_processing_fixed_fee")
            }
            Self::CrossBorderPercentageFee => {
                serializer.serialize_str("cross_border_percentage_fee")
            }
            Self::FxPercentageFee => serializer.serialize_str("fx_percentage_fee"),
            Self::OrchestrationPercentageFee => {
                serializer.serialize_str("orchestration_percentage_fee")
            }
            Self::ThreeDsFixedFee => serializer.serialize_str("three_ds_fixed_fee"),
            Self::BillingPercentageFee => serializer.serialize_str("billing_percentage_fee"),
            Self::RevsharePercentageFee => serializer.serialize_str("revshare_percentage_fee"),
            Self::ApplicationFee => serializer.serialize_str("application_fee"),
            Self::HighRiskMerchantFee => serializer.serialize_str("high_risk_merchant_fee"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PaymentFeeOrigin {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "stripe_domestic_processing_fee" => Ok(Self::StripeDomesticProcessingFee),
            "stripe_international_processing_fee" => Ok(Self::StripeInternationalProcessingFee),
            "stripe_fixed_processing_fee" => Ok(Self::StripeFixedProcessingFee),
            "stripe_billing_fee" => Ok(Self::StripeBillingFee),
            "stripe_radar_fee" => Ok(Self::StripeRadarFee),
            "sales_tax_remittance" => Ok(Self::SalesTaxRemittance),
            "sales_tax_remittance_reversal" => Ok(Self::SalesTaxRemittanceReversal),
            "stripe_sales_tax_fee" => Ok(Self::StripeSalesTaxFee),
            "whop_processing_fee" => Ok(Self::WhopProcessingFee),
            "marketplace_affiliate_fee" => Ok(Self::MarketplaceAffiliateFee),
            "affiliate_fee" => Ok(Self::AffiliateFee),
            "crypto_fee" => Ok(Self::CryptoFee),
            "stripe_standard_processing_fee" => Ok(Self::StripeStandardProcessingFee),
            "paypal_fee" => Ok(Self::PaypalFee),
            "stripe_payout_fee" => Ok(Self::StripePayoutFee),
            "dispute_fee" => Ok(Self::DisputeFee),
            "dispute_alert_fee" => Ok(Self::DisputeAlertFee),
            "dispute_representment_fee" => Ok(Self::DisputeRepresentmentFee),
            "apple_processing_fee" => Ok(Self::AppleProcessingFee),
            "buyer_fee" => Ok(Self::BuyerFee),
            "sezzle_processing_fee" => Ok(Self::SezzleProcessingFee),
            "splitit_processing_fee" => Ok(Self::SplititProcessingFee),
            "platform_balance_processing_fee" => Ok(Self::PlatformBalanceProcessingFee),
            "payment_processing_percentage_fee" => Ok(Self::PaymentProcessingPercentageFee),
            "payment_processing_fixed_fee" => Ok(Self::PaymentProcessingFixedFee),
            "cross_border_percentage_fee" => Ok(Self::CrossBorderPercentageFee),
            "fx_percentage_fee" => Ok(Self::FxPercentageFee),
            "orchestration_percentage_fee" => Ok(Self::OrchestrationPercentageFee),
            "three_ds_fixed_fee" => Ok(Self::ThreeDsFixedFee),
            "billing_percentage_fee" => Ok(Self::BillingPercentageFee),
            "revshare_percentage_fee" => Ok(Self::RevsharePercentageFee),
            "application_fee" => Ok(Self::ApplicationFee),
            "high_risk_merchant_fee" => Ok(Self::HighRiskMerchantFee),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PaymentFeeOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StripeDomesticProcessingFee => write!(f, "stripe_domestic_processing_fee"),
            Self::StripeInternationalProcessingFee => {
                write!(f, "stripe_international_processing_fee")
            }
            Self::StripeFixedProcessingFee => write!(f, "stripe_fixed_processing_fee"),
            Self::StripeBillingFee => write!(f, "stripe_billing_fee"),
            Self::StripeRadarFee => write!(f, "stripe_radar_fee"),
            Self::SalesTaxRemittance => write!(f, "sales_tax_remittance"),
            Self::SalesTaxRemittanceReversal => write!(f, "sales_tax_remittance_reversal"),
            Self::StripeSalesTaxFee => write!(f, "stripe_sales_tax_fee"),
            Self::WhopProcessingFee => write!(f, "whop_processing_fee"),
            Self::MarketplaceAffiliateFee => write!(f, "marketplace_affiliate_fee"),
            Self::AffiliateFee => write!(f, "affiliate_fee"),
            Self::CryptoFee => write!(f, "crypto_fee"),
            Self::StripeStandardProcessingFee => write!(f, "stripe_standard_processing_fee"),
            Self::PaypalFee => write!(f, "paypal_fee"),
            Self::StripePayoutFee => write!(f, "stripe_payout_fee"),
            Self::DisputeFee => write!(f, "dispute_fee"),
            Self::DisputeAlertFee => write!(f, "dispute_alert_fee"),
            Self::DisputeRepresentmentFee => write!(f, "dispute_representment_fee"),
            Self::AppleProcessingFee => write!(f, "apple_processing_fee"),
            Self::BuyerFee => write!(f, "buyer_fee"),
            Self::SezzleProcessingFee => write!(f, "sezzle_processing_fee"),
            Self::SplititProcessingFee => write!(f, "splitit_processing_fee"),
            Self::PlatformBalanceProcessingFee => write!(f, "platform_balance_processing_fee"),
            Self::PaymentProcessingPercentageFee => write!(f, "payment_processing_percentage_fee"),
            Self::PaymentProcessingFixedFee => write!(f, "payment_processing_fixed_fee"),
            Self::CrossBorderPercentageFee => write!(f, "cross_border_percentage_fee"),
            Self::FxPercentageFee => write!(f, "fx_percentage_fee"),
            Self::OrchestrationPercentageFee => write!(f, "orchestration_percentage_fee"),
            Self::ThreeDsFixedFee => write!(f, "three_ds_fixed_fee"),
            Self::BillingPercentageFee => write!(f, "billing_percentage_fee"),
            Self::RevsharePercentageFee => write!(f, "revshare_percentage_fee"),
            Self::ApplicationFee => write!(f, "application_fee"),
            Self::HighRiskMerchantFee => write!(f, "high_risk_merchant_fee"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
