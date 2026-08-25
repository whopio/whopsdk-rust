pub use crate::prelude::*;

/// The ledger line category the row aggregates. Balance summary rows carry the balance bucket instead.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GetFinancialReportResponseRowsItemLineCategory {
    AcceleratedSettlementFee,
    AccountSettlement,
    AdBalanceFundingReceipt,
    AdBudgetRelease,
    AdCampaignBudget,
    AdFundingDisbursement,
    AdIncomeExpense,
    AdIncomeReceipt,
    AdNetworkCost,
    AdNetworkSettlement,
    AdPublisherPayout,
    AdPublisherPayoutReceived,
    AdSpendCharge,
    AdSpendMargin,
    AdsCardSpread,
    AffiliateFee,
    AggregatedFee,
    Airdrop,
    AirdropExpense,
    AirdropExpenseReversal,
    AirdropLinkCanceled,
    AirdropLinkClaimed,
    AirdropLinkCreated,
    AirdropLinkFunded,
    AirdropLinkRedeemed,
    AirdropLinkReturned,
    AirdropReversal,
    ApplicationFee,
    ApplicationFeePayable,
    ApplicationFeePayout,
    Available,
    BadDebtExpense,
    BadDebtOffset,
    BalanceReservation,
    BalanceReservationHold,
    BalanceReservationRelease,
    BalanceReservationReversal,
    BillingPercentageFee,
    BuyerFee,
    CardInterchange,
    CardInterchangeReceivable,
    CardLoadDeposit,
    CardLoadTransfer,
    CardSpendAuthorization,
    CardSpendAuthorizationHold,
    CardSpendAuthorizationVoid,
    CardSpendAuthorizationVoidRelease,
    CardSpendCapture,
    CardSpendCaptureOffset,
    CardSpendRefund,
    CardSpendRefundOffset,
    CardUnloadDeposit,
    CardUnloadTransfer,
    ClawbackFee,
    ClawbackReceivable,
    ClawbackReceivableReversal,
    ClawbackReceivableSettlement,
    ClawbackReceivableSettlementReversal,
    CompanyReferral,
    ConnectedAccountClawback,
    ConnectedAccountNegativeBalance,
    CrossBorderPercentageFee,
    Crypto,
    CurrencyConversionIncoming,
    CurrencyConversionOutgoing,
    DisputeAlertFee,
    DisputeHold,
    DisputeHoldAdjustment,
    DisputeHoldBlocked,
    DisputeManagementFee,
    DisputePayableClearing,
    DisputePayableReversal,
    DisputeRepresentmentFee,
    DisputeSettlement,
    DisputeSettlementReversal,
    DisputeUnreconciledClearing,
    DisputeUnreconciledReversal,
    ExternalAccountConnectionFee,
    ExternalCardLoadDeposit,
    ExternalCardLoadOffset,
    FraudPreventionFee,
    FxGainLoss,
    FxMarkup,
    FxPercentageFee,
    FxSettlementGainLoss,
    HighRiskMerchantFee,
    IdentityVerificationFee,
    InstallmentDefault,
    InternalBalanceTransferIncoming,
    InternalBalanceTransferOutgoing,
    InternalWithdrawal,
    InternalWithdrawalComplete,
    InternalWithdrawalFee,
    InternalWithdrawalFeeReversal,
    InternalWithdrawalInTransit,
    InternalWithdrawalInTransitReversal,
    InternalWithdrawalMarkupFee,
    InternalWithdrawalMarkupFeePayout,
    InternalWithdrawalMarkupFeePayoutReversal,
    InternalWithdrawalMarkupFeeReversal,
    InternalWithdrawalPayable,
    InternalWithdrawalPayableReversal,
    InternalWithdrawalReversal,
    LegacyCryptoPayment,
    LegacyPayment,
    LegacyPaymentRefund,
    LicenseSale,
    LicenseSaleCommission,
    LicenseSaleRevenue,
    MarketplaceAffiliateFee,
    MiscPurchase,
    MiscRefund,
    MiscReversal,
    OnchainDeposit,
    OnchainDepositOffset,
    OnchainSwapOffset,
    OnchainSwapSource,
    OnchainSwapTarget,
    OnchainWalletTransferIncoming,
    OnchainWalletTransferOutgoing,
    OnchainWithdrawal,
    OnchainWithdrawalOffset,
    OrchestrationPercentageFee,
    PassthroughGmv,
    PassthroughGmvOffset,
    PaymentDispute,
    PaymentDisputeAdjustment,
    PaymentDisputeFee,
    PaymentDisputeReversal,
    PaymentGross,
    PaymentGrossReversal,
    PaymentProcessingFixedFee,
    PaymentProcessingPercentageFee,
    PaymentReceivableClearing,
    PaymentReceivableReversal,
    PaymentReceivableSettlement,
    PaymentReferral,
    PaymentReferralPayable,
    PaymentReferralRefund,
    PaymentReferralReversal,
    PaymentRefund,
    PaymentRefundFee,
    PaymentRefundReversal,
    PaymentRevshare,
    PaymentRevsharePayout,
    PaymentRevshareRefund,
    PaymentRevshareReversal,
    PaymentUnreconciledClearing,
    PaymentUnreconciledReversal,
    PayoutFee,
    PayoutReceivable,
    PayoutSubsidy,
    PayoutUnreconciledClearing,
    Pending,
    PlatformAffiliatePayment,
    PlatformAffiliatePaymentReversal,
    PlatformBalancePayment,
    PlatformBalancePaymentRefund,
    PlatformBalanceTransferFee,
    PlatformBalanceTransferIncoming,
    PlatformBalanceTransferOutgoing,
    PlatformCoveredDispute,
    PlatformEarning,
    PlatformEarningSettlement,
    PlatformEarningUnreconciledClearing,
    PromoReversal,
    PspAcceleratedSettlementFee,
    PspAccountUpdaterFee,
    PspAdjustedProcessingFee,
    PspAuthenticationFee,
    PspBankPullClearing,
    PspBankPullSettlement,
    PspBillingFee,
    PspClawbackFee,
    PspClawbackSettlement,
    PspClawbackSettlementReversal,
    PspCollectionSettlement,
    PspCommissionFee,
    PspConnectFee,
    PspCorrection,
    PspCurrencyConversionIncoming,
    PspCurrencyConversionOutgoing,
    PspCurrencyConversionReceivable,
    PspCurrencyConversionReceivableClearing,
    PspDisputeAlertFee,
    PspDisputeFee,
    PspDisputeManagementFee,
    PspDisputePayable,
    PspExternalAccountConnectionFee,
    PspFixedFee,
    PspGatewayFee,
    PspIdentityVerificationFee,
    PspInterchangeFee,
    PspInvoiceTaxFee,
    PspInvoicingFee,
    PspMarkupFee,
    PspNetworkTokenFee,
    PspOptimizationFee,
    PspPayinClearing,
    PspPayinSettlement,
    PspPaymentMethodFee,
    PspPaymentReceivable,
    PspPayoutConsolidation,
    PspPayoutDeposit,
    PspPayoutFee,
    PspPayoutReceivable,
    PspPayoutReceivableClearing,
    PspPayoutSettlement,
    PspPayoutSettlementReversal,
    PspPoolClearing,
    PspPoolDisputeSettlement,
    PspPoolSettlement,
    PspProcessingFee,
    PspReceivablePooled,
    PspRecipientWalletLoad,
    PspRefundFee,
    PspRefundPayable,
    PspReserveHold,
    PspReserveRelease,
    PspRiskFee,
    PspSchemeFee,
    PspServiceFee,
    PspTaxFilingFee,
    PspTaxServiceFee,
    PspTerminalFee,
    PspTransferSettlement,
    PspTransferSettlementReversal,
    PspTreasuryFee,
    PspVariableFee,
    PspWithholdingTax,
    RecipientWalletLoad,
    ReferralBonus,
    RefundPayableClearing,
    RefundPayableReversal,
    RefundSettlement,
    RefundSettlementReversal,
    RefundUnreconciledClearing,
    RefundUnreconciledReversal,
    Reserve,
    ResolutionCenterRefund,
    RevsharePercentageFee,
    SalesTaxCollected,
    SalesTaxCollectedReversal,
    SalesTaxFee,
    SalesTaxRemittance,
    SalesTaxRemittanceReversal,
    SalesTaxRemitted,
    SettlementRoundingVariance,
    SoftwareRentalRevshare,
    SoftwareRentalTransaction,
    StripeDomesticProcessingFee,
    StripeInternationalProcessingFee,
    SwapFee,
    TaxFilingFee,
    ThreeDsFixedFee,
    Topup,
    TopupFee,
    TopupReversal,
    TreasuryFee,
    TreasuryPayin,
    TreasuryPayinReceivable,
    WhopProcessingFee,
    WhopSwapFeeReceived,
    Withdrawal,
    WithdrawalClawback,
    WithdrawalClawbackReversal,
    WithdrawalFee,
    WithdrawalFeeReversal,
    WithdrawalMarkupFee,
    WithdrawalMarkupFeePayout,
    WithdrawalMarkupFeePayoutReversal,
    WithdrawalMarkupFeeReversal,
    WithdrawalPayable,
    WithdrawalPayableClearing,
    WithdrawalPayableClearingReversal,
    WithdrawalPayableReversal,
    WithdrawalReclassification,
    WithdrawalReversal,
    WithdrawalTopupAdjustment,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for GetFinancialReportResponseRowsItemLineCategory {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AcceleratedSettlementFee => {
                serializer.serialize_str("accelerated_settlement_fee")
            }
            Self::AccountSettlement => serializer.serialize_str("account_settlement"),
            Self::AdBalanceFundingReceipt => serializer.serialize_str("ad_balance_funding_receipt"),
            Self::AdBudgetRelease => serializer.serialize_str("ad_budget_release"),
            Self::AdCampaignBudget => serializer.serialize_str("ad_campaign_budget"),
            Self::AdFundingDisbursement => serializer.serialize_str("ad_funding_disbursement"),
            Self::AdIncomeExpense => serializer.serialize_str("ad_income_expense"),
            Self::AdIncomeReceipt => serializer.serialize_str("ad_income_receipt"),
            Self::AdNetworkCost => serializer.serialize_str("ad_network_cost"),
            Self::AdNetworkSettlement => serializer.serialize_str("ad_network_settlement"),
            Self::AdPublisherPayout => serializer.serialize_str("ad_publisher_payout"),
            Self::AdPublisherPayoutReceived => {
                serializer.serialize_str("ad_publisher_payout_received")
            }
            Self::AdSpendCharge => serializer.serialize_str("ad_spend_charge"),
            Self::AdSpendMargin => serializer.serialize_str("ad_spend_margin"),
            Self::AdsCardSpread => serializer.serialize_str("ads_card_spread"),
            Self::AffiliateFee => serializer.serialize_str("affiliate_fee"),
            Self::AggregatedFee => serializer.serialize_str("aggregated_fee"),
            Self::Airdrop => serializer.serialize_str("airdrop"),
            Self::AirdropExpense => serializer.serialize_str("airdrop_expense"),
            Self::AirdropExpenseReversal => serializer.serialize_str("airdrop_expense_reversal"),
            Self::AirdropLinkCanceled => serializer.serialize_str("airdrop_link_canceled"),
            Self::AirdropLinkClaimed => serializer.serialize_str("airdrop_link_claimed"),
            Self::AirdropLinkCreated => serializer.serialize_str("airdrop_link_created"),
            Self::AirdropLinkFunded => serializer.serialize_str("airdrop_link_funded"),
            Self::AirdropLinkRedeemed => serializer.serialize_str("airdrop_link_redeemed"),
            Self::AirdropLinkReturned => serializer.serialize_str("airdrop_link_returned"),
            Self::AirdropReversal => serializer.serialize_str("airdrop_reversal"),
            Self::ApplicationFee => serializer.serialize_str("application_fee"),
            Self::ApplicationFeePayable => serializer.serialize_str("application_fee_payable"),
            Self::ApplicationFeePayout => serializer.serialize_str("application_fee_payout"),
            Self::Available => serializer.serialize_str("available"),
            Self::BadDebtExpense => serializer.serialize_str("bad_debt_expense"),
            Self::BadDebtOffset => serializer.serialize_str("bad_debt_offset"),
            Self::BalanceReservation => serializer.serialize_str("balance_reservation"),
            Self::BalanceReservationHold => serializer.serialize_str("balance_reservation_hold"),
            Self::BalanceReservationRelease => {
                serializer.serialize_str("balance_reservation_release")
            }
            Self::BalanceReservationReversal => {
                serializer.serialize_str("balance_reservation_reversal")
            }
            Self::BillingPercentageFee => serializer.serialize_str("billing_percentage_fee"),
            Self::BuyerFee => serializer.serialize_str("buyer_fee"),
            Self::CardInterchange => serializer.serialize_str("card_interchange"),
            Self::CardInterchangeReceivable => {
                serializer.serialize_str("card_interchange_receivable")
            }
            Self::CardLoadDeposit => serializer.serialize_str("card_load_deposit"),
            Self::CardLoadTransfer => serializer.serialize_str("card_load_transfer"),
            Self::CardSpendAuthorization => serializer.serialize_str("card_spend_authorization"),
            Self::CardSpendAuthorizationHold => {
                serializer.serialize_str("card_spend_authorization_hold")
            }
            Self::CardSpendAuthorizationVoid => {
                serializer.serialize_str("card_spend_authorization_void")
            }
            Self::CardSpendAuthorizationVoidRelease => {
                serializer.serialize_str("card_spend_authorization_void_release")
            }
            Self::CardSpendCapture => serializer.serialize_str("card_spend_capture"),
            Self::CardSpendCaptureOffset => serializer.serialize_str("card_spend_capture_offset"),
            Self::CardSpendRefund => serializer.serialize_str("card_spend_refund"),
            Self::CardSpendRefundOffset => serializer.serialize_str("card_spend_refund_offset"),
            Self::CardUnloadDeposit => serializer.serialize_str("card_unload_deposit"),
            Self::CardUnloadTransfer => serializer.serialize_str("card_unload_transfer"),
            Self::ClawbackFee => serializer.serialize_str("clawback_fee"),
            Self::ClawbackReceivable => serializer.serialize_str("clawback_receivable"),
            Self::ClawbackReceivableReversal => {
                serializer.serialize_str("clawback_receivable_reversal")
            }
            Self::ClawbackReceivableSettlement => {
                serializer.serialize_str("clawback_receivable_settlement")
            }
            Self::ClawbackReceivableSettlementReversal => {
                serializer.serialize_str("clawback_receivable_settlement_reversal")
            }
            Self::CompanyReferral => serializer.serialize_str("company_referral"),
            Self::ConnectedAccountClawback => {
                serializer.serialize_str("connected_account_clawback")
            }
            Self::ConnectedAccountNegativeBalance => {
                serializer.serialize_str("connected_account_negative_balance")
            }
            Self::CrossBorderPercentageFee => {
                serializer.serialize_str("cross_border_percentage_fee")
            }
            Self::Crypto => serializer.serialize_str("crypto"),
            Self::CurrencyConversionIncoming => {
                serializer.serialize_str("currency_conversion_incoming")
            }
            Self::CurrencyConversionOutgoing => {
                serializer.serialize_str("currency_conversion_outgoing")
            }
            Self::DisputeAlertFee => serializer.serialize_str("dispute_alert_fee"),
            Self::DisputeHold => serializer.serialize_str("dispute_hold"),
            Self::DisputeHoldAdjustment => serializer.serialize_str("dispute_hold_adjustment"),
            Self::DisputeHoldBlocked => serializer.serialize_str("dispute_hold_blocked"),
            Self::DisputeManagementFee => serializer.serialize_str("dispute_management_fee"),
            Self::DisputePayableClearing => serializer.serialize_str("dispute_payable_clearing"),
            Self::DisputePayableReversal => serializer.serialize_str("dispute_payable_reversal"),
            Self::DisputeRepresentmentFee => serializer.serialize_str("dispute_representment_fee"),
            Self::DisputeSettlement => serializer.serialize_str("dispute_settlement"),
            Self::DisputeSettlementReversal => {
                serializer.serialize_str("dispute_settlement_reversal")
            }
            Self::DisputeUnreconciledClearing => {
                serializer.serialize_str("dispute_unreconciled_clearing")
            }
            Self::DisputeUnreconciledReversal => {
                serializer.serialize_str("dispute_unreconciled_reversal")
            }
            Self::ExternalAccountConnectionFee => {
                serializer.serialize_str("external_account_connection_fee")
            }
            Self::ExternalCardLoadDeposit => serializer.serialize_str("external_card_load_deposit"),
            Self::ExternalCardLoadOffset => serializer.serialize_str("external_card_load_offset"),
            Self::FraudPreventionFee => serializer.serialize_str("fraud_prevention_fee"),
            Self::FxGainLoss => serializer.serialize_str("fx_gain_loss"),
            Self::FxMarkup => serializer.serialize_str("fx_markup"),
            Self::FxPercentageFee => serializer.serialize_str("fx_percentage_fee"),
            Self::FxSettlementGainLoss => serializer.serialize_str("fx_settlement_gain_loss"),
            Self::HighRiskMerchantFee => serializer.serialize_str("high_risk_merchant_fee"),
            Self::IdentityVerificationFee => serializer.serialize_str("identity_verification_fee"),
            Self::InstallmentDefault => serializer.serialize_str("installment_default"),
            Self::InternalBalanceTransferIncoming => {
                serializer.serialize_str("internal_balance_transfer_incoming")
            }
            Self::InternalBalanceTransferOutgoing => {
                serializer.serialize_str("internal_balance_transfer_outgoing")
            }
            Self::InternalWithdrawal => serializer.serialize_str("internal_withdrawal"),
            Self::InternalWithdrawalComplete => {
                serializer.serialize_str("internal_withdrawal_complete")
            }
            Self::InternalWithdrawalFee => serializer.serialize_str("internal_withdrawal_fee"),
            Self::InternalWithdrawalFeeReversal => {
                serializer.serialize_str("internal_withdrawal_fee_reversal")
            }
            Self::InternalWithdrawalInTransit => {
                serializer.serialize_str("internal_withdrawal_in_transit")
            }
            Self::InternalWithdrawalInTransitReversal => {
                serializer.serialize_str("internal_withdrawal_in_transit_reversal")
            }
            Self::InternalWithdrawalMarkupFee => {
                serializer.serialize_str("internal_withdrawal_markup_fee")
            }
            Self::InternalWithdrawalMarkupFeePayout => {
                serializer.serialize_str("internal_withdrawal_markup_fee_payout")
            }
            Self::InternalWithdrawalMarkupFeePayoutReversal => {
                serializer.serialize_str("internal_withdrawal_markup_fee_payout_reversal")
            }
            Self::InternalWithdrawalMarkupFeeReversal => {
                serializer.serialize_str("internal_withdrawal_markup_fee_reversal")
            }
            Self::InternalWithdrawalPayable => {
                serializer.serialize_str("internal_withdrawal_payable")
            }
            Self::InternalWithdrawalPayableReversal => {
                serializer.serialize_str("internal_withdrawal_payable_reversal")
            }
            Self::InternalWithdrawalReversal => {
                serializer.serialize_str("internal_withdrawal_reversal")
            }
            Self::LegacyCryptoPayment => serializer.serialize_str("legacy_crypto_payment"),
            Self::LegacyPayment => serializer.serialize_str("legacy_payment"),
            Self::LegacyPaymentRefund => serializer.serialize_str("legacy_payment_refund"),
            Self::LicenseSale => serializer.serialize_str("license_sale"),
            Self::LicenseSaleCommission => serializer.serialize_str("license_sale_commission"),
            Self::LicenseSaleRevenue => serializer.serialize_str("license_sale_revenue"),
            Self::MarketplaceAffiliateFee => serializer.serialize_str("marketplace_affiliate_fee"),
            Self::MiscPurchase => serializer.serialize_str("misc_purchase"),
            Self::MiscRefund => serializer.serialize_str("misc_refund"),
            Self::MiscReversal => serializer.serialize_str("misc_reversal"),
            Self::OnchainDeposit => serializer.serialize_str("onchain_deposit"),
            Self::OnchainDepositOffset => serializer.serialize_str("onchain_deposit_offset"),
            Self::OnchainSwapOffset => serializer.serialize_str("onchain_swap_offset"),
            Self::OnchainSwapSource => serializer.serialize_str("onchain_swap_source"),
            Self::OnchainSwapTarget => serializer.serialize_str("onchain_swap_target"),
            Self::OnchainWalletTransferIncoming => {
                serializer.serialize_str("onchain_wallet_transfer_incoming")
            }
            Self::OnchainWalletTransferOutgoing => {
                serializer.serialize_str("onchain_wallet_transfer_outgoing")
            }
            Self::OnchainWithdrawal => serializer.serialize_str("onchain_withdrawal"),
            Self::OnchainWithdrawalOffset => serializer.serialize_str("onchain_withdrawal_offset"),
            Self::OrchestrationPercentageFee => {
                serializer.serialize_str("orchestration_percentage_fee")
            }
            Self::PassthroughGmv => serializer.serialize_str("passthrough_gmv"),
            Self::PassthroughGmvOffset => serializer.serialize_str("passthrough_gmv_offset"),
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
            Self::PaymentReceivableClearing => {
                serializer.serialize_str("payment_receivable_clearing")
            }
            Self::PaymentReceivableReversal => {
                serializer.serialize_str("payment_receivable_reversal")
            }
            Self::PaymentReceivableSettlement => {
                serializer.serialize_str("payment_receivable_settlement")
            }
            Self::PaymentReferral => serializer.serialize_str("payment_referral"),
            Self::PaymentReferralPayable => serializer.serialize_str("payment_referral_payable"),
            Self::PaymentReferralRefund => serializer.serialize_str("payment_referral_refund"),
            Self::PaymentReferralReversal => serializer.serialize_str("payment_referral_reversal"),
            Self::PaymentRefund => serializer.serialize_str("payment_refund"),
            Self::PaymentRefundFee => serializer.serialize_str("payment_refund_fee"),
            Self::PaymentRefundReversal => serializer.serialize_str("payment_refund_reversal"),
            Self::PaymentRevshare => serializer.serialize_str("payment_revshare"),
            Self::PaymentRevsharePayout => serializer.serialize_str("payment_revshare_payout"),
            Self::PaymentRevshareRefund => serializer.serialize_str("payment_revshare_refund"),
            Self::PaymentRevshareReversal => serializer.serialize_str("payment_revshare_reversal"),
            Self::PaymentUnreconciledClearing => {
                serializer.serialize_str("payment_unreconciled_clearing")
            }
            Self::PaymentUnreconciledReversal => {
                serializer.serialize_str("payment_unreconciled_reversal")
            }
            Self::PayoutFee => serializer.serialize_str("payout_fee"),
            Self::PayoutReceivable => serializer.serialize_str("payout_receivable"),
            Self::PayoutSubsidy => serializer.serialize_str("payout_subsidy"),
            Self::PayoutUnreconciledClearing => {
                serializer.serialize_str("payout_unreconciled_clearing")
            }
            Self::Pending => serializer.serialize_str("pending"),
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
            Self::PlatformBalanceTransferFee => {
                serializer.serialize_str("platform_balance_transfer_fee")
            }
            Self::PlatformBalanceTransferIncoming => {
                serializer.serialize_str("platform_balance_transfer_incoming")
            }
            Self::PlatformBalanceTransferOutgoing => {
                serializer.serialize_str("platform_balance_transfer_outgoing")
            }
            Self::PlatformCoveredDispute => serializer.serialize_str("platform_covered_dispute"),
            Self::PlatformEarning => serializer.serialize_str("platform_earning"),
            Self::PlatformEarningSettlement => {
                serializer.serialize_str("platform_earning_settlement")
            }
            Self::PlatformEarningUnreconciledClearing => {
                serializer.serialize_str("platform_earning_unreconciled_clearing")
            }
            Self::PromoReversal => serializer.serialize_str("promo_reversal"),
            Self::PspAcceleratedSettlementFee => {
                serializer.serialize_str("psp_accelerated_settlement_fee")
            }
            Self::PspAccountUpdaterFee => serializer.serialize_str("psp_account_updater_fee"),
            Self::PspAdjustedProcessingFee => {
                serializer.serialize_str("psp_adjusted_processing_fee")
            }
            Self::PspAuthenticationFee => serializer.serialize_str("psp_authentication_fee"),
            Self::PspBankPullClearing => serializer.serialize_str("psp_bank_pull_clearing"),
            Self::PspBankPullSettlement => serializer.serialize_str("psp_bank_pull_settlement"),
            Self::PspBillingFee => serializer.serialize_str("psp_billing_fee"),
            Self::PspClawbackFee => serializer.serialize_str("psp_clawback_fee"),
            Self::PspClawbackSettlement => serializer.serialize_str("psp_clawback_settlement"),
            Self::PspClawbackSettlementReversal => {
                serializer.serialize_str("psp_clawback_settlement_reversal")
            }
            Self::PspCollectionSettlement => serializer.serialize_str("psp_collection_settlement"),
            Self::PspCommissionFee => serializer.serialize_str("psp_commission_fee"),
            Self::PspConnectFee => serializer.serialize_str("psp_connect_fee"),
            Self::PspCorrection => serializer.serialize_str("psp_correction"),
            Self::PspCurrencyConversionIncoming => {
                serializer.serialize_str("psp_currency_conversion_incoming")
            }
            Self::PspCurrencyConversionOutgoing => {
                serializer.serialize_str("psp_currency_conversion_outgoing")
            }
            Self::PspCurrencyConversionReceivable => {
                serializer.serialize_str("psp_currency_conversion_receivable")
            }
            Self::PspCurrencyConversionReceivableClearing => {
                serializer.serialize_str("psp_currency_conversion_receivable_clearing")
            }
            Self::PspDisputeAlertFee => serializer.serialize_str("psp_dispute_alert_fee"),
            Self::PspDisputeFee => serializer.serialize_str("psp_dispute_fee"),
            Self::PspDisputeManagementFee => serializer.serialize_str("psp_dispute_management_fee"),
            Self::PspDisputePayable => serializer.serialize_str("psp_dispute_payable"),
            Self::PspExternalAccountConnectionFee => {
                serializer.serialize_str("psp_external_account_connection_fee")
            }
            Self::PspFixedFee => serializer.serialize_str("psp_fixed_fee"),
            Self::PspGatewayFee => serializer.serialize_str("psp_gateway_fee"),
            Self::PspIdentityVerificationFee => {
                serializer.serialize_str("psp_identity_verification_fee")
            }
            Self::PspInterchangeFee => serializer.serialize_str("psp_interchange_fee"),
            Self::PspInvoiceTaxFee => serializer.serialize_str("psp_invoice_tax_fee"),
            Self::PspInvoicingFee => serializer.serialize_str("psp_invoicing_fee"),
            Self::PspMarkupFee => serializer.serialize_str("psp_markup_fee"),
            Self::PspNetworkTokenFee => serializer.serialize_str("psp_network_token_fee"),
            Self::PspOptimizationFee => serializer.serialize_str("psp_optimization_fee"),
            Self::PspPayinClearing => serializer.serialize_str("psp_payin_clearing"),
            Self::PspPayinSettlement => serializer.serialize_str("psp_payin_settlement"),
            Self::PspPaymentMethodFee => serializer.serialize_str("psp_payment_method_fee"),
            Self::PspPaymentReceivable => serializer.serialize_str("psp_payment_receivable"),
            Self::PspPayoutConsolidation => serializer.serialize_str("psp_payout_consolidation"),
            Self::PspPayoutDeposit => serializer.serialize_str("psp_payout_deposit"),
            Self::PspPayoutFee => serializer.serialize_str("psp_payout_fee"),
            Self::PspPayoutReceivable => serializer.serialize_str("psp_payout_receivable"),
            Self::PspPayoutReceivableClearing => {
                serializer.serialize_str("psp_payout_receivable_clearing")
            }
            Self::PspPayoutSettlement => serializer.serialize_str("psp_payout_settlement"),
            Self::PspPayoutSettlementReversal => {
                serializer.serialize_str("psp_payout_settlement_reversal")
            }
            Self::PspPoolClearing => serializer.serialize_str("psp_pool_clearing"),
            Self::PspPoolDisputeSettlement => {
                serializer.serialize_str("psp_pool_dispute_settlement")
            }
            Self::PspPoolSettlement => serializer.serialize_str("psp_pool_settlement"),
            Self::PspProcessingFee => serializer.serialize_str("psp_processing_fee"),
            Self::PspReceivablePooled => serializer.serialize_str("psp_receivable_pooled"),
            Self::PspRecipientWalletLoad => serializer.serialize_str("psp_recipient_wallet_load"),
            Self::PspRefundFee => serializer.serialize_str("psp_refund_fee"),
            Self::PspRefundPayable => serializer.serialize_str("psp_refund_payable"),
            Self::PspReserveHold => serializer.serialize_str("psp_reserve_hold"),
            Self::PspReserveRelease => serializer.serialize_str("psp_reserve_release"),
            Self::PspRiskFee => serializer.serialize_str("psp_risk_fee"),
            Self::PspSchemeFee => serializer.serialize_str("psp_scheme_fee"),
            Self::PspServiceFee => serializer.serialize_str("psp_service_fee"),
            Self::PspTaxFilingFee => serializer.serialize_str("psp_tax_filing_fee"),
            Self::PspTaxServiceFee => serializer.serialize_str("psp_tax_service_fee"),
            Self::PspTerminalFee => serializer.serialize_str("psp_terminal_fee"),
            Self::PspTransferSettlement => serializer.serialize_str("psp_transfer_settlement"),
            Self::PspTransferSettlementReversal => {
                serializer.serialize_str("psp_transfer_settlement_reversal")
            }
            Self::PspTreasuryFee => serializer.serialize_str("psp_treasury_fee"),
            Self::PspVariableFee => serializer.serialize_str("psp_variable_fee"),
            Self::PspWithholdingTax => serializer.serialize_str("psp_withholding_tax"),
            Self::RecipientWalletLoad => serializer.serialize_str("recipient_wallet_load"),
            Self::ReferralBonus => serializer.serialize_str("referral_bonus"),
            Self::RefundPayableClearing => serializer.serialize_str("refund_payable_clearing"),
            Self::RefundPayableReversal => serializer.serialize_str("refund_payable_reversal"),
            Self::RefundSettlement => serializer.serialize_str("refund_settlement"),
            Self::RefundSettlementReversal => {
                serializer.serialize_str("refund_settlement_reversal")
            }
            Self::RefundUnreconciledClearing => {
                serializer.serialize_str("refund_unreconciled_clearing")
            }
            Self::RefundUnreconciledReversal => {
                serializer.serialize_str("refund_unreconciled_reversal")
            }
            Self::Reserve => serializer.serialize_str("reserve"),
            Self::ResolutionCenterRefund => serializer.serialize_str("resolution_center_refund"),
            Self::RevsharePercentageFee => serializer.serialize_str("revshare_percentage_fee"),
            Self::SalesTaxCollected => serializer.serialize_str("sales_tax_collected"),
            Self::SalesTaxCollectedReversal => {
                serializer.serialize_str("sales_tax_collected_reversal")
            }
            Self::SalesTaxFee => serializer.serialize_str("sales_tax_fee"),
            Self::SalesTaxRemittance => serializer.serialize_str("sales_tax_remittance"),
            Self::SalesTaxRemittanceReversal => {
                serializer.serialize_str("sales_tax_remittance_reversal")
            }
            Self::SalesTaxRemitted => serializer.serialize_str("sales_tax_remitted"),
            Self::SettlementRoundingVariance => {
                serializer.serialize_str("settlement_rounding_variance")
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
            Self::SwapFee => serializer.serialize_str("swap_fee"),
            Self::TaxFilingFee => serializer.serialize_str("tax_filing_fee"),
            Self::ThreeDsFixedFee => serializer.serialize_str("three_ds_fixed_fee"),
            Self::Topup => serializer.serialize_str("topup"),
            Self::TopupFee => serializer.serialize_str("topup_fee"),
            Self::TopupReversal => serializer.serialize_str("topup_reversal"),
            Self::TreasuryFee => serializer.serialize_str("treasury_fee"),
            Self::TreasuryPayin => serializer.serialize_str("treasury_payin"),
            Self::TreasuryPayinReceivable => serializer.serialize_str("treasury_payin_receivable"),
            Self::WhopProcessingFee => serializer.serialize_str("whop_processing_fee"),
            Self::WhopSwapFeeReceived => serializer.serialize_str("whop_swap_fee_received"),
            Self::Withdrawal => serializer.serialize_str("withdrawal"),
            Self::WithdrawalClawback => serializer.serialize_str("withdrawal_clawback"),
            Self::WithdrawalClawbackReversal => {
                serializer.serialize_str("withdrawal_clawback_reversal")
            }
            Self::WithdrawalFee => serializer.serialize_str("withdrawal_fee"),
            Self::WithdrawalFeeReversal => serializer.serialize_str("withdrawal_fee_reversal"),
            Self::WithdrawalMarkupFee => serializer.serialize_str("withdrawal_markup_fee"),
            Self::WithdrawalMarkupFeePayout => {
                serializer.serialize_str("withdrawal_markup_fee_payout")
            }
            Self::WithdrawalMarkupFeePayoutReversal => {
                serializer.serialize_str("withdrawal_markup_fee_payout_reversal")
            }
            Self::WithdrawalMarkupFeeReversal => {
                serializer.serialize_str("withdrawal_markup_fee_reversal")
            }
            Self::WithdrawalPayable => serializer.serialize_str("withdrawal_payable"),
            Self::WithdrawalPayableClearing => {
                serializer.serialize_str("withdrawal_payable_clearing")
            }
            Self::WithdrawalPayableClearingReversal => {
                serializer.serialize_str("withdrawal_payable_clearing_reversal")
            }
            Self::WithdrawalPayableReversal => {
                serializer.serialize_str("withdrawal_payable_reversal")
            }
            Self::WithdrawalReclassification => {
                serializer.serialize_str("withdrawal_reclassification")
            }
            Self::WithdrawalReversal => serializer.serialize_str("withdrawal_reversal"),
            Self::WithdrawalTopupAdjustment => {
                serializer.serialize_str("withdrawal_topup_adjustment")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for GetFinancialReportResponseRowsItemLineCategory {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "accelerated_settlement_fee" => Ok(Self::AcceleratedSettlementFee),
            "account_settlement" => Ok(Self::AccountSettlement),
            "ad_balance_funding_receipt" => Ok(Self::AdBalanceFundingReceipt),
            "ad_budget_release" => Ok(Self::AdBudgetRelease),
            "ad_campaign_budget" => Ok(Self::AdCampaignBudget),
            "ad_funding_disbursement" => Ok(Self::AdFundingDisbursement),
            "ad_income_expense" => Ok(Self::AdIncomeExpense),
            "ad_income_receipt" => Ok(Self::AdIncomeReceipt),
            "ad_network_cost" => Ok(Self::AdNetworkCost),
            "ad_network_settlement" => Ok(Self::AdNetworkSettlement),
            "ad_publisher_payout" => Ok(Self::AdPublisherPayout),
            "ad_publisher_payout_received" => Ok(Self::AdPublisherPayoutReceived),
            "ad_spend_charge" => Ok(Self::AdSpendCharge),
            "ad_spend_margin" => Ok(Self::AdSpendMargin),
            "ads_card_spread" => Ok(Self::AdsCardSpread),
            "affiliate_fee" => Ok(Self::AffiliateFee),
            "aggregated_fee" => Ok(Self::AggregatedFee),
            "airdrop" => Ok(Self::Airdrop),
            "airdrop_expense" => Ok(Self::AirdropExpense),
            "airdrop_expense_reversal" => Ok(Self::AirdropExpenseReversal),
            "airdrop_link_canceled" => Ok(Self::AirdropLinkCanceled),
            "airdrop_link_claimed" => Ok(Self::AirdropLinkClaimed),
            "airdrop_link_created" => Ok(Self::AirdropLinkCreated),
            "airdrop_link_funded" => Ok(Self::AirdropLinkFunded),
            "airdrop_link_redeemed" => Ok(Self::AirdropLinkRedeemed),
            "airdrop_link_returned" => Ok(Self::AirdropLinkReturned),
            "airdrop_reversal" => Ok(Self::AirdropReversal),
            "application_fee" => Ok(Self::ApplicationFee),
            "application_fee_payable" => Ok(Self::ApplicationFeePayable),
            "application_fee_payout" => Ok(Self::ApplicationFeePayout),
            "available" => Ok(Self::Available),
            "bad_debt_expense" => Ok(Self::BadDebtExpense),
            "bad_debt_offset" => Ok(Self::BadDebtOffset),
            "balance_reservation" => Ok(Self::BalanceReservation),
            "balance_reservation_hold" => Ok(Self::BalanceReservationHold),
            "balance_reservation_release" => Ok(Self::BalanceReservationRelease),
            "balance_reservation_reversal" => Ok(Self::BalanceReservationReversal),
            "billing_percentage_fee" => Ok(Self::BillingPercentageFee),
            "buyer_fee" => Ok(Self::BuyerFee),
            "card_interchange" => Ok(Self::CardInterchange),
            "card_interchange_receivable" => Ok(Self::CardInterchangeReceivable),
            "card_load_deposit" => Ok(Self::CardLoadDeposit),
            "card_load_transfer" => Ok(Self::CardLoadTransfer),
            "card_spend_authorization" => Ok(Self::CardSpendAuthorization),
            "card_spend_authorization_hold" => Ok(Self::CardSpendAuthorizationHold),
            "card_spend_authorization_void" => Ok(Self::CardSpendAuthorizationVoid),
            "card_spend_authorization_void_release" => Ok(Self::CardSpendAuthorizationVoidRelease),
            "card_spend_capture" => Ok(Self::CardSpendCapture),
            "card_spend_capture_offset" => Ok(Self::CardSpendCaptureOffset),
            "card_spend_refund" => Ok(Self::CardSpendRefund),
            "card_spend_refund_offset" => Ok(Self::CardSpendRefundOffset),
            "card_unload_deposit" => Ok(Self::CardUnloadDeposit),
            "card_unload_transfer" => Ok(Self::CardUnloadTransfer),
            "clawback_fee" => Ok(Self::ClawbackFee),
            "clawback_receivable" => Ok(Self::ClawbackReceivable),
            "clawback_receivable_reversal" => Ok(Self::ClawbackReceivableReversal),
            "clawback_receivable_settlement" => Ok(Self::ClawbackReceivableSettlement),
            "clawback_receivable_settlement_reversal" => {
                Ok(Self::ClawbackReceivableSettlementReversal)
            }
            "company_referral" => Ok(Self::CompanyReferral),
            "connected_account_clawback" => Ok(Self::ConnectedAccountClawback),
            "connected_account_negative_balance" => Ok(Self::ConnectedAccountNegativeBalance),
            "cross_border_percentage_fee" => Ok(Self::CrossBorderPercentageFee),
            "crypto" => Ok(Self::Crypto),
            "currency_conversion_incoming" => Ok(Self::CurrencyConversionIncoming),
            "currency_conversion_outgoing" => Ok(Self::CurrencyConversionOutgoing),
            "dispute_alert_fee" => Ok(Self::DisputeAlertFee),
            "dispute_hold" => Ok(Self::DisputeHold),
            "dispute_hold_adjustment" => Ok(Self::DisputeHoldAdjustment),
            "dispute_hold_blocked" => Ok(Self::DisputeHoldBlocked),
            "dispute_management_fee" => Ok(Self::DisputeManagementFee),
            "dispute_payable_clearing" => Ok(Self::DisputePayableClearing),
            "dispute_payable_reversal" => Ok(Self::DisputePayableReversal),
            "dispute_representment_fee" => Ok(Self::DisputeRepresentmentFee),
            "dispute_settlement" => Ok(Self::DisputeSettlement),
            "dispute_settlement_reversal" => Ok(Self::DisputeSettlementReversal),
            "dispute_unreconciled_clearing" => Ok(Self::DisputeUnreconciledClearing),
            "dispute_unreconciled_reversal" => Ok(Self::DisputeUnreconciledReversal),
            "external_account_connection_fee" => Ok(Self::ExternalAccountConnectionFee),
            "external_card_load_deposit" => Ok(Self::ExternalCardLoadDeposit),
            "external_card_load_offset" => Ok(Self::ExternalCardLoadOffset),
            "fraud_prevention_fee" => Ok(Self::FraudPreventionFee),
            "fx_gain_loss" => Ok(Self::FxGainLoss),
            "fx_markup" => Ok(Self::FxMarkup),
            "fx_percentage_fee" => Ok(Self::FxPercentageFee),
            "fx_settlement_gain_loss" => Ok(Self::FxSettlementGainLoss),
            "high_risk_merchant_fee" => Ok(Self::HighRiskMerchantFee),
            "identity_verification_fee" => Ok(Self::IdentityVerificationFee),
            "installment_default" => Ok(Self::InstallmentDefault),
            "internal_balance_transfer_incoming" => Ok(Self::InternalBalanceTransferIncoming),
            "internal_balance_transfer_outgoing" => Ok(Self::InternalBalanceTransferOutgoing),
            "internal_withdrawal" => Ok(Self::InternalWithdrawal),
            "internal_withdrawal_complete" => Ok(Self::InternalWithdrawalComplete),
            "internal_withdrawal_fee" => Ok(Self::InternalWithdrawalFee),
            "internal_withdrawal_fee_reversal" => Ok(Self::InternalWithdrawalFeeReversal),
            "internal_withdrawal_in_transit" => Ok(Self::InternalWithdrawalInTransit),
            "internal_withdrawal_in_transit_reversal" => {
                Ok(Self::InternalWithdrawalInTransitReversal)
            }
            "internal_withdrawal_markup_fee" => Ok(Self::InternalWithdrawalMarkupFee),
            "internal_withdrawal_markup_fee_payout" => Ok(Self::InternalWithdrawalMarkupFeePayout),
            "internal_withdrawal_markup_fee_payout_reversal" => {
                Ok(Self::InternalWithdrawalMarkupFeePayoutReversal)
            }
            "internal_withdrawal_markup_fee_reversal" => {
                Ok(Self::InternalWithdrawalMarkupFeeReversal)
            }
            "internal_withdrawal_payable" => Ok(Self::InternalWithdrawalPayable),
            "internal_withdrawal_payable_reversal" => Ok(Self::InternalWithdrawalPayableReversal),
            "internal_withdrawal_reversal" => Ok(Self::InternalWithdrawalReversal),
            "legacy_crypto_payment" => Ok(Self::LegacyCryptoPayment),
            "legacy_payment" => Ok(Self::LegacyPayment),
            "legacy_payment_refund" => Ok(Self::LegacyPaymentRefund),
            "license_sale" => Ok(Self::LicenseSale),
            "license_sale_commission" => Ok(Self::LicenseSaleCommission),
            "license_sale_revenue" => Ok(Self::LicenseSaleRevenue),
            "marketplace_affiliate_fee" => Ok(Self::MarketplaceAffiliateFee),
            "misc_purchase" => Ok(Self::MiscPurchase),
            "misc_refund" => Ok(Self::MiscRefund),
            "misc_reversal" => Ok(Self::MiscReversal),
            "onchain_deposit" => Ok(Self::OnchainDeposit),
            "onchain_deposit_offset" => Ok(Self::OnchainDepositOffset),
            "onchain_swap_offset" => Ok(Self::OnchainSwapOffset),
            "onchain_swap_source" => Ok(Self::OnchainSwapSource),
            "onchain_swap_target" => Ok(Self::OnchainSwapTarget),
            "onchain_wallet_transfer_incoming" => Ok(Self::OnchainWalletTransferIncoming),
            "onchain_wallet_transfer_outgoing" => Ok(Self::OnchainWalletTransferOutgoing),
            "onchain_withdrawal" => Ok(Self::OnchainWithdrawal),
            "onchain_withdrawal_offset" => Ok(Self::OnchainWithdrawalOffset),
            "orchestration_percentage_fee" => Ok(Self::OrchestrationPercentageFee),
            "passthrough_gmv" => Ok(Self::PassthroughGmv),
            "passthrough_gmv_offset" => Ok(Self::PassthroughGmvOffset),
            "payment_dispute" => Ok(Self::PaymentDispute),
            "payment_dispute_adjustment" => Ok(Self::PaymentDisputeAdjustment),
            "payment_dispute_fee" => Ok(Self::PaymentDisputeFee),
            "payment_dispute_reversal" => Ok(Self::PaymentDisputeReversal),
            "payment_gross" => Ok(Self::PaymentGross),
            "payment_gross_reversal" => Ok(Self::PaymentGrossReversal),
            "payment_processing_fixed_fee" => Ok(Self::PaymentProcessingFixedFee),
            "payment_processing_percentage_fee" => Ok(Self::PaymentProcessingPercentageFee),
            "payment_receivable_clearing" => Ok(Self::PaymentReceivableClearing),
            "payment_receivable_reversal" => Ok(Self::PaymentReceivableReversal),
            "payment_receivable_settlement" => Ok(Self::PaymentReceivableSettlement),
            "payment_referral" => Ok(Self::PaymentReferral),
            "payment_referral_payable" => Ok(Self::PaymentReferralPayable),
            "payment_referral_refund" => Ok(Self::PaymentReferralRefund),
            "payment_referral_reversal" => Ok(Self::PaymentReferralReversal),
            "payment_refund" => Ok(Self::PaymentRefund),
            "payment_refund_fee" => Ok(Self::PaymentRefundFee),
            "payment_refund_reversal" => Ok(Self::PaymentRefundReversal),
            "payment_revshare" => Ok(Self::PaymentRevshare),
            "payment_revshare_payout" => Ok(Self::PaymentRevsharePayout),
            "payment_revshare_refund" => Ok(Self::PaymentRevshareRefund),
            "payment_revshare_reversal" => Ok(Self::PaymentRevshareReversal),
            "payment_unreconciled_clearing" => Ok(Self::PaymentUnreconciledClearing),
            "payment_unreconciled_reversal" => Ok(Self::PaymentUnreconciledReversal),
            "payout_fee" => Ok(Self::PayoutFee),
            "payout_receivable" => Ok(Self::PayoutReceivable),
            "payout_subsidy" => Ok(Self::PayoutSubsidy),
            "payout_unreconciled_clearing" => Ok(Self::PayoutUnreconciledClearing),
            "pending" => Ok(Self::Pending),
            "platform_affiliate_payment" => Ok(Self::PlatformAffiliatePayment),
            "platform_affiliate_payment_reversal" => Ok(Self::PlatformAffiliatePaymentReversal),
            "platform_balance_payment" => Ok(Self::PlatformBalancePayment),
            "platform_balance_payment_refund" => Ok(Self::PlatformBalancePaymentRefund),
            "platform_balance_transfer_fee" => Ok(Self::PlatformBalanceTransferFee),
            "platform_balance_transfer_incoming" => Ok(Self::PlatformBalanceTransferIncoming),
            "platform_balance_transfer_outgoing" => Ok(Self::PlatformBalanceTransferOutgoing),
            "platform_covered_dispute" => Ok(Self::PlatformCoveredDispute),
            "platform_earning" => Ok(Self::PlatformEarning),
            "platform_earning_settlement" => Ok(Self::PlatformEarningSettlement),
            "platform_earning_unreconciled_clearing" => {
                Ok(Self::PlatformEarningUnreconciledClearing)
            }
            "promo_reversal" => Ok(Self::PromoReversal),
            "psp_accelerated_settlement_fee" => Ok(Self::PspAcceleratedSettlementFee),
            "psp_account_updater_fee" => Ok(Self::PspAccountUpdaterFee),
            "psp_adjusted_processing_fee" => Ok(Self::PspAdjustedProcessingFee),
            "psp_authentication_fee" => Ok(Self::PspAuthenticationFee),
            "psp_bank_pull_clearing" => Ok(Self::PspBankPullClearing),
            "psp_bank_pull_settlement" => Ok(Self::PspBankPullSettlement),
            "psp_billing_fee" => Ok(Self::PspBillingFee),
            "psp_clawback_fee" => Ok(Self::PspClawbackFee),
            "psp_clawback_settlement" => Ok(Self::PspClawbackSettlement),
            "psp_clawback_settlement_reversal" => Ok(Self::PspClawbackSettlementReversal),
            "psp_collection_settlement" => Ok(Self::PspCollectionSettlement),
            "psp_commission_fee" => Ok(Self::PspCommissionFee),
            "psp_connect_fee" => Ok(Self::PspConnectFee),
            "psp_correction" => Ok(Self::PspCorrection),
            "psp_currency_conversion_incoming" => Ok(Self::PspCurrencyConversionIncoming),
            "psp_currency_conversion_outgoing" => Ok(Self::PspCurrencyConversionOutgoing),
            "psp_currency_conversion_receivable" => Ok(Self::PspCurrencyConversionReceivable),
            "psp_currency_conversion_receivable_clearing" => {
                Ok(Self::PspCurrencyConversionReceivableClearing)
            }
            "psp_dispute_alert_fee" => Ok(Self::PspDisputeAlertFee),
            "psp_dispute_fee" => Ok(Self::PspDisputeFee),
            "psp_dispute_management_fee" => Ok(Self::PspDisputeManagementFee),
            "psp_dispute_payable" => Ok(Self::PspDisputePayable),
            "psp_external_account_connection_fee" => Ok(Self::PspExternalAccountConnectionFee),
            "psp_fixed_fee" => Ok(Self::PspFixedFee),
            "psp_gateway_fee" => Ok(Self::PspGatewayFee),
            "psp_identity_verification_fee" => Ok(Self::PspIdentityVerificationFee),
            "psp_interchange_fee" => Ok(Self::PspInterchangeFee),
            "psp_invoice_tax_fee" => Ok(Self::PspInvoiceTaxFee),
            "psp_invoicing_fee" => Ok(Self::PspInvoicingFee),
            "psp_markup_fee" => Ok(Self::PspMarkupFee),
            "psp_network_token_fee" => Ok(Self::PspNetworkTokenFee),
            "psp_optimization_fee" => Ok(Self::PspOptimizationFee),
            "psp_payin_clearing" => Ok(Self::PspPayinClearing),
            "psp_payin_settlement" => Ok(Self::PspPayinSettlement),
            "psp_payment_method_fee" => Ok(Self::PspPaymentMethodFee),
            "psp_payment_receivable" => Ok(Self::PspPaymentReceivable),
            "psp_payout_consolidation" => Ok(Self::PspPayoutConsolidation),
            "psp_payout_deposit" => Ok(Self::PspPayoutDeposit),
            "psp_payout_fee" => Ok(Self::PspPayoutFee),
            "psp_payout_receivable" => Ok(Self::PspPayoutReceivable),
            "psp_payout_receivable_clearing" => Ok(Self::PspPayoutReceivableClearing),
            "psp_payout_settlement" => Ok(Self::PspPayoutSettlement),
            "psp_payout_settlement_reversal" => Ok(Self::PspPayoutSettlementReversal),
            "psp_pool_clearing" => Ok(Self::PspPoolClearing),
            "psp_pool_dispute_settlement" => Ok(Self::PspPoolDisputeSettlement),
            "psp_pool_settlement" => Ok(Self::PspPoolSettlement),
            "psp_processing_fee" => Ok(Self::PspProcessingFee),
            "psp_receivable_pooled" => Ok(Self::PspReceivablePooled),
            "psp_recipient_wallet_load" => Ok(Self::PspRecipientWalletLoad),
            "psp_refund_fee" => Ok(Self::PspRefundFee),
            "psp_refund_payable" => Ok(Self::PspRefundPayable),
            "psp_reserve_hold" => Ok(Self::PspReserveHold),
            "psp_reserve_release" => Ok(Self::PspReserveRelease),
            "psp_risk_fee" => Ok(Self::PspRiskFee),
            "psp_scheme_fee" => Ok(Self::PspSchemeFee),
            "psp_service_fee" => Ok(Self::PspServiceFee),
            "psp_tax_filing_fee" => Ok(Self::PspTaxFilingFee),
            "psp_tax_service_fee" => Ok(Self::PspTaxServiceFee),
            "psp_terminal_fee" => Ok(Self::PspTerminalFee),
            "psp_transfer_settlement" => Ok(Self::PspTransferSettlement),
            "psp_transfer_settlement_reversal" => Ok(Self::PspTransferSettlementReversal),
            "psp_treasury_fee" => Ok(Self::PspTreasuryFee),
            "psp_variable_fee" => Ok(Self::PspVariableFee),
            "psp_withholding_tax" => Ok(Self::PspWithholdingTax),
            "recipient_wallet_load" => Ok(Self::RecipientWalletLoad),
            "referral_bonus" => Ok(Self::ReferralBonus),
            "refund_payable_clearing" => Ok(Self::RefundPayableClearing),
            "refund_payable_reversal" => Ok(Self::RefundPayableReversal),
            "refund_settlement" => Ok(Self::RefundSettlement),
            "refund_settlement_reversal" => Ok(Self::RefundSettlementReversal),
            "refund_unreconciled_clearing" => Ok(Self::RefundUnreconciledClearing),
            "refund_unreconciled_reversal" => Ok(Self::RefundUnreconciledReversal),
            "reserve" => Ok(Self::Reserve),
            "resolution_center_refund" => Ok(Self::ResolutionCenterRefund),
            "revshare_percentage_fee" => Ok(Self::RevsharePercentageFee),
            "sales_tax_collected" => Ok(Self::SalesTaxCollected),
            "sales_tax_collected_reversal" => Ok(Self::SalesTaxCollectedReversal),
            "sales_tax_fee" => Ok(Self::SalesTaxFee),
            "sales_tax_remittance" => Ok(Self::SalesTaxRemittance),
            "sales_tax_remittance_reversal" => Ok(Self::SalesTaxRemittanceReversal),
            "sales_tax_remitted" => Ok(Self::SalesTaxRemitted),
            "settlement_rounding_variance" => Ok(Self::SettlementRoundingVariance),
            "software_rental_revshare" => Ok(Self::SoftwareRentalRevshare),
            "software_rental_transaction" => Ok(Self::SoftwareRentalTransaction),
            "stripe_domestic_processing_fee" => Ok(Self::StripeDomesticProcessingFee),
            "stripe_international_processing_fee" => Ok(Self::StripeInternationalProcessingFee),
            "swap_fee" => Ok(Self::SwapFee),
            "tax_filing_fee" => Ok(Self::TaxFilingFee),
            "three_ds_fixed_fee" => Ok(Self::ThreeDsFixedFee),
            "topup" => Ok(Self::Topup),
            "topup_fee" => Ok(Self::TopupFee),
            "topup_reversal" => Ok(Self::TopupReversal),
            "treasury_fee" => Ok(Self::TreasuryFee),
            "treasury_payin" => Ok(Self::TreasuryPayin),
            "treasury_payin_receivable" => Ok(Self::TreasuryPayinReceivable),
            "whop_processing_fee" => Ok(Self::WhopProcessingFee),
            "whop_swap_fee_received" => Ok(Self::WhopSwapFeeReceived),
            "withdrawal" => Ok(Self::Withdrawal),
            "withdrawal_clawback" => Ok(Self::WithdrawalClawback),
            "withdrawal_clawback_reversal" => Ok(Self::WithdrawalClawbackReversal),
            "withdrawal_fee" => Ok(Self::WithdrawalFee),
            "withdrawal_fee_reversal" => Ok(Self::WithdrawalFeeReversal),
            "withdrawal_markup_fee" => Ok(Self::WithdrawalMarkupFee),
            "withdrawal_markup_fee_payout" => Ok(Self::WithdrawalMarkupFeePayout),
            "withdrawal_markup_fee_payout_reversal" => Ok(Self::WithdrawalMarkupFeePayoutReversal),
            "withdrawal_markup_fee_reversal" => Ok(Self::WithdrawalMarkupFeeReversal),
            "withdrawal_payable" => Ok(Self::WithdrawalPayable),
            "withdrawal_payable_clearing" => Ok(Self::WithdrawalPayableClearing),
            "withdrawal_payable_clearing_reversal" => Ok(Self::WithdrawalPayableClearingReversal),
            "withdrawal_payable_reversal" => Ok(Self::WithdrawalPayableReversal),
            "withdrawal_reclassification" => Ok(Self::WithdrawalReclassification),
            "withdrawal_reversal" => Ok(Self::WithdrawalReversal),
            "withdrawal_topup_adjustment" => Ok(Self::WithdrawalTopupAdjustment),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for GetFinancialReportResponseRowsItemLineCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AcceleratedSettlementFee => write!(f, "accelerated_settlement_fee"),
            Self::AccountSettlement => write!(f, "account_settlement"),
            Self::AdBalanceFundingReceipt => write!(f, "ad_balance_funding_receipt"),
            Self::AdBudgetRelease => write!(f, "ad_budget_release"),
            Self::AdCampaignBudget => write!(f, "ad_campaign_budget"),
            Self::AdFundingDisbursement => write!(f, "ad_funding_disbursement"),
            Self::AdIncomeExpense => write!(f, "ad_income_expense"),
            Self::AdIncomeReceipt => write!(f, "ad_income_receipt"),
            Self::AdNetworkCost => write!(f, "ad_network_cost"),
            Self::AdNetworkSettlement => write!(f, "ad_network_settlement"),
            Self::AdPublisherPayout => write!(f, "ad_publisher_payout"),
            Self::AdPublisherPayoutReceived => write!(f, "ad_publisher_payout_received"),
            Self::AdSpendCharge => write!(f, "ad_spend_charge"),
            Self::AdSpendMargin => write!(f, "ad_spend_margin"),
            Self::AdsCardSpread => write!(f, "ads_card_spread"),
            Self::AffiliateFee => write!(f, "affiliate_fee"),
            Self::AggregatedFee => write!(f, "aggregated_fee"),
            Self::Airdrop => write!(f, "airdrop"),
            Self::AirdropExpense => write!(f, "airdrop_expense"),
            Self::AirdropExpenseReversal => write!(f, "airdrop_expense_reversal"),
            Self::AirdropLinkCanceled => write!(f, "airdrop_link_canceled"),
            Self::AirdropLinkClaimed => write!(f, "airdrop_link_claimed"),
            Self::AirdropLinkCreated => write!(f, "airdrop_link_created"),
            Self::AirdropLinkFunded => write!(f, "airdrop_link_funded"),
            Self::AirdropLinkRedeemed => write!(f, "airdrop_link_redeemed"),
            Self::AirdropLinkReturned => write!(f, "airdrop_link_returned"),
            Self::AirdropReversal => write!(f, "airdrop_reversal"),
            Self::ApplicationFee => write!(f, "application_fee"),
            Self::ApplicationFeePayable => write!(f, "application_fee_payable"),
            Self::ApplicationFeePayout => write!(f, "application_fee_payout"),
            Self::Available => write!(f, "available"),
            Self::BadDebtExpense => write!(f, "bad_debt_expense"),
            Self::BadDebtOffset => write!(f, "bad_debt_offset"),
            Self::BalanceReservation => write!(f, "balance_reservation"),
            Self::BalanceReservationHold => write!(f, "balance_reservation_hold"),
            Self::BalanceReservationRelease => write!(f, "balance_reservation_release"),
            Self::BalanceReservationReversal => write!(f, "balance_reservation_reversal"),
            Self::BillingPercentageFee => write!(f, "billing_percentage_fee"),
            Self::BuyerFee => write!(f, "buyer_fee"),
            Self::CardInterchange => write!(f, "card_interchange"),
            Self::CardInterchangeReceivable => write!(f, "card_interchange_receivable"),
            Self::CardLoadDeposit => write!(f, "card_load_deposit"),
            Self::CardLoadTransfer => write!(f, "card_load_transfer"),
            Self::CardSpendAuthorization => write!(f, "card_spend_authorization"),
            Self::CardSpendAuthorizationHold => write!(f, "card_spend_authorization_hold"),
            Self::CardSpendAuthorizationVoid => write!(f, "card_spend_authorization_void"),
            Self::CardSpendAuthorizationVoidRelease => {
                write!(f, "card_spend_authorization_void_release")
            }
            Self::CardSpendCapture => write!(f, "card_spend_capture"),
            Self::CardSpendCaptureOffset => write!(f, "card_spend_capture_offset"),
            Self::CardSpendRefund => write!(f, "card_spend_refund"),
            Self::CardSpendRefundOffset => write!(f, "card_spend_refund_offset"),
            Self::CardUnloadDeposit => write!(f, "card_unload_deposit"),
            Self::CardUnloadTransfer => write!(f, "card_unload_transfer"),
            Self::ClawbackFee => write!(f, "clawback_fee"),
            Self::ClawbackReceivable => write!(f, "clawback_receivable"),
            Self::ClawbackReceivableReversal => write!(f, "clawback_receivable_reversal"),
            Self::ClawbackReceivableSettlement => write!(f, "clawback_receivable_settlement"),
            Self::ClawbackReceivableSettlementReversal => {
                write!(f, "clawback_receivable_settlement_reversal")
            }
            Self::CompanyReferral => write!(f, "company_referral"),
            Self::ConnectedAccountClawback => write!(f, "connected_account_clawback"),
            Self::ConnectedAccountNegativeBalance => {
                write!(f, "connected_account_negative_balance")
            }
            Self::CrossBorderPercentageFee => write!(f, "cross_border_percentage_fee"),
            Self::Crypto => write!(f, "crypto"),
            Self::CurrencyConversionIncoming => write!(f, "currency_conversion_incoming"),
            Self::CurrencyConversionOutgoing => write!(f, "currency_conversion_outgoing"),
            Self::DisputeAlertFee => write!(f, "dispute_alert_fee"),
            Self::DisputeHold => write!(f, "dispute_hold"),
            Self::DisputeHoldAdjustment => write!(f, "dispute_hold_adjustment"),
            Self::DisputeHoldBlocked => write!(f, "dispute_hold_blocked"),
            Self::DisputeManagementFee => write!(f, "dispute_management_fee"),
            Self::DisputePayableClearing => write!(f, "dispute_payable_clearing"),
            Self::DisputePayableReversal => write!(f, "dispute_payable_reversal"),
            Self::DisputeRepresentmentFee => write!(f, "dispute_representment_fee"),
            Self::DisputeSettlement => write!(f, "dispute_settlement"),
            Self::DisputeSettlementReversal => write!(f, "dispute_settlement_reversal"),
            Self::DisputeUnreconciledClearing => write!(f, "dispute_unreconciled_clearing"),
            Self::DisputeUnreconciledReversal => write!(f, "dispute_unreconciled_reversal"),
            Self::ExternalAccountConnectionFee => write!(f, "external_account_connection_fee"),
            Self::ExternalCardLoadDeposit => write!(f, "external_card_load_deposit"),
            Self::ExternalCardLoadOffset => write!(f, "external_card_load_offset"),
            Self::FraudPreventionFee => write!(f, "fraud_prevention_fee"),
            Self::FxGainLoss => write!(f, "fx_gain_loss"),
            Self::FxMarkup => write!(f, "fx_markup"),
            Self::FxPercentageFee => write!(f, "fx_percentage_fee"),
            Self::FxSettlementGainLoss => write!(f, "fx_settlement_gain_loss"),
            Self::HighRiskMerchantFee => write!(f, "high_risk_merchant_fee"),
            Self::IdentityVerificationFee => write!(f, "identity_verification_fee"),
            Self::InstallmentDefault => write!(f, "installment_default"),
            Self::InternalBalanceTransferIncoming => {
                write!(f, "internal_balance_transfer_incoming")
            }
            Self::InternalBalanceTransferOutgoing => {
                write!(f, "internal_balance_transfer_outgoing")
            }
            Self::InternalWithdrawal => write!(f, "internal_withdrawal"),
            Self::InternalWithdrawalComplete => write!(f, "internal_withdrawal_complete"),
            Self::InternalWithdrawalFee => write!(f, "internal_withdrawal_fee"),
            Self::InternalWithdrawalFeeReversal => write!(f, "internal_withdrawal_fee_reversal"),
            Self::InternalWithdrawalInTransit => write!(f, "internal_withdrawal_in_transit"),
            Self::InternalWithdrawalInTransitReversal => {
                write!(f, "internal_withdrawal_in_transit_reversal")
            }
            Self::InternalWithdrawalMarkupFee => write!(f, "internal_withdrawal_markup_fee"),
            Self::InternalWithdrawalMarkupFeePayout => {
                write!(f, "internal_withdrawal_markup_fee_payout")
            }
            Self::InternalWithdrawalMarkupFeePayoutReversal => {
                write!(f, "internal_withdrawal_markup_fee_payout_reversal")
            }
            Self::InternalWithdrawalMarkupFeeReversal => {
                write!(f, "internal_withdrawal_markup_fee_reversal")
            }
            Self::InternalWithdrawalPayable => write!(f, "internal_withdrawal_payable"),
            Self::InternalWithdrawalPayableReversal => {
                write!(f, "internal_withdrawal_payable_reversal")
            }
            Self::InternalWithdrawalReversal => write!(f, "internal_withdrawal_reversal"),
            Self::LegacyCryptoPayment => write!(f, "legacy_crypto_payment"),
            Self::LegacyPayment => write!(f, "legacy_payment"),
            Self::LegacyPaymentRefund => write!(f, "legacy_payment_refund"),
            Self::LicenseSale => write!(f, "license_sale"),
            Self::LicenseSaleCommission => write!(f, "license_sale_commission"),
            Self::LicenseSaleRevenue => write!(f, "license_sale_revenue"),
            Self::MarketplaceAffiliateFee => write!(f, "marketplace_affiliate_fee"),
            Self::MiscPurchase => write!(f, "misc_purchase"),
            Self::MiscRefund => write!(f, "misc_refund"),
            Self::MiscReversal => write!(f, "misc_reversal"),
            Self::OnchainDeposit => write!(f, "onchain_deposit"),
            Self::OnchainDepositOffset => write!(f, "onchain_deposit_offset"),
            Self::OnchainSwapOffset => write!(f, "onchain_swap_offset"),
            Self::OnchainSwapSource => write!(f, "onchain_swap_source"),
            Self::OnchainSwapTarget => write!(f, "onchain_swap_target"),
            Self::OnchainWalletTransferIncoming => write!(f, "onchain_wallet_transfer_incoming"),
            Self::OnchainWalletTransferOutgoing => write!(f, "onchain_wallet_transfer_outgoing"),
            Self::OnchainWithdrawal => write!(f, "onchain_withdrawal"),
            Self::OnchainWithdrawalOffset => write!(f, "onchain_withdrawal_offset"),
            Self::OrchestrationPercentageFee => write!(f, "orchestration_percentage_fee"),
            Self::PassthroughGmv => write!(f, "passthrough_gmv"),
            Self::PassthroughGmvOffset => write!(f, "passthrough_gmv_offset"),
            Self::PaymentDispute => write!(f, "payment_dispute"),
            Self::PaymentDisputeAdjustment => write!(f, "payment_dispute_adjustment"),
            Self::PaymentDisputeFee => write!(f, "payment_dispute_fee"),
            Self::PaymentDisputeReversal => write!(f, "payment_dispute_reversal"),
            Self::PaymentGross => write!(f, "payment_gross"),
            Self::PaymentGrossReversal => write!(f, "payment_gross_reversal"),
            Self::PaymentProcessingFixedFee => write!(f, "payment_processing_fixed_fee"),
            Self::PaymentProcessingPercentageFee => write!(f, "payment_processing_percentage_fee"),
            Self::PaymentReceivableClearing => write!(f, "payment_receivable_clearing"),
            Self::PaymentReceivableReversal => write!(f, "payment_receivable_reversal"),
            Self::PaymentReceivableSettlement => write!(f, "payment_receivable_settlement"),
            Self::PaymentReferral => write!(f, "payment_referral"),
            Self::PaymentReferralPayable => write!(f, "payment_referral_payable"),
            Self::PaymentReferralRefund => write!(f, "payment_referral_refund"),
            Self::PaymentReferralReversal => write!(f, "payment_referral_reversal"),
            Self::PaymentRefund => write!(f, "payment_refund"),
            Self::PaymentRefundFee => write!(f, "payment_refund_fee"),
            Self::PaymentRefundReversal => write!(f, "payment_refund_reversal"),
            Self::PaymentRevshare => write!(f, "payment_revshare"),
            Self::PaymentRevsharePayout => write!(f, "payment_revshare_payout"),
            Self::PaymentRevshareRefund => write!(f, "payment_revshare_refund"),
            Self::PaymentRevshareReversal => write!(f, "payment_revshare_reversal"),
            Self::PaymentUnreconciledClearing => write!(f, "payment_unreconciled_clearing"),
            Self::PaymentUnreconciledReversal => write!(f, "payment_unreconciled_reversal"),
            Self::PayoutFee => write!(f, "payout_fee"),
            Self::PayoutReceivable => write!(f, "payout_receivable"),
            Self::PayoutSubsidy => write!(f, "payout_subsidy"),
            Self::PayoutUnreconciledClearing => write!(f, "payout_unreconciled_clearing"),
            Self::Pending => write!(f, "pending"),
            Self::PlatformAffiliatePayment => write!(f, "platform_affiliate_payment"),
            Self::PlatformAffiliatePaymentReversal => {
                write!(f, "platform_affiliate_payment_reversal")
            }
            Self::PlatformBalancePayment => write!(f, "platform_balance_payment"),
            Self::PlatformBalancePaymentRefund => write!(f, "platform_balance_payment_refund"),
            Self::PlatformBalanceTransferFee => write!(f, "platform_balance_transfer_fee"),
            Self::PlatformBalanceTransferIncoming => {
                write!(f, "platform_balance_transfer_incoming")
            }
            Self::PlatformBalanceTransferOutgoing => {
                write!(f, "platform_balance_transfer_outgoing")
            }
            Self::PlatformCoveredDispute => write!(f, "platform_covered_dispute"),
            Self::PlatformEarning => write!(f, "platform_earning"),
            Self::PlatformEarningSettlement => write!(f, "platform_earning_settlement"),
            Self::PlatformEarningUnreconciledClearing => {
                write!(f, "platform_earning_unreconciled_clearing")
            }
            Self::PromoReversal => write!(f, "promo_reversal"),
            Self::PspAcceleratedSettlementFee => write!(f, "psp_accelerated_settlement_fee"),
            Self::PspAccountUpdaterFee => write!(f, "psp_account_updater_fee"),
            Self::PspAdjustedProcessingFee => write!(f, "psp_adjusted_processing_fee"),
            Self::PspAuthenticationFee => write!(f, "psp_authentication_fee"),
            Self::PspBankPullClearing => write!(f, "psp_bank_pull_clearing"),
            Self::PspBankPullSettlement => write!(f, "psp_bank_pull_settlement"),
            Self::PspBillingFee => write!(f, "psp_billing_fee"),
            Self::PspClawbackFee => write!(f, "psp_clawback_fee"),
            Self::PspClawbackSettlement => write!(f, "psp_clawback_settlement"),
            Self::PspClawbackSettlementReversal => write!(f, "psp_clawback_settlement_reversal"),
            Self::PspCollectionSettlement => write!(f, "psp_collection_settlement"),
            Self::PspCommissionFee => write!(f, "psp_commission_fee"),
            Self::PspConnectFee => write!(f, "psp_connect_fee"),
            Self::PspCorrection => write!(f, "psp_correction"),
            Self::PspCurrencyConversionIncoming => write!(f, "psp_currency_conversion_incoming"),
            Self::PspCurrencyConversionOutgoing => write!(f, "psp_currency_conversion_outgoing"),
            Self::PspCurrencyConversionReceivable => {
                write!(f, "psp_currency_conversion_receivable")
            }
            Self::PspCurrencyConversionReceivableClearing => {
                write!(f, "psp_currency_conversion_receivable_clearing")
            }
            Self::PspDisputeAlertFee => write!(f, "psp_dispute_alert_fee"),
            Self::PspDisputeFee => write!(f, "psp_dispute_fee"),
            Self::PspDisputeManagementFee => write!(f, "psp_dispute_management_fee"),
            Self::PspDisputePayable => write!(f, "psp_dispute_payable"),
            Self::PspExternalAccountConnectionFee => {
                write!(f, "psp_external_account_connection_fee")
            }
            Self::PspFixedFee => write!(f, "psp_fixed_fee"),
            Self::PspGatewayFee => write!(f, "psp_gateway_fee"),
            Self::PspIdentityVerificationFee => write!(f, "psp_identity_verification_fee"),
            Self::PspInterchangeFee => write!(f, "psp_interchange_fee"),
            Self::PspInvoiceTaxFee => write!(f, "psp_invoice_tax_fee"),
            Self::PspInvoicingFee => write!(f, "psp_invoicing_fee"),
            Self::PspMarkupFee => write!(f, "psp_markup_fee"),
            Self::PspNetworkTokenFee => write!(f, "psp_network_token_fee"),
            Self::PspOptimizationFee => write!(f, "psp_optimization_fee"),
            Self::PspPayinClearing => write!(f, "psp_payin_clearing"),
            Self::PspPayinSettlement => write!(f, "psp_payin_settlement"),
            Self::PspPaymentMethodFee => write!(f, "psp_payment_method_fee"),
            Self::PspPaymentReceivable => write!(f, "psp_payment_receivable"),
            Self::PspPayoutConsolidation => write!(f, "psp_payout_consolidation"),
            Self::PspPayoutDeposit => write!(f, "psp_payout_deposit"),
            Self::PspPayoutFee => write!(f, "psp_payout_fee"),
            Self::PspPayoutReceivable => write!(f, "psp_payout_receivable"),
            Self::PspPayoutReceivableClearing => write!(f, "psp_payout_receivable_clearing"),
            Self::PspPayoutSettlement => write!(f, "psp_payout_settlement"),
            Self::PspPayoutSettlementReversal => write!(f, "psp_payout_settlement_reversal"),
            Self::PspPoolClearing => write!(f, "psp_pool_clearing"),
            Self::PspPoolDisputeSettlement => write!(f, "psp_pool_dispute_settlement"),
            Self::PspPoolSettlement => write!(f, "psp_pool_settlement"),
            Self::PspProcessingFee => write!(f, "psp_processing_fee"),
            Self::PspReceivablePooled => write!(f, "psp_receivable_pooled"),
            Self::PspRecipientWalletLoad => write!(f, "psp_recipient_wallet_load"),
            Self::PspRefundFee => write!(f, "psp_refund_fee"),
            Self::PspRefundPayable => write!(f, "psp_refund_payable"),
            Self::PspReserveHold => write!(f, "psp_reserve_hold"),
            Self::PspReserveRelease => write!(f, "psp_reserve_release"),
            Self::PspRiskFee => write!(f, "psp_risk_fee"),
            Self::PspSchemeFee => write!(f, "psp_scheme_fee"),
            Self::PspServiceFee => write!(f, "psp_service_fee"),
            Self::PspTaxFilingFee => write!(f, "psp_tax_filing_fee"),
            Self::PspTaxServiceFee => write!(f, "psp_tax_service_fee"),
            Self::PspTerminalFee => write!(f, "psp_terminal_fee"),
            Self::PspTransferSettlement => write!(f, "psp_transfer_settlement"),
            Self::PspTransferSettlementReversal => write!(f, "psp_transfer_settlement_reversal"),
            Self::PspTreasuryFee => write!(f, "psp_treasury_fee"),
            Self::PspVariableFee => write!(f, "psp_variable_fee"),
            Self::PspWithholdingTax => write!(f, "psp_withholding_tax"),
            Self::RecipientWalletLoad => write!(f, "recipient_wallet_load"),
            Self::ReferralBonus => write!(f, "referral_bonus"),
            Self::RefundPayableClearing => write!(f, "refund_payable_clearing"),
            Self::RefundPayableReversal => write!(f, "refund_payable_reversal"),
            Self::RefundSettlement => write!(f, "refund_settlement"),
            Self::RefundSettlementReversal => write!(f, "refund_settlement_reversal"),
            Self::RefundUnreconciledClearing => write!(f, "refund_unreconciled_clearing"),
            Self::RefundUnreconciledReversal => write!(f, "refund_unreconciled_reversal"),
            Self::Reserve => write!(f, "reserve"),
            Self::ResolutionCenterRefund => write!(f, "resolution_center_refund"),
            Self::RevsharePercentageFee => write!(f, "revshare_percentage_fee"),
            Self::SalesTaxCollected => write!(f, "sales_tax_collected"),
            Self::SalesTaxCollectedReversal => write!(f, "sales_tax_collected_reversal"),
            Self::SalesTaxFee => write!(f, "sales_tax_fee"),
            Self::SalesTaxRemittance => write!(f, "sales_tax_remittance"),
            Self::SalesTaxRemittanceReversal => write!(f, "sales_tax_remittance_reversal"),
            Self::SalesTaxRemitted => write!(f, "sales_tax_remitted"),
            Self::SettlementRoundingVariance => write!(f, "settlement_rounding_variance"),
            Self::SoftwareRentalRevshare => write!(f, "software_rental_revshare"),
            Self::SoftwareRentalTransaction => write!(f, "software_rental_transaction"),
            Self::StripeDomesticProcessingFee => write!(f, "stripe_domestic_processing_fee"),
            Self::StripeInternationalProcessingFee => {
                write!(f, "stripe_international_processing_fee")
            }
            Self::SwapFee => write!(f, "swap_fee"),
            Self::TaxFilingFee => write!(f, "tax_filing_fee"),
            Self::ThreeDsFixedFee => write!(f, "three_ds_fixed_fee"),
            Self::Topup => write!(f, "topup"),
            Self::TopupFee => write!(f, "topup_fee"),
            Self::TopupReversal => write!(f, "topup_reversal"),
            Self::TreasuryFee => write!(f, "treasury_fee"),
            Self::TreasuryPayin => write!(f, "treasury_payin"),
            Self::TreasuryPayinReceivable => write!(f, "treasury_payin_receivable"),
            Self::WhopProcessingFee => write!(f, "whop_processing_fee"),
            Self::WhopSwapFeeReceived => write!(f, "whop_swap_fee_received"),
            Self::Withdrawal => write!(f, "withdrawal"),
            Self::WithdrawalClawback => write!(f, "withdrawal_clawback"),
            Self::WithdrawalClawbackReversal => write!(f, "withdrawal_clawback_reversal"),
            Self::WithdrawalFee => write!(f, "withdrawal_fee"),
            Self::WithdrawalFeeReversal => write!(f, "withdrawal_fee_reversal"),
            Self::WithdrawalMarkupFee => write!(f, "withdrawal_markup_fee"),
            Self::WithdrawalMarkupFeePayout => write!(f, "withdrawal_markup_fee_payout"),
            Self::WithdrawalMarkupFeePayoutReversal => {
                write!(f, "withdrawal_markup_fee_payout_reversal")
            }
            Self::WithdrawalMarkupFeeReversal => write!(f, "withdrawal_markup_fee_reversal"),
            Self::WithdrawalPayable => write!(f, "withdrawal_payable"),
            Self::WithdrawalPayableClearing => write!(f, "withdrawal_payable_clearing"),
            Self::WithdrawalPayableClearingReversal => {
                write!(f, "withdrawal_payable_clearing_reversal")
            }
            Self::WithdrawalPayableReversal => write!(f, "withdrawal_payable_reversal"),
            Self::WithdrawalReclassification => write!(f, "withdrawal_reclassification"),
            Self::WithdrawalReversal => write!(f, "withdrawal_reversal"),
            Self::WithdrawalTopupAdjustment => write!(f, "withdrawal_topup_adjustment"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
