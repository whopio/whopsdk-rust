pub use crate::prelude::*;

/// The ledger line category this activity was posted under.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LedgerActivityLineType {
    AdBudgetRelease,
    AdCampaignBudget,
    AdPublisherPayout,
    AdPublisherPayoutReceived,
    AdSpendCharge,
    AffiliateFee,
    Airdrop,
    AirdropLinkCreated,
    AirdropLinkRedeemed,
    AirdropLinkReturned,
    AirdropReversal,
    ApplicationFee,
    ApplicationFeePayout,
    BalanceReservation,
    BalanceReservationReversal,
    BankTransfer,
    BillingPercentageFee,
    BuyerFee,
    CardSpendAuthorization,
    CardSpendAuthorizationVoid,
    CardSpendRefund,
    CompanyReferral,
    CrossBorderPercentageFee,
    CurrencyConversionIncoming,
    CurrencyConversionOutgoing,
    DisputeAlertFee,
    DisputeHoldAdjustment,
    DisputeRepresentmentFee,
    FraudPreventionFee,
    FxPercentageFee,
    HighRiskMerchantFee,
    InstallmentDefault,
    InternalBalanceTransferIncoming,
    InternalBalanceTransferOutgoing,
    InternalWithdrawal,
    InternalWithdrawalReversal,
    LegacyCryptoPayment,
    LegacyPayment,
    LegacyPaymentRefund,
    LicenseSale,
    LicenseSaleCommission,
    LicenseSaleRevenue,
    MiscPurchase,
    MiscRefund,
    MiscReversal,
    OnchainDeposit,
    OnchainSwapTarget,
    OnchainWalletTransferIncoming,
    OnchainWalletTransferOutgoing,
    OrchestrationPercentageFee,
    PassthroughGmv,
    PaymentDispute,
    PaymentDisputeAdjustment,
    PaymentDisputeFee,
    PaymentDisputeReversal,
    PaymentGross,
    PaymentGrossReversal,
    PaymentProcessingFixedFee,
    PaymentProcessingPercentageFee,
    PaymentReferral,
    PaymentReferralReversal,
    PaymentRefund,
    PaymentRefundReversal,
    PaymentRevshare,
    PaymentRevsharePayout,
    PaymentRevshareRefund,
    PaymentRevshareReversal,
    PayoutFee,
    PlatformAffiliatePayment,
    PlatformAffiliatePaymentReversal,
    PlatformBalancePayment,
    PlatformBalancePaymentRefund,
    PlatformBalanceTransferIncoming,
    PlatformBalanceTransferOutgoing,
    PlatformCoveredDispute,
    PromoReversal,
    ReferralBonus,
    ResolutionCenterRefund,
    RevsharePercentageFee,
    SalesTaxFee,
    SalesTaxRemittance,
    SalesTaxRemittanceReversal,
    SoftwareRentalRevshare,
    SoftwareRentalTransaction,
    StripeDomesticProcessingFee,
    StripeInternationalProcessingFee,
    ThreeDsFixedFee,
    Topup,
    TopupFee,
    TopupReversal,
    TreasuryPayin,
    WhopProcessingFee,
    Withdrawal,
    WithdrawalClawback,
    WithdrawalClawbackReversal,
    WithdrawalReclassification,
    WithdrawalReversal,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LedgerActivityLineType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AdBudgetRelease => serializer.serialize_str("ad_budget_release"),
            Self::AdCampaignBudget => serializer.serialize_str("ad_campaign_budget"),
            Self::AdPublisherPayout => serializer.serialize_str("ad_publisher_payout"),
            Self::AdPublisherPayoutReceived => {
                serializer.serialize_str("ad_publisher_payout_received")
            }
            Self::AdSpendCharge => serializer.serialize_str("ad_spend_charge"),
            Self::AffiliateFee => serializer.serialize_str("affiliate_fee"),
            Self::Airdrop => serializer.serialize_str("airdrop"),
            Self::AirdropLinkCreated => serializer.serialize_str("airdrop_link_created"),
            Self::AirdropLinkRedeemed => serializer.serialize_str("airdrop_link_redeemed"),
            Self::AirdropLinkReturned => serializer.serialize_str("airdrop_link_returned"),
            Self::AirdropReversal => serializer.serialize_str("airdrop_reversal"),
            Self::ApplicationFee => serializer.serialize_str("application_fee"),
            Self::ApplicationFeePayout => serializer.serialize_str("application_fee_payout"),
            Self::BalanceReservation => serializer.serialize_str("balance_reservation"),
            Self::BalanceReservationReversal => {
                serializer.serialize_str("balance_reservation_reversal")
            }
            Self::BankTransfer => serializer.serialize_str("bank_transfer"),
            Self::BillingPercentageFee => serializer.serialize_str("billing_percentage_fee"),
            Self::BuyerFee => serializer.serialize_str("buyer_fee"),
            Self::CardSpendAuthorization => serializer.serialize_str("card_spend_authorization"),
            Self::CardSpendAuthorizationVoid => {
                serializer.serialize_str("card_spend_authorization_void")
            }
            Self::CardSpendRefund => serializer.serialize_str("card_spend_refund"),
            Self::CompanyReferral => serializer.serialize_str("company_referral"),
            Self::CrossBorderPercentageFee => {
                serializer.serialize_str("cross_border_percentage_fee")
            }
            Self::CurrencyConversionIncoming => {
                serializer.serialize_str("currency_conversion_incoming")
            }
            Self::CurrencyConversionOutgoing => {
                serializer.serialize_str("currency_conversion_outgoing")
            }
            Self::DisputeAlertFee => serializer.serialize_str("dispute_alert_fee"),
            Self::DisputeHoldAdjustment => serializer.serialize_str("dispute_hold_adjustment"),
            Self::DisputeRepresentmentFee => serializer.serialize_str("dispute_representment_fee"),
            Self::FraudPreventionFee => serializer.serialize_str("fraud_prevention_fee"),
            Self::FxPercentageFee => serializer.serialize_str("fx_percentage_fee"),
            Self::HighRiskMerchantFee => serializer.serialize_str("high_risk_merchant_fee"),
            Self::InstallmentDefault => serializer.serialize_str("installment_default"),
            Self::InternalBalanceTransferIncoming => {
                serializer.serialize_str("internal_balance_transfer_incoming")
            }
            Self::InternalBalanceTransferOutgoing => {
                serializer.serialize_str("internal_balance_transfer_outgoing")
            }
            Self::InternalWithdrawal => serializer.serialize_str("internal_withdrawal"),
            Self::InternalWithdrawalReversal => {
                serializer.serialize_str("internal_withdrawal_reversal")
            }
            Self::LegacyCryptoPayment => serializer.serialize_str("legacy_crypto_payment"),
            Self::LegacyPayment => serializer.serialize_str("legacy_payment"),
            Self::LegacyPaymentRefund => serializer.serialize_str("legacy_payment_refund"),
            Self::LicenseSale => serializer.serialize_str("license_sale"),
            Self::LicenseSaleCommission => serializer.serialize_str("license_sale_commission"),
            Self::LicenseSaleRevenue => serializer.serialize_str("license_sale_revenue"),
            Self::MiscPurchase => serializer.serialize_str("misc_purchase"),
            Self::MiscRefund => serializer.serialize_str("misc_refund"),
            Self::MiscReversal => serializer.serialize_str("misc_reversal"),
            Self::OnchainDeposit => serializer.serialize_str("onchain_deposit"),
            Self::OnchainSwapTarget => serializer.serialize_str("onchain_swap_target"),
            Self::OnchainWalletTransferIncoming => {
                serializer.serialize_str("onchain_wallet_transfer_incoming")
            }
            Self::OnchainWalletTransferOutgoing => {
                serializer.serialize_str("onchain_wallet_transfer_outgoing")
            }
            Self::OrchestrationPercentageFee => {
                serializer.serialize_str("orchestration_percentage_fee")
            }
            Self::PassthroughGmv => serializer.serialize_str("passthrough_gmv"),
            Self::PaymentDispute => serializer.serialize_str("payment_dispute"),
            Self::PaymentDisputeAdjustment => {
                serializer.serialize_str("payment_dispute_adjustment")
            }
            Self::PaymentDisputeFee => serializer.serialize_str("payment_dispute_fee"),
            Self::PaymentDisputeReversal => serializer.serialize_str("payment_dispute_reversal"),
            Self::PaymentGross => serializer.serialize_str("payment_gross"),
            Self::PaymentGrossReversal => serializer.serialize_str("payment_gross_reversal"),
            Self::PaymentProcessingFixedFee => {
                serializer.serialize_str("payment_processing_fixed_fee")
            }
            Self::PaymentProcessingPercentageFee => {
                serializer.serialize_str("payment_processing_percentage_fee")
            }
            Self::PaymentReferral => serializer.serialize_str("payment_referral"),
            Self::PaymentReferralReversal => serializer.serialize_str("payment_referral_reversal"),
            Self::PaymentRefund => serializer.serialize_str("payment_refund"),
            Self::PaymentRefundReversal => serializer.serialize_str("payment_refund_reversal"),
            Self::PaymentRevshare => serializer.serialize_str("payment_revshare"),
            Self::PaymentRevsharePayout => serializer.serialize_str("payment_revshare_payout"),
            Self::PaymentRevshareRefund => serializer.serialize_str("payment_revshare_refund"),
            Self::PaymentRevshareReversal => serializer.serialize_str("payment_revshare_reversal"),
            Self::PayoutFee => serializer.serialize_str("payout_fee"),
            Self::PlatformAffiliatePayment => {
                serializer.serialize_str("platform_affiliate_payment")
            }
            Self::PlatformAffiliatePaymentReversal => {
                serializer.serialize_str("platform_affiliate_payment_reversal")
            }
            Self::PlatformBalancePayment => serializer.serialize_str("platform_balance_payment"),
            Self::PlatformBalancePaymentRefund => {
                serializer.serialize_str("platform_balance_payment_refund")
            }
            Self::PlatformBalanceTransferIncoming => {
                serializer.serialize_str("platform_balance_transfer_incoming")
            }
            Self::PlatformBalanceTransferOutgoing => {
                serializer.serialize_str("platform_balance_transfer_outgoing")
            }
            Self::PlatformCoveredDispute => serializer.serialize_str("platform_covered_dispute"),
            Self::PromoReversal => serializer.serialize_str("promo_reversal"),
            Self::ReferralBonus => serializer.serialize_str("referral_bonus"),
            Self::ResolutionCenterRefund => serializer.serialize_str("resolution_center_refund"),
            Self::RevsharePercentageFee => serializer.serialize_str("revshare_percentage_fee"),
            Self::SalesTaxFee => serializer.serialize_str("sales_tax_fee"),
            Self::SalesTaxRemittance => serializer.serialize_str("sales_tax_remittance"),
            Self::SalesTaxRemittanceReversal => {
                serializer.serialize_str("sales_tax_remittance_reversal")
            }
            Self::SoftwareRentalRevshare => serializer.serialize_str("software_rental_revshare"),
            Self::SoftwareRentalTransaction => {
                serializer.serialize_str("software_rental_transaction")
            }
            Self::StripeDomesticProcessingFee => {
                serializer.serialize_str("stripe_domestic_processing_fee")
            }
            Self::StripeInternationalProcessingFee => {
                serializer.serialize_str("stripe_international_processing_fee")
            }
            Self::ThreeDsFixedFee => serializer.serialize_str("three_ds_fixed_fee"),
            Self::Topup => serializer.serialize_str("topup"),
            Self::TopupFee => serializer.serialize_str("topup_fee"),
            Self::TopupReversal => serializer.serialize_str("topup_reversal"),
            Self::TreasuryPayin => serializer.serialize_str("treasury_payin"),
            Self::WhopProcessingFee => serializer.serialize_str("whop_processing_fee"),
            Self::Withdrawal => serializer.serialize_str("withdrawal"),
            Self::WithdrawalClawback => serializer.serialize_str("withdrawal_clawback"),
            Self::WithdrawalClawbackReversal => {
                serializer.serialize_str("withdrawal_clawback_reversal")
            }
            Self::WithdrawalReclassification => {
                serializer.serialize_str("withdrawal_reclassification")
            }
            Self::WithdrawalReversal => serializer.serialize_str("withdrawal_reversal"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LedgerActivityLineType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "ad_budget_release" => Ok(Self::AdBudgetRelease),
            "ad_campaign_budget" => Ok(Self::AdCampaignBudget),
            "ad_publisher_payout" => Ok(Self::AdPublisherPayout),
            "ad_publisher_payout_received" => Ok(Self::AdPublisherPayoutReceived),
            "ad_spend_charge" => Ok(Self::AdSpendCharge),
            "affiliate_fee" => Ok(Self::AffiliateFee),
            "airdrop" => Ok(Self::Airdrop),
            "airdrop_link_created" => Ok(Self::AirdropLinkCreated),
            "airdrop_link_redeemed" => Ok(Self::AirdropLinkRedeemed),
            "airdrop_link_returned" => Ok(Self::AirdropLinkReturned),
            "airdrop_reversal" => Ok(Self::AirdropReversal),
            "application_fee" => Ok(Self::ApplicationFee),
            "application_fee_payout" => Ok(Self::ApplicationFeePayout),
            "balance_reservation" => Ok(Self::BalanceReservation),
            "balance_reservation_reversal" => Ok(Self::BalanceReservationReversal),
            "bank_transfer" => Ok(Self::BankTransfer),
            "billing_percentage_fee" => Ok(Self::BillingPercentageFee),
            "buyer_fee" => Ok(Self::BuyerFee),
            "card_spend_authorization" => Ok(Self::CardSpendAuthorization),
            "card_spend_authorization_void" => Ok(Self::CardSpendAuthorizationVoid),
            "card_spend_refund" => Ok(Self::CardSpendRefund),
            "company_referral" => Ok(Self::CompanyReferral),
            "cross_border_percentage_fee" => Ok(Self::CrossBorderPercentageFee),
            "currency_conversion_incoming" => Ok(Self::CurrencyConversionIncoming),
            "currency_conversion_outgoing" => Ok(Self::CurrencyConversionOutgoing),
            "dispute_alert_fee" => Ok(Self::DisputeAlertFee),
            "dispute_hold_adjustment" => Ok(Self::DisputeHoldAdjustment),
            "dispute_representment_fee" => Ok(Self::DisputeRepresentmentFee),
            "fraud_prevention_fee" => Ok(Self::FraudPreventionFee),
            "fx_percentage_fee" => Ok(Self::FxPercentageFee),
            "high_risk_merchant_fee" => Ok(Self::HighRiskMerchantFee),
            "installment_default" => Ok(Self::InstallmentDefault),
            "internal_balance_transfer_incoming" => Ok(Self::InternalBalanceTransferIncoming),
            "internal_balance_transfer_outgoing" => Ok(Self::InternalBalanceTransferOutgoing),
            "internal_withdrawal" => Ok(Self::InternalWithdrawal),
            "internal_withdrawal_reversal" => Ok(Self::InternalWithdrawalReversal),
            "legacy_crypto_payment" => Ok(Self::LegacyCryptoPayment),
            "legacy_payment" => Ok(Self::LegacyPayment),
            "legacy_payment_refund" => Ok(Self::LegacyPaymentRefund),
            "license_sale" => Ok(Self::LicenseSale),
            "license_sale_commission" => Ok(Self::LicenseSaleCommission),
            "license_sale_revenue" => Ok(Self::LicenseSaleRevenue),
            "misc_purchase" => Ok(Self::MiscPurchase),
            "misc_refund" => Ok(Self::MiscRefund),
            "misc_reversal" => Ok(Self::MiscReversal),
            "onchain_deposit" => Ok(Self::OnchainDeposit),
            "onchain_swap_target" => Ok(Self::OnchainSwapTarget),
            "onchain_wallet_transfer_incoming" => Ok(Self::OnchainWalletTransferIncoming),
            "onchain_wallet_transfer_outgoing" => Ok(Self::OnchainWalletTransferOutgoing),
            "orchestration_percentage_fee" => Ok(Self::OrchestrationPercentageFee),
            "passthrough_gmv" => Ok(Self::PassthroughGmv),
            "payment_dispute" => Ok(Self::PaymentDispute),
            "payment_dispute_adjustment" => Ok(Self::PaymentDisputeAdjustment),
            "payment_dispute_fee" => Ok(Self::PaymentDisputeFee),
            "payment_dispute_reversal" => Ok(Self::PaymentDisputeReversal),
            "payment_gross" => Ok(Self::PaymentGross),
            "payment_gross_reversal" => Ok(Self::PaymentGrossReversal),
            "payment_processing_fixed_fee" => Ok(Self::PaymentProcessingFixedFee),
            "payment_processing_percentage_fee" => Ok(Self::PaymentProcessingPercentageFee),
            "payment_referral" => Ok(Self::PaymentReferral),
            "payment_referral_reversal" => Ok(Self::PaymentReferralReversal),
            "payment_refund" => Ok(Self::PaymentRefund),
            "payment_refund_reversal" => Ok(Self::PaymentRefundReversal),
            "payment_revshare" => Ok(Self::PaymentRevshare),
            "payment_revshare_payout" => Ok(Self::PaymentRevsharePayout),
            "payment_revshare_refund" => Ok(Self::PaymentRevshareRefund),
            "payment_revshare_reversal" => Ok(Self::PaymentRevshareReversal),
            "payout_fee" => Ok(Self::PayoutFee),
            "platform_affiliate_payment" => Ok(Self::PlatformAffiliatePayment),
            "platform_affiliate_payment_reversal" => Ok(Self::PlatformAffiliatePaymentReversal),
            "platform_balance_payment" => Ok(Self::PlatformBalancePayment),
            "platform_balance_payment_refund" => Ok(Self::PlatformBalancePaymentRefund),
            "platform_balance_transfer_incoming" => Ok(Self::PlatformBalanceTransferIncoming),
            "platform_balance_transfer_outgoing" => Ok(Self::PlatformBalanceTransferOutgoing),
            "platform_covered_dispute" => Ok(Self::PlatformCoveredDispute),
            "promo_reversal" => Ok(Self::PromoReversal),
            "referral_bonus" => Ok(Self::ReferralBonus),
            "resolution_center_refund" => Ok(Self::ResolutionCenterRefund),
            "revshare_percentage_fee" => Ok(Self::RevsharePercentageFee),
            "sales_tax_fee" => Ok(Self::SalesTaxFee),
            "sales_tax_remittance" => Ok(Self::SalesTaxRemittance),
            "sales_tax_remittance_reversal" => Ok(Self::SalesTaxRemittanceReversal),
            "software_rental_revshare" => Ok(Self::SoftwareRentalRevshare),
            "software_rental_transaction" => Ok(Self::SoftwareRentalTransaction),
            "stripe_domestic_processing_fee" => Ok(Self::StripeDomesticProcessingFee),
            "stripe_international_processing_fee" => Ok(Self::StripeInternationalProcessingFee),
            "three_ds_fixed_fee" => Ok(Self::ThreeDsFixedFee),
            "topup" => Ok(Self::Topup),
            "topup_fee" => Ok(Self::TopupFee),
            "topup_reversal" => Ok(Self::TopupReversal),
            "treasury_payin" => Ok(Self::TreasuryPayin),
            "whop_processing_fee" => Ok(Self::WhopProcessingFee),
            "withdrawal" => Ok(Self::Withdrawal),
            "withdrawal_clawback" => Ok(Self::WithdrawalClawback),
            "withdrawal_clawback_reversal" => Ok(Self::WithdrawalClawbackReversal),
            "withdrawal_reclassification" => Ok(Self::WithdrawalReclassification),
            "withdrawal_reversal" => Ok(Self::WithdrawalReversal),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LedgerActivityLineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AdBudgetRelease => write!(f, "ad_budget_release"),
            Self::AdCampaignBudget => write!(f, "ad_campaign_budget"),
            Self::AdPublisherPayout => write!(f, "ad_publisher_payout"),
            Self::AdPublisherPayoutReceived => write!(f, "ad_publisher_payout_received"),
            Self::AdSpendCharge => write!(f, "ad_spend_charge"),
            Self::AffiliateFee => write!(f, "affiliate_fee"),
            Self::Airdrop => write!(f, "airdrop"),
            Self::AirdropLinkCreated => write!(f, "airdrop_link_created"),
            Self::AirdropLinkRedeemed => write!(f, "airdrop_link_redeemed"),
            Self::AirdropLinkReturned => write!(f, "airdrop_link_returned"),
            Self::AirdropReversal => write!(f, "airdrop_reversal"),
            Self::ApplicationFee => write!(f, "application_fee"),
            Self::ApplicationFeePayout => write!(f, "application_fee_payout"),
            Self::BalanceReservation => write!(f, "balance_reservation"),
            Self::BalanceReservationReversal => write!(f, "balance_reservation_reversal"),
            Self::BankTransfer => write!(f, "bank_transfer"),
            Self::BillingPercentageFee => write!(f, "billing_percentage_fee"),
            Self::BuyerFee => write!(f, "buyer_fee"),
            Self::CardSpendAuthorization => write!(f, "card_spend_authorization"),
            Self::CardSpendAuthorizationVoid => write!(f, "card_spend_authorization_void"),
            Self::CardSpendRefund => write!(f, "card_spend_refund"),
            Self::CompanyReferral => write!(f, "company_referral"),
            Self::CrossBorderPercentageFee => write!(f, "cross_border_percentage_fee"),
            Self::CurrencyConversionIncoming => write!(f, "currency_conversion_incoming"),
            Self::CurrencyConversionOutgoing => write!(f, "currency_conversion_outgoing"),
            Self::DisputeAlertFee => write!(f, "dispute_alert_fee"),
            Self::DisputeHoldAdjustment => write!(f, "dispute_hold_adjustment"),
            Self::DisputeRepresentmentFee => write!(f, "dispute_representment_fee"),
            Self::FraudPreventionFee => write!(f, "fraud_prevention_fee"),
            Self::FxPercentageFee => write!(f, "fx_percentage_fee"),
            Self::HighRiskMerchantFee => write!(f, "high_risk_merchant_fee"),
            Self::InstallmentDefault => write!(f, "installment_default"),
            Self::InternalBalanceTransferIncoming => {
                write!(f, "internal_balance_transfer_incoming")
            }
            Self::InternalBalanceTransferOutgoing => {
                write!(f, "internal_balance_transfer_outgoing")
            }
            Self::InternalWithdrawal => write!(f, "internal_withdrawal"),
            Self::InternalWithdrawalReversal => write!(f, "internal_withdrawal_reversal"),
            Self::LegacyCryptoPayment => write!(f, "legacy_crypto_payment"),
            Self::LegacyPayment => write!(f, "legacy_payment"),
            Self::LegacyPaymentRefund => write!(f, "legacy_payment_refund"),
            Self::LicenseSale => write!(f, "license_sale"),
            Self::LicenseSaleCommission => write!(f, "license_sale_commission"),
            Self::LicenseSaleRevenue => write!(f, "license_sale_revenue"),
            Self::MiscPurchase => write!(f, "misc_purchase"),
            Self::MiscRefund => write!(f, "misc_refund"),
            Self::MiscReversal => write!(f, "misc_reversal"),
            Self::OnchainDeposit => write!(f, "onchain_deposit"),
            Self::OnchainSwapTarget => write!(f, "onchain_swap_target"),
            Self::OnchainWalletTransferIncoming => write!(f, "onchain_wallet_transfer_incoming"),
            Self::OnchainWalletTransferOutgoing => write!(f, "onchain_wallet_transfer_outgoing"),
            Self::OrchestrationPercentageFee => write!(f, "orchestration_percentage_fee"),
            Self::PassthroughGmv => write!(f, "passthrough_gmv"),
            Self::PaymentDispute => write!(f, "payment_dispute"),
            Self::PaymentDisputeAdjustment => write!(f, "payment_dispute_adjustment"),
            Self::PaymentDisputeFee => write!(f, "payment_dispute_fee"),
            Self::PaymentDisputeReversal => write!(f, "payment_dispute_reversal"),
            Self::PaymentGross => write!(f, "payment_gross"),
            Self::PaymentGrossReversal => write!(f, "payment_gross_reversal"),
            Self::PaymentProcessingFixedFee => write!(f, "payment_processing_fixed_fee"),
            Self::PaymentProcessingPercentageFee => write!(f, "payment_processing_percentage_fee"),
            Self::PaymentReferral => write!(f, "payment_referral"),
            Self::PaymentReferralReversal => write!(f, "payment_referral_reversal"),
            Self::PaymentRefund => write!(f, "payment_refund"),
            Self::PaymentRefundReversal => write!(f, "payment_refund_reversal"),
            Self::PaymentRevshare => write!(f, "payment_revshare"),
            Self::PaymentRevsharePayout => write!(f, "payment_revshare_payout"),
            Self::PaymentRevshareRefund => write!(f, "payment_revshare_refund"),
            Self::PaymentRevshareReversal => write!(f, "payment_revshare_reversal"),
            Self::PayoutFee => write!(f, "payout_fee"),
            Self::PlatformAffiliatePayment => write!(f, "platform_affiliate_payment"),
            Self::PlatformAffiliatePaymentReversal => {
                write!(f, "platform_affiliate_payment_reversal")
            }
            Self::PlatformBalancePayment => write!(f, "platform_balance_payment"),
            Self::PlatformBalancePaymentRefund => write!(f, "platform_balance_payment_refund"),
            Self::PlatformBalanceTransferIncoming => {
                write!(f, "platform_balance_transfer_incoming")
            }
            Self::PlatformBalanceTransferOutgoing => {
                write!(f, "platform_balance_transfer_outgoing")
            }
            Self::PlatformCoveredDispute => write!(f, "platform_covered_dispute"),
            Self::PromoReversal => write!(f, "promo_reversal"),
            Self::ReferralBonus => write!(f, "referral_bonus"),
            Self::ResolutionCenterRefund => write!(f, "resolution_center_refund"),
            Self::RevsharePercentageFee => write!(f, "revshare_percentage_fee"),
            Self::SalesTaxFee => write!(f, "sales_tax_fee"),
            Self::SalesTaxRemittance => write!(f, "sales_tax_remittance"),
            Self::SalesTaxRemittanceReversal => write!(f, "sales_tax_remittance_reversal"),
            Self::SoftwareRentalRevshare => write!(f, "software_rental_revshare"),
            Self::SoftwareRentalTransaction => write!(f, "software_rental_transaction"),
            Self::StripeDomesticProcessingFee => write!(f, "stripe_domestic_processing_fee"),
            Self::StripeInternationalProcessingFee => {
                write!(f, "stripe_international_processing_fee")
            }
            Self::ThreeDsFixedFee => write!(f, "three_ds_fixed_fee"),
            Self::Topup => write!(f, "topup"),
            Self::TopupFee => write!(f, "topup_fee"),
            Self::TopupReversal => write!(f, "topup_reversal"),
            Self::TreasuryPayin => write!(f, "treasury_payin"),
            Self::WhopProcessingFee => write!(f, "whop_processing_fee"),
            Self::Withdrawal => write!(f, "withdrawal"),
            Self::WithdrawalClawback => write!(f, "withdrawal_clawback"),
            Self::WithdrawalClawbackReversal => write!(f, "withdrawal_clawback_reversal"),
            Self::WithdrawalReclassification => write!(f, "withdrawal_reclassification"),
            Self::WithdrawalReversal => write!(f, "withdrawal_reversal"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
