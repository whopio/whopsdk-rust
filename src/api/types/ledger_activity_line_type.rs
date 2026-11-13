pub use crate::prelude::*;

/// The ledger line category this activity was posted under.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LedgerActivityLineType {
    PspPaymentReceivable,
    PaymentGross,
    Topup,
    TopupFee,
    PaymentGrossReversal,
    TopupReversal,
    PaymentRefund,
    PaymentRefundReversal,
    PlatformBalancePayment,
    PlatformBalancePaymentRefund,
    AdSpendPurchase,
    PspRefundPayable,
    PassthroughGmv,
    PassthroughGmvOffset,
    InstallmentDefault,
    AggregatedFee,
    PaymentProcessingFixedFee,
    PaymentProcessingPercentageFee,
    BillingPercentageFee,
    CrossBorderPercentageFee,
    FxPercentageFee,
    OrchestrationPercentageFee,
    RevsharePercentageFee,
    StripeDomesticProcessingFee,
    StripeInternationalProcessingFee,
    ThreeDsFixedFee,
    WhopProcessingFee,
    EconomicIntelligencePercentageFee,
    HighRiskMerchantFee,
    FraudPreventionFee,
    BuyerFee,
    SalesTaxFee,
    PayoutFee,
    AffiliateFee,
    MarketplaceAffiliateFee,
    ApplicationFee,
    PlatformMarkupFee,
    PlatformMarkupFeePayout,
    SalesTaxRemittance,
    SalesTaxRemittanceReversal,
    SalesTaxCollected,
    SalesTaxCollectedReversal,
    SalesTaxRemitted,
    PaymentDispute,
    PaymentDisputeAdjustment,
    PaymentDisputeFee,
    PaymentDisputeReversal,
    PlatformCoveredDispute,
    DisputeAlertFee,
    DisputeRepresentmentFee,
    PaymentReturnFee,
    DisputeHoldAdjustment,
    DisputeHoldBlocked,
    PspDisputePayable,
    Withdrawal,
    WithdrawalPayable,
    WithdrawalPayableReversal,
    WithdrawalFee,
    WithdrawalFeeReversal,
    WithdrawalMarkupFee,
    WithdrawalMarkupFeeReversal,
    WithdrawalMarkupFeePayout,
    WithdrawalMarkupFeePayoutReversal,
    WithdrawalReversal,
    WithdrawalClawback,
    WithdrawalClawbackReversal,
    PayoutReceivable,
    WithdrawalReclassification,
    ClawbackReceivable,
    ClawbackReceivableReversal,
    CapitalWithholding,
    CapitalWithholdingReversal,
    CapitalHold,
    CapitalHoldRelease,
    CapitalRemittance,
    CapitalRemittanceReversal,
    TreasuryPayin,
    TreasuryPayinReceivable,
    OnchainDeposit,
    OnchainDepositOffset,
    OnchainWithdrawal,
    OnchainWithdrawalOffset,
    TradingAccountWithdrawal,
    TradingAccountWithdrawalOffset,
    TradingAccountDeposit,
    TradingAccountDepositOffset,
    PerpsLoadTransfer,
    PerpsLoadDeposit,
    PerpsUnloadTransfer,
    PerpsUnloadDeposit,
    PerpsBridgeFee,
    PerpsBridgeFeeOffset,
    PerpsVolume,
    PerpsVolumeOffset,
    PerpsFee,
    PerpsFeeOffset,
    PlatformTradingCommission,
    TradingFee,
    PerpsRealizedPnl,
    PerpsRealizedPnlOffset,
    OnchainWalletTransferOutgoing,
    OnchainWalletTransferIncoming,
    OnchainPlatformTransferOutgoing,
    OnchainPlatformTransferIncoming,
    OnchainInternalTransferOutgoing,
    OnchainInternalTransferIncoming,
    OnchainSwapSource,
    OnchainSwapTarget,
    OnchainSwapOffset,
    WhopSwapFeeReceived,
    SwapFee,
    InternalWithdrawal,
    InternalWithdrawalReversal,
    InternalWithdrawalPayable,
    InternalWithdrawalPayableReversal,
    InternalWithdrawalFee,
    InternalWithdrawalFeeReversal,
    InternalWithdrawalMarkupFee,
    InternalWithdrawalMarkupFeeReversal,
    InternalWithdrawalMarkupFeePayout,
    InternalWithdrawalMarkupFeePayoutReversal,
    InternalWithdrawalInTransit,
    InternalWithdrawalComplete,
    InternalWithdrawalInTransitReversal,
    CardLoadTransfer,
    CardLoadDeposit,
    CardUnloadTransfer,
    CardUnloadDeposit,
    ExternalCardLoadDeposit,
    ExternalCardLoadOffset,
    CardSpendAuthorization,
    CardSpendAuthorizationHold,
    CardSpendCapture,
    CardSpendCaptureOffset,
    CardSpendAuthorizationVoid,
    CardSpendAuthorizationVoidRelease,
    CardSpendRefund,
    CardSpendRefundOffset,
    Cashback,
    CashbackExpense,
    CashbackFunding,
    CardInterchangeReceivable,
    CardInterchange,
    AccountSettlement,
    CurrencyConversionOutgoing,
    CurrencyConversionIncoming,
    BalanceReservation,
    BalanceReservationHold,
    BalanceReservationReversal,
    BalanceReservationRelease,
    BadDebtOffset,
    BadDebtExpense,
    PlatformCreditsPayment,
    PlatformCreditsPaymentRefund,
    PlatformCreditsGranted,
    LevelsPointsCard,
    LevelsPointsAds,
    LevelsPointsTrading,
    LevelsPointsPartner,
    LevelsPointsBurned,
    LevelsPointsIssued,
    LevelsRebate,
    MarketingExpense,
    PlatformBalanceTransferOutgoing,
    PlatformBalanceTransferIncoming,
    InternalBalanceTransferOutgoing,
    InternalBalanceTransferIncoming,
    PlatformBalanceTransferFee,
    ConnectedAccountClawback,
    ConnectedAccountNegativeBalance,
    AdPublisherPayout,
    AdPublisherPayoutReceived,
    CompanyReferral,
    PlatformAffiliatePayment,
    PlatformAffiliatePaymentReversal,
    OnboardingReward,
    PartnerGrantReward,
    PaymentRevshare,
    PaymentRevsharePayout,
    PaymentRevshareReversal,
    PaymentRevshareRefund,
    PaymentReferral,
    PaymentReferralPayable,
    PaymentReferralReversal,
    PaymentReferralRefund,
    ApplicationFeePayable,
    ApplicationFeePayout,
    PlatformEarning,
    Airdrop,
    AirdropReversal,
    AirdropExpense,
    AirdropExpenseReversal,
    AirdropLinkFunded,
    AirdropLinkCanceled,
    AirdropLinkReturned,
    AirdropLinkClaimed,
    AirdropLinkRedeemed,
    AirdropLinkCreated,
    ResolutionCenterRefund,
    AdFundingDisbursement,
    AdCampaignBudget,
    AdsCardSpread,
    AdSpendCharge,
    AdNetworkCost,
    AdSpendMargin,
    AdIncomeExpense,
    AdIncomeReceipt,
    AdBudgetRelease,
    AdNetworkSettlement,
    AdAffiliatePayout,
    AdAffiliatePayoutReceived,
    AdBalanceFundingReceipt,
    MiscReversal,
    FxGainLoss,
    FxMarkup,
    LegacyCryptoPayment,
    LegacyPayment,
    LegacyPaymentRefund,
    LicenseSale,
    LicenseSaleCommission,
    LicenseSaleRevenue,
    SoftwareRentalRevshare,
    SoftwareRentalTransaction,
    MiscPurchase,
    MiscRefund,
    ReferralBonus,
    PromoReversal,
    PaymentReceivableSettlement,
    PlatformEarningSettlement,
    PlatformEarningUnreconciledClearing,
    PaymentReceivableClearing,
    PaymentReceivableReversal,
    PaymentUnreconciledClearing,
    PaymentUnreconciledReversal,
    SettlementShortfallLoss,
    FxSettlementGainLoss,
    SettlementRoundingVariance,
    PspPayoutSettlement,
    PspPayoutDeposit,
    PspPayoutReceivable,
    PspPayoutReceivableClearing,
    PayoutUnreconciledClearing,
    PspCurrencyConversionOutgoing,
    PspCurrencyConversionIncoming,
    PspCurrencyConversionReceivable,
    PspCurrencyConversionReceivableClearing,
    PspFxAdjustment,
    RefundSettlement,
    RefundSettlementReversal,
    RefundPayableClearing,
    RefundPayableReversal,
    RefundUnreconciledClearing,
    RefundUnreconciledReversal,
    PspAccountDebit,
    PaymentRefundFee,
    DisputeSettlement,
    DisputeSettlementReversal,
    DisputePayableClearing,
    DisputePayableReversal,
    DisputeUnreconciledClearing,
    DisputeUnreconciledReversal,
    PspReserveHold,
    PspReserveRelease,
    PspDisputeFee,
    PspDisputeAlertFee,
    PspRefundFee,
    PspAccountUpdaterFee,
    PspAdjustedProcessingFee,
    PspAuthenticationFee,
    PspCommissionFee,
    PspFixedFee,
    PspGatewayFee,
    PspInterchangeFee,
    PspInvoiceTaxFee,
    PspMarkupFee,
    PspNetworkTokenFee,
    PspOptimizationFee,
    PspPaymentMethodFee,
    PspPayoutFee,
    PspProcessingFee,
    PspRiskFee,
    PspSchemeFee,
    PspVariableFee,
    PspBillingFee,
    PspTaxServiceFee,
    PspConnectFee,
    ExternalAccountConnectionFee,
    PspExternalAccountConnectionFee,
    PspInvoicingFee,
    PspTerminalFee,
    TreasuryFee,
    PspTreasuryFee,
    AcceleratedSettlementFee,
    PspAcceleratedSettlementFee,
    PspReceivablePooled,
    PspPoolSettlement,
    PspPoolClearing,
    PspPoolRefundSettlement,
    PspPoolRefundClearing,
    PspCorrection,
    PspPayinSettlement,
    PspPayinClearing,
    PspBankPullSettlement,
    PspBankPullClearing,
    PspPoolDisputeSettlement,
    PspPayoutConsolidation,
    PspWithholdingTax,
    PspPayoutSettlementReversal,
    PspTransferSettlement,
    PspTransferSettlementReversal,
    PspCollectionSettlement,
    PayoutSubsidy,
    WithdrawalTopupAdjustment,
    WithdrawalPayableClearing,
    WithdrawalPayableClearingReversal,
    IdentityVerificationFee,
    PspIdentityVerificationFee,
    TaxFilingFee,
    PspTaxFilingFee,
    PspServiceFee,
    RecipientWalletLoad,
    PspRecipientWalletLoad,
    DisputeManagementFee,
    PspDisputeManagementFee,
    PspClawbackSettlement,
    PspClawbackSettlementReversal,
    ClawbackReceivableSettlement,
    ClawbackReceivableSettlementReversal,
    ClawbackFee,
    PspClawbackFee,
    Deposit,
    WalletTransferIncoming,
    WalletTransferOutgoing,
    SwapSource,
    SwapTarget,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for LedgerActivityLineType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::PspPaymentReceivable => serializer.serialize_str("psp_payment_receivable"),
            Self::PaymentGross => serializer.serialize_str("payment_gross"),
            Self::Topup => serializer.serialize_str("topup"),
            Self::TopupFee => serializer.serialize_str("topup_fee"),
            Self::PaymentGrossReversal => serializer.serialize_str("payment_gross_reversal"),
            Self::TopupReversal => serializer.serialize_str("topup_reversal"),
            Self::PaymentRefund => serializer.serialize_str("payment_refund"),
            Self::PaymentRefundReversal => serializer.serialize_str("payment_refund_reversal"),
            Self::PlatformBalancePayment => serializer.serialize_str("platform_balance_payment"),
            Self::PlatformBalancePaymentRefund => {
                serializer.serialize_str("platform_balance_payment_refund")
            }
            Self::AdSpendPurchase => serializer.serialize_str("ad_spend_purchase"),
            Self::PspRefundPayable => serializer.serialize_str("psp_refund_payable"),
            Self::PassthroughGmv => serializer.serialize_str("passthrough_gmv"),
            Self::PassthroughGmvOffset => serializer.serialize_str("passthrough_gmv_offset"),
            Self::InstallmentDefault => serializer.serialize_str("installment_default"),
            Self::AggregatedFee => serializer.serialize_str("aggregated_fee"),
            Self::PaymentProcessingFixedFee => {
                serializer.serialize_str("payment_processing_fixed_fee")
            }
            Self::PaymentProcessingPercentageFee => {
                serializer.serialize_str("payment_processing_percentage_fee")
            }
            Self::BillingPercentageFee => serializer.serialize_str("billing_percentage_fee"),
            Self::CrossBorderPercentageFee => {
                serializer.serialize_str("cross_border_percentage_fee")
            }
            Self::FxPercentageFee => serializer.serialize_str("fx_percentage_fee"),
            Self::OrchestrationPercentageFee => {
                serializer.serialize_str("orchestration_percentage_fee")
            }
            Self::RevsharePercentageFee => serializer.serialize_str("revshare_percentage_fee"),
            Self::StripeDomesticProcessingFee => {
                serializer.serialize_str("stripe_domestic_processing_fee")
            }
            Self::StripeInternationalProcessingFee => {
                serializer.serialize_str("stripe_international_processing_fee")
            }
            Self::ThreeDsFixedFee => serializer.serialize_str("three_ds_fixed_fee"),
            Self::WhopProcessingFee => serializer.serialize_str("whop_processing_fee"),
            Self::EconomicIntelligencePercentageFee => {
                serializer.serialize_str("economic_intelligence_percentage_fee")
            }
            Self::HighRiskMerchantFee => serializer.serialize_str("high_risk_merchant_fee"),
            Self::FraudPreventionFee => serializer.serialize_str("fraud_prevention_fee"),
            Self::BuyerFee => serializer.serialize_str("buyer_fee"),
            Self::SalesTaxFee => serializer.serialize_str("sales_tax_fee"),
            Self::PayoutFee => serializer.serialize_str("payout_fee"),
            Self::AffiliateFee => serializer.serialize_str("affiliate_fee"),
            Self::MarketplaceAffiliateFee => serializer.serialize_str("marketplace_affiliate_fee"),
            Self::ApplicationFee => serializer.serialize_str("application_fee"),
            Self::PlatformMarkupFee => serializer.serialize_str("platform_markup_fee"),
            Self::PlatformMarkupFeePayout => serializer.serialize_str("platform_markup_fee_payout"),
            Self::SalesTaxRemittance => serializer.serialize_str("sales_tax_remittance"),
            Self::SalesTaxRemittanceReversal => {
                serializer.serialize_str("sales_tax_remittance_reversal")
            }
            Self::SalesTaxCollected => serializer.serialize_str("sales_tax_collected"),
            Self::SalesTaxCollectedReversal => {
                serializer.serialize_str("sales_tax_collected_reversal")
            }
            Self::SalesTaxRemitted => serializer.serialize_str("sales_tax_remitted"),
            Self::PaymentDispute => serializer.serialize_str("payment_dispute"),
            Self::PaymentDisputeAdjustment => {
                serializer.serialize_str("payment_dispute_adjustment")
            }
            Self::PaymentDisputeFee => serializer.serialize_str("payment_dispute_fee"),
            Self::PaymentDisputeReversal => serializer.serialize_str("payment_dispute_reversal"),
            Self::PlatformCoveredDispute => serializer.serialize_str("platform_covered_dispute"),
            Self::DisputeAlertFee => serializer.serialize_str("dispute_alert_fee"),
            Self::DisputeRepresentmentFee => serializer.serialize_str("dispute_representment_fee"),
            Self::PaymentReturnFee => serializer.serialize_str("payment_return_fee"),
            Self::DisputeHoldAdjustment => serializer.serialize_str("dispute_hold_adjustment"),
            Self::DisputeHoldBlocked => serializer.serialize_str("dispute_hold_blocked"),
            Self::PspDisputePayable => serializer.serialize_str("psp_dispute_payable"),
            Self::Withdrawal => serializer.serialize_str("withdrawal"),
            Self::WithdrawalPayable => serializer.serialize_str("withdrawal_payable"),
            Self::WithdrawalPayableReversal => {
                serializer.serialize_str("withdrawal_payable_reversal")
            }
            Self::WithdrawalFee => serializer.serialize_str("withdrawal_fee"),
            Self::WithdrawalFeeReversal => serializer.serialize_str("withdrawal_fee_reversal"),
            Self::WithdrawalMarkupFee => serializer.serialize_str("withdrawal_markup_fee"),
            Self::WithdrawalMarkupFeeReversal => {
                serializer.serialize_str("withdrawal_markup_fee_reversal")
            }
            Self::WithdrawalMarkupFeePayout => {
                serializer.serialize_str("withdrawal_markup_fee_payout")
            }
            Self::WithdrawalMarkupFeePayoutReversal => {
                serializer.serialize_str("withdrawal_markup_fee_payout_reversal")
            }
            Self::WithdrawalReversal => serializer.serialize_str("withdrawal_reversal"),
            Self::WithdrawalClawback => serializer.serialize_str("withdrawal_clawback"),
            Self::WithdrawalClawbackReversal => {
                serializer.serialize_str("withdrawal_clawback_reversal")
            }
            Self::PayoutReceivable => serializer.serialize_str("payout_receivable"),
            Self::WithdrawalReclassification => {
                serializer.serialize_str("withdrawal_reclassification")
            }
            Self::ClawbackReceivable => serializer.serialize_str("clawback_receivable"),
            Self::ClawbackReceivableReversal => {
                serializer.serialize_str("clawback_receivable_reversal")
            }
            Self::CapitalWithholding => serializer.serialize_str("capital_withholding"),
            Self::CapitalWithholdingReversal => {
                serializer.serialize_str("capital_withholding_reversal")
            }
            Self::CapitalHold => serializer.serialize_str("capital_hold"),
            Self::CapitalHoldRelease => serializer.serialize_str("capital_hold_release"),
            Self::CapitalRemittance => serializer.serialize_str("capital_remittance"),
            Self::CapitalRemittanceReversal => {
                serializer.serialize_str("capital_remittance_reversal")
            }
            Self::TreasuryPayin => serializer.serialize_str("treasury_payin"),
            Self::TreasuryPayinReceivable => serializer.serialize_str("treasury_payin_receivable"),
            Self::OnchainDeposit => serializer.serialize_str("onchain_deposit"),
            Self::OnchainDepositOffset => serializer.serialize_str("onchain_deposit_offset"),
            Self::OnchainWithdrawal => serializer.serialize_str("onchain_withdrawal"),
            Self::OnchainWithdrawalOffset => serializer.serialize_str("onchain_withdrawal_offset"),
            Self::TradingAccountWithdrawal => {
                serializer.serialize_str("trading_account_withdrawal")
            }
            Self::TradingAccountWithdrawalOffset => {
                serializer.serialize_str("trading_account_withdrawal_offset")
            }
            Self::TradingAccountDeposit => serializer.serialize_str("trading_account_deposit"),
            Self::TradingAccountDepositOffset => {
                serializer.serialize_str("trading_account_deposit_offset")
            }
            Self::PerpsLoadTransfer => serializer.serialize_str("perps_load_transfer"),
            Self::PerpsLoadDeposit => serializer.serialize_str("perps_load_deposit"),
            Self::PerpsUnloadTransfer => serializer.serialize_str("perps_unload_transfer"),
            Self::PerpsUnloadDeposit => serializer.serialize_str("perps_unload_deposit"),
            Self::PerpsBridgeFee => serializer.serialize_str("perps_bridge_fee"),
            Self::PerpsBridgeFeeOffset => serializer.serialize_str("perps_bridge_fee_offset"),
            Self::PerpsVolume => serializer.serialize_str("perps_volume"),
            Self::PerpsVolumeOffset => serializer.serialize_str("perps_volume_offset"),
            Self::PerpsFee => serializer.serialize_str("perps_fee"),
            Self::PerpsFeeOffset => serializer.serialize_str("perps_fee_offset"),
            Self::PlatformTradingCommission => {
                serializer.serialize_str("platform_trading_commission")
            }
            Self::TradingFee => serializer.serialize_str("trading_fee"),
            Self::PerpsRealizedPnl => serializer.serialize_str("perps_realized_pnl"),
            Self::PerpsRealizedPnlOffset => serializer.serialize_str("perps_realized_pnl_offset"),
            Self::OnchainWalletTransferOutgoing => {
                serializer.serialize_str("onchain_wallet_transfer_outgoing")
            }
            Self::OnchainWalletTransferIncoming => {
                serializer.serialize_str("onchain_wallet_transfer_incoming")
            }
            Self::OnchainPlatformTransferOutgoing => {
                serializer.serialize_str("onchain_platform_transfer_outgoing")
            }
            Self::OnchainPlatformTransferIncoming => {
                serializer.serialize_str("onchain_platform_transfer_incoming")
            }
            Self::OnchainInternalTransferOutgoing => {
                serializer.serialize_str("onchain_internal_transfer_outgoing")
            }
            Self::OnchainInternalTransferIncoming => {
                serializer.serialize_str("onchain_internal_transfer_incoming")
            }
            Self::OnchainSwapSource => serializer.serialize_str("onchain_swap_source"),
            Self::OnchainSwapTarget => serializer.serialize_str("onchain_swap_target"),
            Self::OnchainSwapOffset => serializer.serialize_str("onchain_swap_offset"),
            Self::WhopSwapFeeReceived => serializer.serialize_str("whop_swap_fee_received"),
            Self::SwapFee => serializer.serialize_str("swap_fee"),
            Self::InternalWithdrawal => serializer.serialize_str("internal_withdrawal"),
            Self::InternalWithdrawalReversal => {
                serializer.serialize_str("internal_withdrawal_reversal")
            }
            Self::InternalWithdrawalPayable => {
                serializer.serialize_str("internal_withdrawal_payable")
            }
            Self::InternalWithdrawalPayableReversal => {
                serializer.serialize_str("internal_withdrawal_payable_reversal")
            }
            Self::InternalWithdrawalFee => serializer.serialize_str("internal_withdrawal_fee"),
            Self::InternalWithdrawalFeeReversal => {
                serializer.serialize_str("internal_withdrawal_fee_reversal")
            }
            Self::InternalWithdrawalMarkupFee => {
                serializer.serialize_str("internal_withdrawal_markup_fee")
            }
            Self::InternalWithdrawalMarkupFeeReversal => {
                serializer.serialize_str("internal_withdrawal_markup_fee_reversal")
            }
            Self::InternalWithdrawalMarkupFeePayout => {
                serializer.serialize_str("internal_withdrawal_markup_fee_payout")
            }
            Self::InternalWithdrawalMarkupFeePayoutReversal => {
                serializer.serialize_str("internal_withdrawal_markup_fee_payout_reversal")
            }
            Self::InternalWithdrawalInTransit => {
                serializer.serialize_str("internal_withdrawal_in_transit")
            }
            Self::InternalWithdrawalComplete => {
                serializer.serialize_str("internal_withdrawal_complete")
            }
            Self::InternalWithdrawalInTransitReversal => {
                serializer.serialize_str("internal_withdrawal_in_transit_reversal")
            }
            Self::CardLoadTransfer => serializer.serialize_str("card_load_transfer"),
            Self::CardLoadDeposit => serializer.serialize_str("card_load_deposit"),
            Self::CardUnloadTransfer => serializer.serialize_str("card_unload_transfer"),
            Self::CardUnloadDeposit => serializer.serialize_str("card_unload_deposit"),
            Self::ExternalCardLoadDeposit => serializer.serialize_str("external_card_load_deposit"),
            Self::ExternalCardLoadOffset => serializer.serialize_str("external_card_load_offset"),
            Self::CardSpendAuthorization => serializer.serialize_str("card_spend_authorization"),
            Self::CardSpendAuthorizationHold => {
                serializer.serialize_str("card_spend_authorization_hold")
            }
            Self::CardSpendCapture => serializer.serialize_str("card_spend_capture"),
            Self::CardSpendCaptureOffset => serializer.serialize_str("card_spend_capture_offset"),
            Self::CardSpendAuthorizationVoid => {
                serializer.serialize_str("card_spend_authorization_void")
            }
            Self::CardSpendAuthorizationVoidRelease => {
                serializer.serialize_str("card_spend_authorization_void_release")
            }
            Self::CardSpendRefund => serializer.serialize_str("card_spend_refund"),
            Self::CardSpendRefundOffset => serializer.serialize_str("card_spend_refund_offset"),
            Self::Cashback => serializer.serialize_str("cashback"),
            Self::CashbackExpense => serializer.serialize_str("cashback_expense"),
            Self::CashbackFunding => serializer.serialize_str("cashback_funding"),
            Self::CardInterchangeReceivable => {
                serializer.serialize_str("card_interchange_receivable")
            }
            Self::CardInterchange => serializer.serialize_str("card_interchange"),
            Self::AccountSettlement => serializer.serialize_str("account_settlement"),
            Self::CurrencyConversionOutgoing => {
                serializer.serialize_str("currency_conversion_outgoing")
            }
            Self::CurrencyConversionIncoming => {
                serializer.serialize_str("currency_conversion_incoming")
            }
            Self::BalanceReservation => serializer.serialize_str("balance_reservation"),
            Self::BalanceReservationHold => serializer.serialize_str("balance_reservation_hold"),
            Self::BalanceReservationReversal => {
                serializer.serialize_str("balance_reservation_reversal")
            }
            Self::BalanceReservationRelease => {
                serializer.serialize_str("balance_reservation_release")
            }
            Self::BadDebtOffset => serializer.serialize_str("bad_debt_offset"),
            Self::BadDebtExpense => serializer.serialize_str("bad_debt_expense"),
            Self::PlatformCreditsPayment => serializer.serialize_str("platform_credits_payment"),
            Self::PlatformCreditsPaymentRefund => {
                serializer.serialize_str("platform_credits_payment_refund")
            }
            Self::PlatformCreditsGranted => serializer.serialize_str("platform_credits_granted"),
            Self::LevelsPointsCard => serializer.serialize_str("levels_points_card"),
            Self::LevelsPointsAds => serializer.serialize_str("levels_points_ads"),
            Self::LevelsPointsTrading => serializer.serialize_str("levels_points_trading"),
            Self::LevelsPointsPartner => serializer.serialize_str("levels_points_partner"),
            Self::LevelsPointsBurned => serializer.serialize_str("levels_points_burned"),
            Self::LevelsPointsIssued => serializer.serialize_str("levels_points_issued"),
            Self::LevelsRebate => serializer.serialize_str("levels_rebate"),
            Self::MarketingExpense => serializer.serialize_str("marketing_expense"),
            Self::PlatformBalanceTransferOutgoing => {
                serializer.serialize_str("platform_balance_transfer_outgoing")
            }
            Self::PlatformBalanceTransferIncoming => {
                serializer.serialize_str("platform_balance_transfer_incoming")
            }
            Self::InternalBalanceTransferOutgoing => {
                serializer.serialize_str("internal_balance_transfer_outgoing")
            }
            Self::InternalBalanceTransferIncoming => {
                serializer.serialize_str("internal_balance_transfer_incoming")
            }
            Self::PlatformBalanceTransferFee => {
                serializer.serialize_str("platform_balance_transfer_fee")
            }
            Self::ConnectedAccountClawback => {
                serializer.serialize_str("connected_account_clawback")
            }
            Self::ConnectedAccountNegativeBalance => {
                serializer.serialize_str("connected_account_negative_balance")
            }
            Self::AdPublisherPayout => serializer.serialize_str("ad_publisher_payout"),
            Self::AdPublisherPayoutReceived => {
                serializer.serialize_str("ad_publisher_payout_received")
            }
            Self::CompanyReferral => serializer.serialize_str("company_referral"),
            Self::PlatformAffiliatePayment => {
                serializer.serialize_str("platform_affiliate_payment")
            }
            Self::PlatformAffiliatePaymentReversal => {
                serializer.serialize_str("platform_affiliate_payment_reversal")
            }
            Self::OnboardingReward => serializer.serialize_str("onboarding_reward"),
            Self::PartnerGrantReward => serializer.serialize_str("partner_grant_reward"),
            Self::PaymentRevshare => serializer.serialize_str("payment_revshare"),
            Self::PaymentRevsharePayout => serializer.serialize_str("payment_revshare_payout"),
            Self::PaymentRevshareReversal => serializer.serialize_str("payment_revshare_reversal"),
            Self::PaymentRevshareRefund => serializer.serialize_str("payment_revshare_refund"),
            Self::PaymentReferral => serializer.serialize_str("payment_referral"),
            Self::PaymentReferralPayable => serializer.serialize_str("payment_referral_payable"),
            Self::PaymentReferralReversal => serializer.serialize_str("payment_referral_reversal"),
            Self::PaymentReferralRefund => serializer.serialize_str("payment_referral_refund"),
            Self::ApplicationFeePayable => serializer.serialize_str("application_fee_payable"),
            Self::ApplicationFeePayout => serializer.serialize_str("application_fee_payout"),
            Self::PlatformEarning => serializer.serialize_str("platform_earning"),
            Self::Airdrop => serializer.serialize_str("airdrop"),
            Self::AirdropReversal => serializer.serialize_str("airdrop_reversal"),
            Self::AirdropExpense => serializer.serialize_str("airdrop_expense"),
            Self::AirdropExpenseReversal => serializer.serialize_str("airdrop_expense_reversal"),
            Self::AirdropLinkFunded => serializer.serialize_str("airdrop_link_funded"),
            Self::AirdropLinkCanceled => serializer.serialize_str("airdrop_link_canceled"),
            Self::AirdropLinkReturned => serializer.serialize_str("airdrop_link_returned"),
            Self::AirdropLinkClaimed => serializer.serialize_str("airdrop_link_claimed"),
            Self::AirdropLinkRedeemed => serializer.serialize_str("airdrop_link_redeemed"),
            Self::AirdropLinkCreated => serializer.serialize_str("airdrop_link_created"),
            Self::ResolutionCenterRefund => serializer.serialize_str("resolution_center_refund"),
            Self::AdFundingDisbursement => serializer.serialize_str("ad_funding_disbursement"),
            Self::AdCampaignBudget => serializer.serialize_str("ad_campaign_budget"),
            Self::AdsCardSpread => serializer.serialize_str("ads_card_spread"),
            Self::AdSpendCharge => serializer.serialize_str("ad_spend_charge"),
            Self::AdNetworkCost => serializer.serialize_str("ad_network_cost"),
            Self::AdSpendMargin => serializer.serialize_str("ad_spend_margin"),
            Self::AdIncomeExpense => serializer.serialize_str("ad_income_expense"),
            Self::AdIncomeReceipt => serializer.serialize_str("ad_income_receipt"),
            Self::AdBudgetRelease => serializer.serialize_str("ad_budget_release"),
            Self::AdNetworkSettlement => serializer.serialize_str("ad_network_settlement"),
            Self::AdAffiliatePayout => serializer.serialize_str("ad_affiliate_payout"),
            Self::AdAffiliatePayoutReceived => {
                serializer.serialize_str("ad_affiliate_payout_received")
            }
            Self::AdBalanceFundingReceipt => serializer.serialize_str("ad_balance_funding_receipt"),
            Self::MiscReversal => serializer.serialize_str("misc_reversal"),
            Self::FxGainLoss => serializer.serialize_str("fx_gain_loss"),
            Self::FxMarkup => serializer.serialize_str("fx_markup"),
            Self::LegacyCryptoPayment => serializer.serialize_str("legacy_crypto_payment"),
            Self::LegacyPayment => serializer.serialize_str("legacy_payment"),
            Self::LegacyPaymentRefund => serializer.serialize_str("legacy_payment_refund"),
            Self::LicenseSale => serializer.serialize_str("license_sale"),
            Self::LicenseSaleCommission => serializer.serialize_str("license_sale_commission"),
            Self::LicenseSaleRevenue => serializer.serialize_str("license_sale_revenue"),
            Self::SoftwareRentalRevshare => serializer.serialize_str("software_rental_revshare"),
            Self::SoftwareRentalTransaction => {
                serializer.serialize_str("software_rental_transaction")
            }
            Self::MiscPurchase => serializer.serialize_str("misc_purchase"),
            Self::MiscRefund => serializer.serialize_str("misc_refund"),
            Self::ReferralBonus => serializer.serialize_str("referral_bonus"),
            Self::PromoReversal => serializer.serialize_str("promo_reversal"),
            Self::PaymentReceivableSettlement => {
                serializer.serialize_str("payment_receivable_settlement")
            }
            Self::PlatformEarningSettlement => {
                serializer.serialize_str("platform_earning_settlement")
            }
            Self::PlatformEarningUnreconciledClearing => {
                serializer.serialize_str("platform_earning_unreconciled_clearing")
            }
            Self::PaymentReceivableClearing => {
                serializer.serialize_str("payment_receivable_clearing")
            }
            Self::PaymentReceivableReversal => {
                serializer.serialize_str("payment_receivable_reversal")
            }
            Self::PaymentUnreconciledClearing => {
                serializer.serialize_str("payment_unreconciled_clearing")
            }
            Self::PaymentUnreconciledReversal => {
                serializer.serialize_str("payment_unreconciled_reversal")
            }
            Self::SettlementShortfallLoss => serializer.serialize_str("settlement_shortfall_loss"),
            Self::FxSettlementGainLoss => serializer.serialize_str("fx_settlement_gain_loss"),
            Self::SettlementRoundingVariance => {
                serializer.serialize_str("settlement_rounding_variance")
            }
            Self::PspPayoutSettlement => serializer.serialize_str("psp_payout_settlement"),
            Self::PspPayoutDeposit => serializer.serialize_str("psp_payout_deposit"),
            Self::PspPayoutReceivable => serializer.serialize_str("psp_payout_receivable"),
            Self::PspPayoutReceivableClearing => {
                serializer.serialize_str("psp_payout_receivable_clearing")
            }
            Self::PayoutUnreconciledClearing => {
                serializer.serialize_str("payout_unreconciled_clearing")
            }
            Self::PspCurrencyConversionOutgoing => {
                serializer.serialize_str("psp_currency_conversion_outgoing")
            }
            Self::PspCurrencyConversionIncoming => {
                serializer.serialize_str("psp_currency_conversion_incoming")
            }
            Self::PspCurrencyConversionReceivable => {
                serializer.serialize_str("psp_currency_conversion_receivable")
            }
            Self::PspCurrencyConversionReceivableClearing => {
                serializer.serialize_str("psp_currency_conversion_receivable_clearing")
            }
            Self::PspFxAdjustment => serializer.serialize_str("psp_fx_adjustment"),
            Self::RefundSettlement => serializer.serialize_str("refund_settlement"),
            Self::RefundSettlementReversal => {
                serializer.serialize_str("refund_settlement_reversal")
            }
            Self::RefundPayableClearing => serializer.serialize_str("refund_payable_clearing"),
            Self::RefundPayableReversal => serializer.serialize_str("refund_payable_reversal"),
            Self::RefundUnreconciledClearing => {
                serializer.serialize_str("refund_unreconciled_clearing")
            }
            Self::RefundUnreconciledReversal => {
                serializer.serialize_str("refund_unreconciled_reversal")
            }
            Self::PspAccountDebit => serializer.serialize_str("psp_account_debit"),
            Self::PaymentRefundFee => serializer.serialize_str("payment_refund_fee"),
            Self::DisputeSettlement => serializer.serialize_str("dispute_settlement"),
            Self::DisputeSettlementReversal => {
                serializer.serialize_str("dispute_settlement_reversal")
            }
            Self::DisputePayableClearing => serializer.serialize_str("dispute_payable_clearing"),
            Self::DisputePayableReversal => serializer.serialize_str("dispute_payable_reversal"),
            Self::DisputeUnreconciledClearing => {
                serializer.serialize_str("dispute_unreconciled_clearing")
            }
            Self::DisputeUnreconciledReversal => {
                serializer.serialize_str("dispute_unreconciled_reversal")
            }
            Self::PspReserveHold => serializer.serialize_str("psp_reserve_hold"),
            Self::PspReserveRelease => serializer.serialize_str("psp_reserve_release"),
            Self::PspDisputeFee => serializer.serialize_str("psp_dispute_fee"),
            Self::PspDisputeAlertFee => serializer.serialize_str("psp_dispute_alert_fee"),
            Self::PspRefundFee => serializer.serialize_str("psp_refund_fee"),
            Self::PspAccountUpdaterFee => serializer.serialize_str("psp_account_updater_fee"),
            Self::PspAdjustedProcessingFee => {
                serializer.serialize_str("psp_adjusted_processing_fee")
            }
            Self::PspAuthenticationFee => serializer.serialize_str("psp_authentication_fee"),
            Self::PspCommissionFee => serializer.serialize_str("psp_commission_fee"),
            Self::PspFixedFee => serializer.serialize_str("psp_fixed_fee"),
            Self::PspGatewayFee => serializer.serialize_str("psp_gateway_fee"),
            Self::PspInterchangeFee => serializer.serialize_str("psp_interchange_fee"),
            Self::PspInvoiceTaxFee => serializer.serialize_str("psp_invoice_tax_fee"),
            Self::PspMarkupFee => serializer.serialize_str("psp_markup_fee"),
            Self::PspNetworkTokenFee => serializer.serialize_str("psp_network_token_fee"),
            Self::PspOptimizationFee => serializer.serialize_str("psp_optimization_fee"),
            Self::PspPaymentMethodFee => serializer.serialize_str("psp_payment_method_fee"),
            Self::PspPayoutFee => serializer.serialize_str("psp_payout_fee"),
            Self::PspProcessingFee => serializer.serialize_str("psp_processing_fee"),
            Self::PspRiskFee => serializer.serialize_str("psp_risk_fee"),
            Self::PspSchemeFee => serializer.serialize_str("psp_scheme_fee"),
            Self::PspVariableFee => serializer.serialize_str("psp_variable_fee"),
            Self::PspBillingFee => serializer.serialize_str("psp_billing_fee"),
            Self::PspTaxServiceFee => serializer.serialize_str("psp_tax_service_fee"),
            Self::PspConnectFee => serializer.serialize_str("psp_connect_fee"),
            Self::ExternalAccountConnectionFee => {
                serializer.serialize_str("external_account_connection_fee")
            }
            Self::PspExternalAccountConnectionFee => {
                serializer.serialize_str("psp_external_account_connection_fee")
            }
            Self::PspInvoicingFee => serializer.serialize_str("psp_invoicing_fee"),
            Self::PspTerminalFee => serializer.serialize_str("psp_terminal_fee"),
            Self::TreasuryFee => serializer.serialize_str("treasury_fee"),
            Self::PspTreasuryFee => serializer.serialize_str("psp_treasury_fee"),
            Self::AcceleratedSettlementFee => {
                serializer.serialize_str("accelerated_settlement_fee")
            }
            Self::PspAcceleratedSettlementFee => {
                serializer.serialize_str("psp_accelerated_settlement_fee")
            }
            Self::PspReceivablePooled => serializer.serialize_str("psp_receivable_pooled"),
            Self::PspPoolSettlement => serializer.serialize_str("psp_pool_settlement"),
            Self::PspPoolClearing => serializer.serialize_str("psp_pool_clearing"),
            Self::PspPoolRefundSettlement => serializer.serialize_str("psp_pool_refund_settlement"),
            Self::PspPoolRefundClearing => serializer.serialize_str("psp_pool_refund_clearing"),
            Self::PspCorrection => serializer.serialize_str("psp_correction"),
            Self::PspPayinSettlement => serializer.serialize_str("psp_payin_settlement"),
            Self::PspPayinClearing => serializer.serialize_str("psp_payin_clearing"),
            Self::PspBankPullSettlement => serializer.serialize_str("psp_bank_pull_settlement"),
            Self::PspBankPullClearing => serializer.serialize_str("psp_bank_pull_clearing"),
            Self::PspPoolDisputeSettlement => {
                serializer.serialize_str("psp_pool_dispute_settlement")
            }
            Self::PspPayoutConsolidation => serializer.serialize_str("psp_payout_consolidation"),
            Self::PspWithholdingTax => serializer.serialize_str("psp_withholding_tax"),
            Self::PspPayoutSettlementReversal => {
                serializer.serialize_str("psp_payout_settlement_reversal")
            }
            Self::PspTransferSettlement => serializer.serialize_str("psp_transfer_settlement"),
            Self::PspTransferSettlementReversal => {
                serializer.serialize_str("psp_transfer_settlement_reversal")
            }
            Self::PspCollectionSettlement => serializer.serialize_str("psp_collection_settlement"),
            Self::PayoutSubsidy => serializer.serialize_str("payout_subsidy"),
            Self::WithdrawalTopupAdjustment => {
                serializer.serialize_str("withdrawal_topup_adjustment")
            }
            Self::WithdrawalPayableClearing => {
                serializer.serialize_str("withdrawal_payable_clearing")
            }
            Self::WithdrawalPayableClearingReversal => {
                serializer.serialize_str("withdrawal_payable_clearing_reversal")
            }
            Self::IdentityVerificationFee => serializer.serialize_str("identity_verification_fee"),
            Self::PspIdentityVerificationFee => {
                serializer.serialize_str("psp_identity_verification_fee")
            }
            Self::TaxFilingFee => serializer.serialize_str("tax_filing_fee"),
            Self::PspTaxFilingFee => serializer.serialize_str("psp_tax_filing_fee"),
            Self::PspServiceFee => serializer.serialize_str("psp_service_fee"),
            Self::RecipientWalletLoad => serializer.serialize_str("recipient_wallet_load"),
            Self::PspRecipientWalletLoad => serializer.serialize_str("psp_recipient_wallet_load"),
            Self::DisputeManagementFee => serializer.serialize_str("dispute_management_fee"),
            Self::PspDisputeManagementFee => serializer.serialize_str("psp_dispute_management_fee"),
            Self::PspClawbackSettlement => serializer.serialize_str("psp_clawback_settlement"),
            Self::PspClawbackSettlementReversal => {
                serializer.serialize_str("psp_clawback_settlement_reversal")
            }
            Self::ClawbackReceivableSettlement => {
                serializer.serialize_str("clawback_receivable_settlement")
            }
            Self::ClawbackReceivableSettlementReversal => {
                serializer.serialize_str("clawback_receivable_settlement_reversal")
            }
            Self::ClawbackFee => serializer.serialize_str("clawback_fee"),
            Self::PspClawbackFee => serializer.serialize_str("psp_clawback_fee"),
            Self::Deposit => serializer.serialize_str("deposit"),
            Self::WalletTransferIncoming => serializer.serialize_str("wallet_transfer_incoming"),
            Self::WalletTransferOutgoing => serializer.serialize_str("wallet_transfer_outgoing"),
            Self::SwapSource => serializer.serialize_str("swap_source"),
            Self::SwapTarget => serializer.serialize_str("swap_target"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for LedgerActivityLineType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "psp_payment_receivable" => Ok(Self::PspPaymentReceivable),
            "payment_gross" => Ok(Self::PaymentGross),
            "topup" => Ok(Self::Topup),
            "topup_fee" => Ok(Self::TopupFee),
            "payment_gross_reversal" => Ok(Self::PaymentGrossReversal),
            "topup_reversal" => Ok(Self::TopupReversal),
            "payment_refund" => Ok(Self::PaymentRefund),
            "payment_refund_reversal" => Ok(Self::PaymentRefundReversal),
            "platform_balance_payment" => Ok(Self::PlatformBalancePayment),
            "platform_balance_payment_refund" => Ok(Self::PlatformBalancePaymentRefund),
            "ad_spend_purchase" => Ok(Self::AdSpendPurchase),
            "psp_refund_payable" => Ok(Self::PspRefundPayable),
            "passthrough_gmv" => Ok(Self::PassthroughGmv),
            "passthrough_gmv_offset" => Ok(Self::PassthroughGmvOffset),
            "installment_default" => Ok(Self::InstallmentDefault),
            "aggregated_fee" => Ok(Self::AggregatedFee),
            "payment_processing_fixed_fee" => Ok(Self::PaymentProcessingFixedFee),
            "payment_processing_percentage_fee" => Ok(Self::PaymentProcessingPercentageFee),
            "billing_percentage_fee" => Ok(Self::BillingPercentageFee),
            "cross_border_percentage_fee" => Ok(Self::CrossBorderPercentageFee),
            "fx_percentage_fee" => Ok(Self::FxPercentageFee),
            "orchestration_percentage_fee" => Ok(Self::OrchestrationPercentageFee),
            "revshare_percentage_fee" => Ok(Self::RevsharePercentageFee),
            "stripe_domestic_processing_fee" => Ok(Self::StripeDomesticProcessingFee),
            "stripe_international_processing_fee" => Ok(Self::StripeInternationalProcessingFee),
            "three_ds_fixed_fee" => Ok(Self::ThreeDsFixedFee),
            "whop_processing_fee" => Ok(Self::WhopProcessingFee),
            "economic_intelligence_percentage_fee" => Ok(Self::EconomicIntelligencePercentageFee),
            "high_risk_merchant_fee" => Ok(Self::HighRiskMerchantFee),
            "fraud_prevention_fee" => Ok(Self::FraudPreventionFee),
            "buyer_fee" => Ok(Self::BuyerFee),
            "sales_tax_fee" => Ok(Self::SalesTaxFee),
            "payout_fee" => Ok(Self::PayoutFee),
            "affiliate_fee" => Ok(Self::AffiliateFee),
            "marketplace_affiliate_fee" => Ok(Self::MarketplaceAffiliateFee),
            "application_fee" => Ok(Self::ApplicationFee),
            "platform_markup_fee" => Ok(Self::PlatformMarkupFee),
            "platform_markup_fee_payout" => Ok(Self::PlatformMarkupFeePayout),
            "sales_tax_remittance" => Ok(Self::SalesTaxRemittance),
            "sales_tax_remittance_reversal" => Ok(Self::SalesTaxRemittanceReversal),
            "sales_tax_collected" => Ok(Self::SalesTaxCollected),
            "sales_tax_collected_reversal" => Ok(Self::SalesTaxCollectedReversal),
            "sales_tax_remitted" => Ok(Self::SalesTaxRemitted),
            "payment_dispute" => Ok(Self::PaymentDispute),
            "payment_dispute_adjustment" => Ok(Self::PaymentDisputeAdjustment),
            "payment_dispute_fee" => Ok(Self::PaymentDisputeFee),
            "payment_dispute_reversal" => Ok(Self::PaymentDisputeReversal),
            "platform_covered_dispute" => Ok(Self::PlatformCoveredDispute),
            "dispute_alert_fee" => Ok(Self::DisputeAlertFee),
            "dispute_representment_fee" => Ok(Self::DisputeRepresentmentFee),
            "payment_return_fee" => Ok(Self::PaymentReturnFee),
            "dispute_hold_adjustment" => Ok(Self::DisputeHoldAdjustment),
            "dispute_hold_blocked" => Ok(Self::DisputeHoldBlocked),
            "psp_dispute_payable" => Ok(Self::PspDisputePayable),
            "withdrawal" => Ok(Self::Withdrawal),
            "withdrawal_payable" => Ok(Self::WithdrawalPayable),
            "withdrawal_payable_reversal" => Ok(Self::WithdrawalPayableReversal),
            "withdrawal_fee" => Ok(Self::WithdrawalFee),
            "withdrawal_fee_reversal" => Ok(Self::WithdrawalFeeReversal),
            "withdrawal_markup_fee" => Ok(Self::WithdrawalMarkupFee),
            "withdrawal_markup_fee_reversal" => Ok(Self::WithdrawalMarkupFeeReversal),
            "withdrawal_markup_fee_payout" => Ok(Self::WithdrawalMarkupFeePayout),
            "withdrawal_markup_fee_payout_reversal" => Ok(Self::WithdrawalMarkupFeePayoutReversal),
            "withdrawal_reversal" => Ok(Self::WithdrawalReversal),
            "withdrawal_clawback" => Ok(Self::WithdrawalClawback),
            "withdrawal_clawback_reversal" => Ok(Self::WithdrawalClawbackReversal),
            "payout_receivable" => Ok(Self::PayoutReceivable),
            "withdrawal_reclassification" => Ok(Self::WithdrawalReclassification),
            "clawback_receivable" => Ok(Self::ClawbackReceivable),
            "clawback_receivable_reversal" => Ok(Self::ClawbackReceivableReversal),
            "capital_withholding" => Ok(Self::CapitalWithholding),
            "capital_withholding_reversal" => Ok(Self::CapitalWithholdingReversal),
            "capital_hold" => Ok(Self::CapitalHold),
            "capital_hold_release" => Ok(Self::CapitalHoldRelease),
            "capital_remittance" => Ok(Self::CapitalRemittance),
            "capital_remittance_reversal" => Ok(Self::CapitalRemittanceReversal),
            "treasury_payin" => Ok(Self::TreasuryPayin),
            "treasury_payin_receivable" => Ok(Self::TreasuryPayinReceivable),
            "onchain_deposit" => Ok(Self::OnchainDeposit),
            "onchain_deposit_offset" => Ok(Self::OnchainDepositOffset),
            "onchain_withdrawal" => Ok(Self::OnchainWithdrawal),
            "onchain_withdrawal_offset" => Ok(Self::OnchainWithdrawalOffset),
            "trading_account_withdrawal" => Ok(Self::TradingAccountWithdrawal),
            "trading_account_withdrawal_offset" => Ok(Self::TradingAccountWithdrawalOffset),
            "trading_account_deposit" => Ok(Self::TradingAccountDeposit),
            "trading_account_deposit_offset" => Ok(Self::TradingAccountDepositOffset),
            "perps_load_transfer" => Ok(Self::PerpsLoadTransfer),
            "perps_load_deposit" => Ok(Self::PerpsLoadDeposit),
            "perps_unload_transfer" => Ok(Self::PerpsUnloadTransfer),
            "perps_unload_deposit" => Ok(Self::PerpsUnloadDeposit),
            "perps_bridge_fee" => Ok(Self::PerpsBridgeFee),
            "perps_bridge_fee_offset" => Ok(Self::PerpsBridgeFeeOffset),
            "perps_volume" => Ok(Self::PerpsVolume),
            "perps_volume_offset" => Ok(Self::PerpsVolumeOffset),
            "perps_fee" => Ok(Self::PerpsFee),
            "perps_fee_offset" => Ok(Self::PerpsFeeOffset),
            "platform_trading_commission" => Ok(Self::PlatformTradingCommission),
            "trading_fee" => Ok(Self::TradingFee),
            "perps_realized_pnl" => Ok(Self::PerpsRealizedPnl),
            "perps_realized_pnl_offset" => Ok(Self::PerpsRealizedPnlOffset),
            "onchain_wallet_transfer_outgoing" => Ok(Self::OnchainWalletTransferOutgoing),
            "onchain_wallet_transfer_incoming" => Ok(Self::OnchainWalletTransferIncoming),
            "onchain_platform_transfer_outgoing" => Ok(Self::OnchainPlatformTransferOutgoing),
            "onchain_platform_transfer_incoming" => Ok(Self::OnchainPlatformTransferIncoming),
            "onchain_internal_transfer_outgoing" => Ok(Self::OnchainInternalTransferOutgoing),
            "onchain_internal_transfer_incoming" => Ok(Self::OnchainInternalTransferIncoming),
            "onchain_swap_source" => Ok(Self::OnchainSwapSource),
            "onchain_swap_target" => Ok(Self::OnchainSwapTarget),
            "onchain_swap_offset" => Ok(Self::OnchainSwapOffset),
            "whop_swap_fee_received" => Ok(Self::WhopSwapFeeReceived),
            "swap_fee" => Ok(Self::SwapFee),
            "internal_withdrawal" => Ok(Self::InternalWithdrawal),
            "internal_withdrawal_reversal" => Ok(Self::InternalWithdrawalReversal),
            "internal_withdrawal_payable" => Ok(Self::InternalWithdrawalPayable),
            "internal_withdrawal_payable_reversal" => Ok(Self::InternalWithdrawalPayableReversal),
            "internal_withdrawal_fee" => Ok(Self::InternalWithdrawalFee),
            "internal_withdrawal_fee_reversal" => Ok(Self::InternalWithdrawalFeeReversal),
            "internal_withdrawal_markup_fee" => Ok(Self::InternalWithdrawalMarkupFee),
            "internal_withdrawal_markup_fee_reversal" => {
                Ok(Self::InternalWithdrawalMarkupFeeReversal)
            }
            "internal_withdrawal_markup_fee_payout" => Ok(Self::InternalWithdrawalMarkupFeePayout),
            "internal_withdrawal_markup_fee_payout_reversal" => {
                Ok(Self::InternalWithdrawalMarkupFeePayoutReversal)
            }
            "internal_withdrawal_in_transit" => Ok(Self::InternalWithdrawalInTransit),
            "internal_withdrawal_complete" => Ok(Self::InternalWithdrawalComplete),
            "internal_withdrawal_in_transit_reversal" => {
                Ok(Self::InternalWithdrawalInTransitReversal)
            }
            "card_load_transfer" => Ok(Self::CardLoadTransfer),
            "card_load_deposit" => Ok(Self::CardLoadDeposit),
            "card_unload_transfer" => Ok(Self::CardUnloadTransfer),
            "card_unload_deposit" => Ok(Self::CardUnloadDeposit),
            "external_card_load_deposit" => Ok(Self::ExternalCardLoadDeposit),
            "external_card_load_offset" => Ok(Self::ExternalCardLoadOffset),
            "card_spend_authorization" => Ok(Self::CardSpendAuthorization),
            "card_spend_authorization_hold" => Ok(Self::CardSpendAuthorizationHold),
            "card_spend_capture" => Ok(Self::CardSpendCapture),
            "card_spend_capture_offset" => Ok(Self::CardSpendCaptureOffset),
            "card_spend_authorization_void" => Ok(Self::CardSpendAuthorizationVoid),
            "card_spend_authorization_void_release" => Ok(Self::CardSpendAuthorizationVoidRelease),
            "card_spend_refund" => Ok(Self::CardSpendRefund),
            "card_spend_refund_offset" => Ok(Self::CardSpendRefundOffset),
            "cashback" => Ok(Self::Cashback),
            "cashback_expense" => Ok(Self::CashbackExpense),
            "cashback_funding" => Ok(Self::CashbackFunding),
            "card_interchange_receivable" => Ok(Self::CardInterchangeReceivable),
            "card_interchange" => Ok(Self::CardInterchange),
            "account_settlement" => Ok(Self::AccountSettlement),
            "currency_conversion_outgoing" => Ok(Self::CurrencyConversionOutgoing),
            "currency_conversion_incoming" => Ok(Self::CurrencyConversionIncoming),
            "balance_reservation" => Ok(Self::BalanceReservation),
            "balance_reservation_hold" => Ok(Self::BalanceReservationHold),
            "balance_reservation_reversal" => Ok(Self::BalanceReservationReversal),
            "balance_reservation_release" => Ok(Self::BalanceReservationRelease),
            "bad_debt_offset" => Ok(Self::BadDebtOffset),
            "bad_debt_expense" => Ok(Self::BadDebtExpense),
            "platform_credits_payment" => Ok(Self::PlatformCreditsPayment),
            "platform_credits_payment_refund" => Ok(Self::PlatformCreditsPaymentRefund),
            "platform_credits_granted" => Ok(Self::PlatformCreditsGranted),
            "levels_points_card" => Ok(Self::LevelsPointsCard),
            "levels_points_ads" => Ok(Self::LevelsPointsAds),
            "levels_points_trading" => Ok(Self::LevelsPointsTrading),
            "levels_points_partner" => Ok(Self::LevelsPointsPartner),
            "levels_points_burned" => Ok(Self::LevelsPointsBurned),
            "levels_points_issued" => Ok(Self::LevelsPointsIssued),
            "levels_rebate" => Ok(Self::LevelsRebate),
            "marketing_expense" => Ok(Self::MarketingExpense),
            "platform_balance_transfer_outgoing" => Ok(Self::PlatformBalanceTransferOutgoing),
            "platform_balance_transfer_incoming" => Ok(Self::PlatformBalanceTransferIncoming),
            "internal_balance_transfer_outgoing" => Ok(Self::InternalBalanceTransferOutgoing),
            "internal_balance_transfer_incoming" => Ok(Self::InternalBalanceTransferIncoming),
            "platform_balance_transfer_fee" => Ok(Self::PlatformBalanceTransferFee),
            "connected_account_clawback" => Ok(Self::ConnectedAccountClawback),
            "connected_account_negative_balance" => Ok(Self::ConnectedAccountNegativeBalance),
            "ad_publisher_payout" => Ok(Self::AdPublisherPayout),
            "ad_publisher_payout_received" => Ok(Self::AdPublisherPayoutReceived),
            "company_referral" => Ok(Self::CompanyReferral),
            "platform_affiliate_payment" => Ok(Self::PlatformAffiliatePayment),
            "platform_affiliate_payment_reversal" => Ok(Self::PlatformAffiliatePaymentReversal),
            "onboarding_reward" => Ok(Self::OnboardingReward),
            "partner_grant_reward" => Ok(Self::PartnerGrantReward),
            "payment_revshare" => Ok(Self::PaymentRevshare),
            "payment_revshare_payout" => Ok(Self::PaymentRevsharePayout),
            "payment_revshare_reversal" => Ok(Self::PaymentRevshareReversal),
            "payment_revshare_refund" => Ok(Self::PaymentRevshareRefund),
            "payment_referral" => Ok(Self::PaymentReferral),
            "payment_referral_payable" => Ok(Self::PaymentReferralPayable),
            "payment_referral_reversal" => Ok(Self::PaymentReferralReversal),
            "payment_referral_refund" => Ok(Self::PaymentReferralRefund),
            "application_fee_payable" => Ok(Self::ApplicationFeePayable),
            "application_fee_payout" => Ok(Self::ApplicationFeePayout),
            "platform_earning" => Ok(Self::PlatformEarning),
            "airdrop" => Ok(Self::Airdrop),
            "airdrop_reversal" => Ok(Self::AirdropReversal),
            "airdrop_expense" => Ok(Self::AirdropExpense),
            "airdrop_expense_reversal" => Ok(Self::AirdropExpenseReversal),
            "airdrop_link_funded" => Ok(Self::AirdropLinkFunded),
            "airdrop_link_canceled" => Ok(Self::AirdropLinkCanceled),
            "airdrop_link_returned" => Ok(Self::AirdropLinkReturned),
            "airdrop_link_claimed" => Ok(Self::AirdropLinkClaimed),
            "airdrop_link_redeemed" => Ok(Self::AirdropLinkRedeemed),
            "airdrop_link_created" => Ok(Self::AirdropLinkCreated),
            "resolution_center_refund" => Ok(Self::ResolutionCenterRefund),
            "ad_funding_disbursement" => Ok(Self::AdFundingDisbursement),
            "ad_campaign_budget" => Ok(Self::AdCampaignBudget),
            "ads_card_spread" => Ok(Self::AdsCardSpread),
            "ad_spend_charge" => Ok(Self::AdSpendCharge),
            "ad_network_cost" => Ok(Self::AdNetworkCost),
            "ad_spend_margin" => Ok(Self::AdSpendMargin),
            "ad_income_expense" => Ok(Self::AdIncomeExpense),
            "ad_income_receipt" => Ok(Self::AdIncomeReceipt),
            "ad_budget_release" => Ok(Self::AdBudgetRelease),
            "ad_network_settlement" => Ok(Self::AdNetworkSettlement),
            "ad_affiliate_payout" => Ok(Self::AdAffiliatePayout),
            "ad_affiliate_payout_received" => Ok(Self::AdAffiliatePayoutReceived),
            "ad_balance_funding_receipt" => Ok(Self::AdBalanceFundingReceipt),
            "misc_reversal" => Ok(Self::MiscReversal),
            "fx_gain_loss" => Ok(Self::FxGainLoss),
            "fx_markup" => Ok(Self::FxMarkup),
            "legacy_crypto_payment" => Ok(Self::LegacyCryptoPayment),
            "legacy_payment" => Ok(Self::LegacyPayment),
            "legacy_payment_refund" => Ok(Self::LegacyPaymentRefund),
            "license_sale" => Ok(Self::LicenseSale),
            "license_sale_commission" => Ok(Self::LicenseSaleCommission),
            "license_sale_revenue" => Ok(Self::LicenseSaleRevenue),
            "software_rental_revshare" => Ok(Self::SoftwareRentalRevshare),
            "software_rental_transaction" => Ok(Self::SoftwareRentalTransaction),
            "misc_purchase" => Ok(Self::MiscPurchase),
            "misc_refund" => Ok(Self::MiscRefund),
            "referral_bonus" => Ok(Self::ReferralBonus),
            "promo_reversal" => Ok(Self::PromoReversal),
            "payment_receivable_settlement" => Ok(Self::PaymentReceivableSettlement),
            "platform_earning_settlement" => Ok(Self::PlatformEarningSettlement),
            "platform_earning_unreconciled_clearing" => {
                Ok(Self::PlatformEarningUnreconciledClearing)
            }
            "payment_receivable_clearing" => Ok(Self::PaymentReceivableClearing),
            "payment_receivable_reversal" => Ok(Self::PaymentReceivableReversal),
            "payment_unreconciled_clearing" => Ok(Self::PaymentUnreconciledClearing),
            "payment_unreconciled_reversal" => Ok(Self::PaymentUnreconciledReversal),
            "settlement_shortfall_loss" => Ok(Self::SettlementShortfallLoss),
            "fx_settlement_gain_loss" => Ok(Self::FxSettlementGainLoss),
            "settlement_rounding_variance" => Ok(Self::SettlementRoundingVariance),
            "psp_payout_settlement" => Ok(Self::PspPayoutSettlement),
            "psp_payout_deposit" => Ok(Self::PspPayoutDeposit),
            "psp_payout_receivable" => Ok(Self::PspPayoutReceivable),
            "psp_payout_receivable_clearing" => Ok(Self::PspPayoutReceivableClearing),
            "payout_unreconciled_clearing" => Ok(Self::PayoutUnreconciledClearing),
            "psp_currency_conversion_outgoing" => Ok(Self::PspCurrencyConversionOutgoing),
            "psp_currency_conversion_incoming" => Ok(Self::PspCurrencyConversionIncoming),
            "psp_currency_conversion_receivable" => Ok(Self::PspCurrencyConversionReceivable),
            "psp_currency_conversion_receivable_clearing" => {
                Ok(Self::PspCurrencyConversionReceivableClearing)
            }
            "psp_fx_adjustment" => Ok(Self::PspFxAdjustment),
            "refund_settlement" => Ok(Self::RefundSettlement),
            "refund_settlement_reversal" => Ok(Self::RefundSettlementReversal),
            "refund_payable_clearing" => Ok(Self::RefundPayableClearing),
            "refund_payable_reversal" => Ok(Self::RefundPayableReversal),
            "refund_unreconciled_clearing" => Ok(Self::RefundUnreconciledClearing),
            "refund_unreconciled_reversal" => Ok(Self::RefundUnreconciledReversal),
            "psp_account_debit" => Ok(Self::PspAccountDebit),
            "payment_refund_fee" => Ok(Self::PaymentRefundFee),
            "dispute_settlement" => Ok(Self::DisputeSettlement),
            "dispute_settlement_reversal" => Ok(Self::DisputeSettlementReversal),
            "dispute_payable_clearing" => Ok(Self::DisputePayableClearing),
            "dispute_payable_reversal" => Ok(Self::DisputePayableReversal),
            "dispute_unreconciled_clearing" => Ok(Self::DisputeUnreconciledClearing),
            "dispute_unreconciled_reversal" => Ok(Self::DisputeUnreconciledReversal),
            "psp_reserve_hold" => Ok(Self::PspReserveHold),
            "psp_reserve_release" => Ok(Self::PspReserveRelease),
            "psp_dispute_fee" => Ok(Self::PspDisputeFee),
            "psp_dispute_alert_fee" => Ok(Self::PspDisputeAlertFee),
            "psp_refund_fee" => Ok(Self::PspRefundFee),
            "psp_account_updater_fee" => Ok(Self::PspAccountUpdaterFee),
            "psp_adjusted_processing_fee" => Ok(Self::PspAdjustedProcessingFee),
            "psp_authentication_fee" => Ok(Self::PspAuthenticationFee),
            "psp_commission_fee" => Ok(Self::PspCommissionFee),
            "psp_fixed_fee" => Ok(Self::PspFixedFee),
            "psp_gateway_fee" => Ok(Self::PspGatewayFee),
            "psp_interchange_fee" => Ok(Self::PspInterchangeFee),
            "psp_invoice_tax_fee" => Ok(Self::PspInvoiceTaxFee),
            "psp_markup_fee" => Ok(Self::PspMarkupFee),
            "psp_network_token_fee" => Ok(Self::PspNetworkTokenFee),
            "psp_optimization_fee" => Ok(Self::PspOptimizationFee),
            "psp_payment_method_fee" => Ok(Self::PspPaymentMethodFee),
            "psp_payout_fee" => Ok(Self::PspPayoutFee),
            "psp_processing_fee" => Ok(Self::PspProcessingFee),
            "psp_risk_fee" => Ok(Self::PspRiskFee),
            "psp_scheme_fee" => Ok(Self::PspSchemeFee),
            "psp_variable_fee" => Ok(Self::PspVariableFee),
            "psp_billing_fee" => Ok(Self::PspBillingFee),
            "psp_tax_service_fee" => Ok(Self::PspTaxServiceFee),
            "psp_connect_fee" => Ok(Self::PspConnectFee),
            "external_account_connection_fee" => Ok(Self::ExternalAccountConnectionFee),
            "psp_external_account_connection_fee" => Ok(Self::PspExternalAccountConnectionFee),
            "psp_invoicing_fee" => Ok(Self::PspInvoicingFee),
            "psp_terminal_fee" => Ok(Self::PspTerminalFee),
            "treasury_fee" => Ok(Self::TreasuryFee),
            "psp_treasury_fee" => Ok(Self::PspTreasuryFee),
            "accelerated_settlement_fee" => Ok(Self::AcceleratedSettlementFee),
            "psp_accelerated_settlement_fee" => Ok(Self::PspAcceleratedSettlementFee),
            "psp_receivable_pooled" => Ok(Self::PspReceivablePooled),
            "psp_pool_settlement" => Ok(Self::PspPoolSettlement),
            "psp_pool_clearing" => Ok(Self::PspPoolClearing),
            "psp_pool_refund_settlement" => Ok(Self::PspPoolRefundSettlement),
            "psp_pool_refund_clearing" => Ok(Self::PspPoolRefundClearing),
            "psp_correction" => Ok(Self::PspCorrection),
            "psp_payin_settlement" => Ok(Self::PspPayinSettlement),
            "psp_payin_clearing" => Ok(Self::PspPayinClearing),
            "psp_bank_pull_settlement" => Ok(Self::PspBankPullSettlement),
            "psp_bank_pull_clearing" => Ok(Self::PspBankPullClearing),
            "psp_pool_dispute_settlement" => Ok(Self::PspPoolDisputeSettlement),
            "psp_payout_consolidation" => Ok(Self::PspPayoutConsolidation),
            "psp_withholding_tax" => Ok(Self::PspWithholdingTax),
            "psp_payout_settlement_reversal" => Ok(Self::PspPayoutSettlementReversal),
            "psp_transfer_settlement" => Ok(Self::PspTransferSettlement),
            "psp_transfer_settlement_reversal" => Ok(Self::PspTransferSettlementReversal),
            "psp_collection_settlement" => Ok(Self::PspCollectionSettlement),
            "payout_subsidy" => Ok(Self::PayoutSubsidy),
            "withdrawal_topup_adjustment" => Ok(Self::WithdrawalTopupAdjustment),
            "withdrawal_payable_clearing" => Ok(Self::WithdrawalPayableClearing),
            "withdrawal_payable_clearing_reversal" => Ok(Self::WithdrawalPayableClearingReversal),
            "identity_verification_fee" => Ok(Self::IdentityVerificationFee),
            "psp_identity_verification_fee" => Ok(Self::PspIdentityVerificationFee),
            "tax_filing_fee" => Ok(Self::TaxFilingFee),
            "psp_tax_filing_fee" => Ok(Self::PspTaxFilingFee),
            "psp_service_fee" => Ok(Self::PspServiceFee),
            "recipient_wallet_load" => Ok(Self::RecipientWalletLoad),
            "psp_recipient_wallet_load" => Ok(Self::PspRecipientWalletLoad),
            "dispute_management_fee" => Ok(Self::DisputeManagementFee),
            "psp_dispute_management_fee" => Ok(Self::PspDisputeManagementFee),
            "psp_clawback_settlement" => Ok(Self::PspClawbackSettlement),
            "psp_clawback_settlement_reversal" => Ok(Self::PspClawbackSettlementReversal),
            "clawback_receivable_settlement" => Ok(Self::ClawbackReceivableSettlement),
            "clawback_receivable_settlement_reversal" => {
                Ok(Self::ClawbackReceivableSettlementReversal)
            }
            "clawback_fee" => Ok(Self::ClawbackFee),
            "psp_clawback_fee" => Ok(Self::PspClawbackFee),
            "deposit" => Ok(Self::Deposit),
            "wallet_transfer_incoming" => Ok(Self::WalletTransferIncoming),
            "wallet_transfer_outgoing" => Ok(Self::WalletTransferOutgoing),
            "swap_source" => Ok(Self::SwapSource),
            "swap_target" => Ok(Self::SwapTarget),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for LedgerActivityLineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PspPaymentReceivable => write!(f, "psp_payment_receivable"),
            Self::PaymentGross => write!(f, "payment_gross"),
            Self::Topup => write!(f, "topup"),
            Self::TopupFee => write!(f, "topup_fee"),
            Self::PaymentGrossReversal => write!(f, "payment_gross_reversal"),
            Self::TopupReversal => write!(f, "topup_reversal"),
            Self::PaymentRefund => write!(f, "payment_refund"),
            Self::PaymentRefundReversal => write!(f, "payment_refund_reversal"),
            Self::PlatformBalancePayment => write!(f, "platform_balance_payment"),
            Self::PlatformBalancePaymentRefund => write!(f, "platform_balance_payment_refund"),
            Self::AdSpendPurchase => write!(f, "ad_spend_purchase"),
            Self::PspRefundPayable => write!(f, "psp_refund_payable"),
            Self::PassthroughGmv => write!(f, "passthrough_gmv"),
            Self::PassthroughGmvOffset => write!(f, "passthrough_gmv_offset"),
            Self::InstallmentDefault => write!(f, "installment_default"),
            Self::AggregatedFee => write!(f, "aggregated_fee"),
            Self::PaymentProcessingFixedFee => write!(f, "payment_processing_fixed_fee"),
            Self::PaymentProcessingPercentageFee => write!(f, "payment_processing_percentage_fee"),
            Self::BillingPercentageFee => write!(f, "billing_percentage_fee"),
            Self::CrossBorderPercentageFee => write!(f, "cross_border_percentage_fee"),
            Self::FxPercentageFee => write!(f, "fx_percentage_fee"),
            Self::OrchestrationPercentageFee => write!(f, "orchestration_percentage_fee"),
            Self::RevsharePercentageFee => write!(f, "revshare_percentage_fee"),
            Self::StripeDomesticProcessingFee => write!(f, "stripe_domestic_processing_fee"),
            Self::StripeInternationalProcessingFee => {
                write!(f, "stripe_international_processing_fee")
            }
            Self::ThreeDsFixedFee => write!(f, "three_ds_fixed_fee"),
            Self::WhopProcessingFee => write!(f, "whop_processing_fee"),
            Self::EconomicIntelligencePercentageFee => {
                write!(f, "economic_intelligence_percentage_fee")
            }
            Self::HighRiskMerchantFee => write!(f, "high_risk_merchant_fee"),
            Self::FraudPreventionFee => write!(f, "fraud_prevention_fee"),
            Self::BuyerFee => write!(f, "buyer_fee"),
            Self::SalesTaxFee => write!(f, "sales_tax_fee"),
            Self::PayoutFee => write!(f, "payout_fee"),
            Self::AffiliateFee => write!(f, "affiliate_fee"),
            Self::MarketplaceAffiliateFee => write!(f, "marketplace_affiliate_fee"),
            Self::ApplicationFee => write!(f, "application_fee"),
            Self::PlatformMarkupFee => write!(f, "platform_markup_fee"),
            Self::PlatformMarkupFeePayout => write!(f, "platform_markup_fee_payout"),
            Self::SalesTaxRemittance => write!(f, "sales_tax_remittance"),
            Self::SalesTaxRemittanceReversal => write!(f, "sales_tax_remittance_reversal"),
            Self::SalesTaxCollected => write!(f, "sales_tax_collected"),
            Self::SalesTaxCollectedReversal => write!(f, "sales_tax_collected_reversal"),
            Self::SalesTaxRemitted => write!(f, "sales_tax_remitted"),
            Self::PaymentDispute => write!(f, "payment_dispute"),
            Self::PaymentDisputeAdjustment => write!(f, "payment_dispute_adjustment"),
            Self::PaymentDisputeFee => write!(f, "payment_dispute_fee"),
            Self::PaymentDisputeReversal => write!(f, "payment_dispute_reversal"),
            Self::PlatformCoveredDispute => write!(f, "platform_covered_dispute"),
            Self::DisputeAlertFee => write!(f, "dispute_alert_fee"),
            Self::DisputeRepresentmentFee => write!(f, "dispute_representment_fee"),
            Self::PaymentReturnFee => write!(f, "payment_return_fee"),
            Self::DisputeHoldAdjustment => write!(f, "dispute_hold_adjustment"),
            Self::DisputeHoldBlocked => write!(f, "dispute_hold_blocked"),
            Self::PspDisputePayable => write!(f, "psp_dispute_payable"),
            Self::Withdrawal => write!(f, "withdrawal"),
            Self::WithdrawalPayable => write!(f, "withdrawal_payable"),
            Self::WithdrawalPayableReversal => write!(f, "withdrawal_payable_reversal"),
            Self::WithdrawalFee => write!(f, "withdrawal_fee"),
            Self::WithdrawalFeeReversal => write!(f, "withdrawal_fee_reversal"),
            Self::WithdrawalMarkupFee => write!(f, "withdrawal_markup_fee"),
            Self::WithdrawalMarkupFeeReversal => write!(f, "withdrawal_markup_fee_reversal"),
            Self::WithdrawalMarkupFeePayout => write!(f, "withdrawal_markup_fee_payout"),
            Self::WithdrawalMarkupFeePayoutReversal => {
                write!(f, "withdrawal_markup_fee_payout_reversal")
            }
            Self::WithdrawalReversal => write!(f, "withdrawal_reversal"),
            Self::WithdrawalClawback => write!(f, "withdrawal_clawback"),
            Self::WithdrawalClawbackReversal => write!(f, "withdrawal_clawback_reversal"),
            Self::PayoutReceivable => write!(f, "payout_receivable"),
            Self::WithdrawalReclassification => write!(f, "withdrawal_reclassification"),
            Self::ClawbackReceivable => write!(f, "clawback_receivable"),
            Self::ClawbackReceivableReversal => write!(f, "clawback_receivable_reversal"),
            Self::CapitalWithholding => write!(f, "capital_withholding"),
            Self::CapitalWithholdingReversal => write!(f, "capital_withholding_reversal"),
            Self::CapitalHold => write!(f, "capital_hold"),
            Self::CapitalHoldRelease => write!(f, "capital_hold_release"),
            Self::CapitalRemittance => write!(f, "capital_remittance"),
            Self::CapitalRemittanceReversal => write!(f, "capital_remittance_reversal"),
            Self::TreasuryPayin => write!(f, "treasury_payin"),
            Self::TreasuryPayinReceivable => write!(f, "treasury_payin_receivable"),
            Self::OnchainDeposit => write!(f, "onchain_deposit"),
            Self::OnchainDepositOffset => write!(f, "onchain_deposit_offset"),
            Self::OnchainWithdrawal => write!(f, "onchain_withdrawal"),
            Self::OnchainWithdrawalOffset => write!(f, "onchain_withdrawal_offset"),
            Self::TradingAccountWithdrawal => write!(f, "trading_account_withdrawal"),
            Self::TradingAccountWithdrawalOffset => write!(f, "trading_account_withdrawal_offset"),
            Self::TradingAccountDeposit => write!(f, "trading_account_deposit"),
            Self::TradingAccountDepositOffset => write!(f, "trading_account_deposit_offset"),
            Self::PerpsLoadTransfer => write!(f, "perps_load_transfer"),
            Self::PerpsLoadDeposit => write!(f, "perps_load_deposit"),
            Self::PerpsUnloadTransfer => write!(f, "perps_unload_transfer"),
            Self::PerpsUnloadDeposit => write!(f, "perps_unload_deposit"),
            Self::PerpsBridgeFee => write!(f, "perps_bridge_fee"),
            Self::PerpsBridgeFeeOffset => write!(f, "perps_bridge_fee_offset"),
            Self::PerpsVolume => write!(f, "perps_volume"),
            Self::PerpsVolumeOffset => write!(f, "perps_volume_offset"),
            Self::PerpsFee => write!(f, "perps_fee"),
            Self::PerpsFeeOffset => write!(f, "perps_fee_offset"),
            Self::PlatformTradingCommission => write!(f, "platform_trading_commission"),
            Self::TradingFee => write!(f, "trading_fee"),
            Self::PerpsRealizedPnl => write!(f, "perps_realized_pnl"),
            Self::PerpsRealizedPnlOffset => write!(f, "perps_realized_pnl_offset"),
            Self::OnchainWalletTransferOutgoing => write!(f, "onchain_wallet_transfer_outgoing"),
            Self::OnchainWalletTransferIncoming => write!(f, "onchain_wallet_transfer_incoming"),
            Self::OnchainPlatformTransferOutgoing => {
                write!(f, "onchain_platform_transfer_outgoing")
            }
            Self::OnchainPlatformTransferIncoming => {
                write!(f, "onchain_platform_transfer_incoming")
            }
            Self::OnchainInternalTransferOutgoing => {
                write!(f, "onchain_internal_transfer_outgoing")
            }
            Self::OnchainInternalTransferIncoming => {
                write!(f, "onchain_internal_transfer_incoming")
            }
            Self::OnchainSwapSource => write!(f, "onchain_swap_source"),
            Self::OnchainSwapTarget => write!(f, "onchain_swap_target"),
            Self::OnchainSwapOffset => write!(f, "onchain_swap_offset"),
            Self::WhopSwapFeeReceived => write!(f, "whop_swap_fee_received"),
            Self::SwapFee => write!(f, "swap_fee"),
            Self::InternalWithdrawal => write!(f, "internal_withdrawal"),
            Self::InternalWithdrawalReversal => write!(f, "internal_withdrawal_reversal"),
            Self::InternalWithdrawalPayable => write!(f, "internal_withdrawal_payable"),
            Self::InternalWithdrawalPayableReversal => {
                write!(f, "internal_withdrawal_payable_reversal")
            }
            Self::InternalWithdrawalFee => write!(f, "internal_withdrawal_fee"),
            Self::InternalWithdrawalFeeReversal => write!(f, "internal_withdrawal_fee_reversal"),
            Self::InternalWithdrawalMarkupFee => write!(f, "internal_withdrawal_markup_fee"),
            Self::InternalWithdrawalMarkupFeeReversal => {
                write!(f, "internal_withdrawal_markup_fee_reversal")
            }
            Self::InternalWithdrawalMarkupFeePayout => {
                write!(f, "internal_withdrawal_markup_fee_payout")
            }
            Self::InternalWithdrawalMarkupFeePayoutReversal => {
                write!(f, "internal_withdrawal_markup_fee_payout_reversal")
            }
            Self::InternalWithdrawalInTransit => write!(f, "internal_withdrawal_in_transit"),
            Self::InternalWithdrawalComplete => write!(f, "internal_withdrawal_complete"),
            Self::InternalWithdrawalInTransitReversal => {
                write!(f, "internal_withdrawal_in_transit_reversal")
            }
            Self::CardLoadTransfer => write!(f, "card_load_transfer"),
            Self::CardLoadDeposit => write!(f, "card_load_deposit"),
            Self::CardUnloadTransfer => write!(f, "card_unload_transfer"),
            Self::CardUnloadDeposit => write!(f, "card_unload_deposit"),
            Self::ExternalCardLoadDeposit => write!(f, "external_card_load_deposit"),
            Self::ExternalCardLoadOffset => write!(f, "external_card_load_offset"),
            Self::CardSpendAuthorization => write!(f, "card_spend_authorization"),
            Self::CardSpendAuthorizationHold => write!(f, "card_spend_authorization_hold"),
            Self::CardSpendCapture => write!(f, "card_spend_capture"),
            Self::CardSpendCaptureOffset => write!(f, "card_spend_capture_offset"),
            Self::CardSpendAuthorizationVoid => write!(f, "card_spend_authorization_void"),
            Self::CardSpendAuthorizationVoidRelease => {
                write!(f, "card_spend_authorization_void_release")
            }
            Self::CardSpendRefund => write!(f, "card_spend_refund"),
            Self::CardSpendRefundOffset => write!(f, "card_spend_refund_offset"),
            Self::Cashback => write!(f, "cashback"),
            Self::CashbackExpense => write!(f, "cashback_expense"),
            Self::CashbackFunding => write!(f, "cashback_funding"),
            Self::CardInterchangeReceivable => write!(f, "card_interchange_receivable"),
            Self::CardInterchange => write!(f, "card_interchange"),
            Self::AccountSettlement => write!(f, "account_settlement"),
            Self::CurrencyConversionOutgoing => write!(f, "currency_conversion_outgoing"),
            Self::CurrencyConversionIncoming => write!(f, "currency_conversion_incoming"),
            Self::BalanceReservation => write!(f, "balance_reservation"),
            Self::BalanceReservationHold => write!(f, "balance_reservation_hold"),
            Self::BalanceReservationReversal => write!(f, "balance_reservation_reversal"),
            Self::BalanceReservationRelease => write!(f, "balance_reservation_release"),
            Self::BadDebtOffset => write!(f, "bad_debt_offset"),
            Self::BadDebtExpense => write!(f, "bad_debt_expense"),
            Self::PlatformCreditsPayment => write!(f, "platform_credits_payment"),
            Self::PlatformCreditsPaymentRefund => write!(f, "platform_credits_payment_refund"),
            Self::PlatformCreditsGranted => write!(f, "platform_credits_granted"),
            Self::LevelsPointsCard => write!(f, "levels_points_card"),
            Self::LevelsPointsAds => write!(f, "levels_points_ads"),
            Self::LevelsPointsTrading => write!(f, "levels_points_trading"),
            Self::LevelsPointsPartner => write!(f, "levels_points_partner"),
            Self::LevelsPointsBurned => write!(f, "levels_points_burned"),
            Self::LevelsPointsIssued => write!(f, "levels_points_issued"),
            Self::LevelsRebate => write!(f, "levels_rebate"),
            Self::MarketingExpense => write!(f, "marketing_expense"),
            Self::PlatformBalanceTransferOutgoing => {
                write!(f, "platform_balance_transfer_outgoing")
            }
            Self::PlatformBalanceTransferIncoming => {
                write!(f, "platform_balance_transfer_incoming")
            }
            Self::InternalBalanceTransferOutgoing => {
                write!(f, "internal_balance_transfer_outgoing")
            }
            Self::InternalBalanceTransferIncoming => {
                write!(f, "internal_balance_transfer_incoming")
            }
            Self::PlatformBalanceTransferFee => write!(f, "platform_balance_transfer_fee"),
            Self::ConnectedAccountClawback => write!(f, "connected_account_clawback"),
            Self::ConnectedAccountNegativeBalance => {
                write!(f, "connected_account_negative_balance")
            }
            Self::AdPublisherPayout => write!(f, "ad_publisher_payout"),
            Self::AdPublisherPayoutReceived => write!(f, "ad_publisher_payout_received"),
            Self::CompanyReferral => write!(f, "company_referral"),
            Self::PlatformAffiliatePayment => write!(f, "platform_affiliate_payment"),
            Self::PlatformAffiliatePaymentReversal => {
                write!(f, "platform_affiliate_payment_reversal")
            }
            Self::OnboardingReward => write!(f, "onboarding_reward"),
            Self::PartnerGrantReward => write!(f, "partner_grant_reward"),
            Self::PaymentRevshare => write!(f, "payment_revshare"),
            Self::PaymentRevsharePayout => write!(f, "payment_revshare_payout"),
            Self::PaymentRevshareReversal => write!(f, "payment_revshare_reversal"),
            Self::PaymentRevshareRefund => write!(f, "payment_revshare_refund"),
            Self::PaymentReferral => write!(f, "payment_referral"),
            Self::PaymentReferralPayable => write!(f, "payment_referral_payable"),
            Self::PaymentReferralReversal => write!(f, "payment_referral_reversal"),
            Self::PaymentReferralRefund => write!(f, "payment_referral_refund"),
            Self::ApplicationFeePayable => write!(f, "application_fee_payable"),
            Self::ApplicationFeePayout => write!(f, "application_fee_payout"),
            Self::PlatformEarning => write!(f, "platform_earning"),
            Self::Airdrop => write!(f, "airdrop"),
            Self::AirdropReversal => write!(f, "airdrop_reversal"),
            Self::AirdropExpense => write!(f, "airdrop_expense"),
            Self::AirdropExpenseReversal => write!(f, "airdrop_expense_reversal"),
            Self::AirdropLinkFunded => write!(f, "airdrop_link_funded"),
            Self::AirdropLinkCanceled => write!(f, "airdrop_link_canceled"),
            Self::AirdropLinkReturned => write!(f, "airdrop_link_returned"),
            Self::AirdropLinkClaimed => write!(f, "airdrop_link_claimed"),
            Self::AirdropLinkRedeemed => write!(f, "airdrop_link_redeemed"),
            Self::AirdropLinkCreated => write!(f, "airdrop_link_created"),
            Self::ResolutionCenterRefund => write!(f, "resolution_center_refund"),
            Self::AdFundingDisbursement => write!(f, "ad_funding_disbursement"),
            Self::AdCampaignBudget => write!(f, "ad_campaign_budget"),
            Self::AdsCardSpread => write!(f, "ads_card_spread"),
            Self::AdSpendCharge => write!(f, "ad_spend_charge"),
            Self::AdNetworkCost => write!(f, "ad_network_cost"),
            Self::AdSpendMargin => write!(f, "ad_spend_margin"),
            Self::AdIncomeExpense => write!(f, "ad_income_expense"),
            Self::AdIncomeReceipt => write!(f, "ad_income_receipt"),
            Self::AdBudgetRelease => write!(f, "ad_budget_release"),
            Self::AdNetworkSettlement => write!(f, "ad_network_settlement"),
            Self::AdAffiliatePayout => write!(f, "ad_affiliate_payout"),
            Self::AdAffiliatePayoutReceived => write!(f, "ad_affiliate_payout_received"),
            Self::AdBalanceFundingReceipt => write!(f, "ad_balance_funding_receipt"),
            Self::MiscReversal => write!(f, "misc_reversal"),
            Self::FxGainLoss => write!(f, "fx_gain_loss"),
            Self::FxMarkup => write!(f, "fx_markup"),
            Self::LegacyCryptoPayment => write!(f, "legacy_crypto_payment"),
            Self::LegacyPayment => write!(f, "legacy_payment"),
            Self::LegacyPaymentRefund => write!(f, "legacy_payment_refund"),
            Self::LicenseSale => write!(f, "license_sale"),
            Self::LicenseSaleCommission => write!(f, "license_sale_commission"),
            Self::LicenseSaleRevenue => write!(f, "license_sale_revenue"),
            Self::SoftwareRentalRevshare => write!(f, "software_rental_revshare"),
            Self::SoftwareRentalTransaction => write!(f, "software_rental_transaction"),
            Self::MiscPurchase => write!(f, "misc_purchase"),
            Self::MiscRefund => write!(f, "misc_refund"),
            Self::ReferralBonus => write!(f, "referral_bonus"),
            Self::PromoReversal => write!(f, "promo_reversal"),
            Self::PaymentReceivableSettlement => write!(f, "payment_receivable_settlement"),
            Self::PlatformEarningSettlement => write!(f, "platform_earning_settlement"),
            Self::PlatformEarningUnreconciledClearing => {
                write!(f, "platform_earning_unreconciled_clearing")
            }
            Self::PaymentReceivableClearing => write!(f, "payment_receivable_clearing"),
            Self::PaymentReceivableReversal => write!(f, "payment_receivable_reversal"),
            Self::PaymentUnreconciledClearing => write!(f, "payment_unreconciled_clearing"),
            Self::PaymentUnreconciledReversal => write!(f, "payment_unreconciled_reversal"),
            Self::SettlementShortfallLoss => write!(f, "settlement_shortfall_loss"),
            Self::FxSettlementGainLoss => write!(f, "fx_settlement_gain_loss"),
            Self::SettlementRoundingVariance => write!(f, "settlement_rounding_variance"),
            Self::PspPayoutSettlement => write!(f, "psp_payout_settlement"),
            Self::PspPayoutDeposit => write!(f, "psp_payout_deposit"),
            Self::PspPayoutReceivable => write!(f, "psp_payout_receivable"),
            Self::PspPayoutReceivableClearing => write!(f, "psp_payout_receivable_clearing"),
            Self::PayoutUnreconciledClearing => write!(f, "payout_unreconciled_clearing"),
            Self::PspCurrencyConversionOutgoing => write!(f, "psp_currency_conversion_outgoing"),
            Self::PspCurrencyConversionIncoming => write!(f, "psp_currency_conversion_incoming"),
            Self::PspCurrencyConversionReceivable => {
                write!(f, "psp_currency_conversion_receivable")
            }
            Self::PspCurrencyConversionReceivableClearing => {
                write!(f, "psp_currency_conversion_receivable_clearing")
            }
            Self::PspFxAdjustment => write!(f, "psp_fx_adjustment"),
            Self::RefundSettlement => write!(f, "refund_settlement"),
            Self::RefundSettlementReversal => write!(f, "refund_settlement_reversal"),
            Self::RefundPayableClearing => write!(f, "refund_payable_clearing"),
            Self::RefundPayableReversal => write!(f, "refund_payable_reversal"),
            Self::RefundUnreconciledClearing => write!(f, "refund_unreconciled_clearing"),
            Self::RefundUnreconciledReversal => write!(f, "refund_unreconciled_reversal"),
            Self::PspAccountDebit => write!(f, "psp_account_debit"),
            Self::PaymentRefundFee => write!(f, "payment_refund_fee"),
            Self::DisputeSettlement => write!(f, "dispute_settlement"),
            Self::DisputeSettlementReversal => write!(f, "dispute_settlement_reversal"),
            Self::DisputePayableClearing => write!(f, "dispute_payable_clearing"),
            Self::DisputePayableReversal => write!(f, "dispute_payable_reversal"),
            Self::DisputeUnreconciledClearing => write!(f, "dispute_unreconciled_clearing"),
            Self::DisputeUnreconciledReversal => write!(f, "dispute_unreconciled_reversal"),
            Self::PspReserveHold => write!(f, "psp_reserve_hold"),
            Self::PspReserveRelease => write!(f, "psp_reserve_release"),
            Self::PspDisputeFee => write!(f, "psp_dispute_fee"),
            Self::PspDisputeAlertFee => write!(f, "psp_dispute_alert_fee"),
            Self::PspRefundFee => write!(f, "psp_refund_fee"),
            Self::PspAccountUpdaterFee => write!(f, "psp_account_updater_fee"),
            Self::PspAdjustedProcessingFee => write!(f, "psp_adjusted_processing_fee"),
            Self::PspAuthenticationFee => write!(f, "psp_authentication_fee"),
            Self::PspCommissionFee => write!(f, "psp_commission_fee"),
            Self::PspFixedFee => write!(f, "psp_fixed_fee"),
            Self::PspGatewayFee => write!(f, "psp_gateway_fee"),
            Self::PspInterchangeFee => write!(f, "psp_interchange_fee"),
            Self::PspInvoiceTaxFee => write!(f, "psp_invoice_tax_fee"),
            Self::PspMarkupFee => write!(f, "psp_markup_fee"),
            Self::PspNetworkTokenFee => write!(f, "psp_network_token_fee"),
            Self::PspOptimizationFee => write!(f, "psp_optimization_fee"),
            Self::PspPaymentMethodFee => write!(f, "psp_payment_method_fee"),
            Self::PspPayoutFee => write!(f, "psp_payout_fee"),
            Self::PspProcessingFee => write!(f, "psp_processing_fee"),
            Self::PspRiskFee => write!(f, "psp_risk_fee"),
            Self::PspSchemeFee => write!(f, "psp_scheme_fee"),
            Self::PspVariableFee => write!(f, "psp_variable_fee"),
            Self::PspBillingFee => write!(f, "psp_billing_fee"),
            Self::PspTaxServiceFee => write!(f, "psp_tax_service_fee"),
            Self::PspConnectFee => write!(f, "psp_connect_fee"),
            Self::ExternalAccountConnectionFee => write!(f, "external_account_connection_fee"),
            Self::PspExternalAccountConnectionFee => {
                write!(f, "psp_external_account_connection_fee")
            }
            Self::PspInvoicingFee => write!(f, "psp_invoicing_fee"),
            Self::PspTerminalFee => write!(f, "psp_terminal_fee"),
            Self::TreasuryFee => write!(f, "treasury_fee"),
            Self::PspTreasuryFee => write!(f, "psp_treasury_fee"),
            Self::AcceleratedSettlementFee => write!(f, "accelerated_settlement_fee"),
            Self::PspAcceleratedSettlementFee => write!(f, "psp_accelerated_settlement_fee"),
            Self::PspReceivablePooled => write!(f, "psp_receivable_pooled"),
            Self::PspPoolSettlement => write!(f, "psp_pool_settlement"),
            Self::PspPoolClearing => write!(f, "psp_pool_clearing"),
            Self::PspPoolRefundSettlement => write!(f, "psp_pool_refund_settlement"),
            Self::PspPoolRefundClearing => write!(f, "psp_pool_refund_clearing"),
            Self::PspCorrection => write!(f, "psp_correction"),
            Self::PspPayinSettlement => write!(f, "psp_payin_settlement"),
            Self::PspPayinClearing => write!(f, "psp_payin_clearing"),
            Self::PspBankPullSettlement => write!(f, "psp_bank_pull_settlement"),
            Self::PspBankPullClearing => write!(f, "psp_bank_pull_clearing"),
            Self::PspPoolDisputeSettlement => write!(f, "psp_pool_dispute_settlement"),
            Self::PspPayoutConsolidation => write!(f, "psp_payout_consolidation"),
            Self::PspWithholdingTax => write!(f, "psp_withholding_tax"),
            Self::PspPayoutSettlementReversal => write!(f, "psp_payout_settlement_reversal"),
            Self::PspTransferSettlement => write!(f, "psp_transfer_settlement"),
            Self::PspTransferSettlementReversal => write!(f, "psp_transfer_settlement_reversal"),
            Self::PspCollectionSettlement => write!(f, "psp_collection_settlement"),
            Self::PayoutSubsidy => write!(f, "payout_subsidy"),
            Self::WithdrawalTopupAdjustment => write!(f, "withdrawal_topup_adjustment"),
            Self::WithdrawalPayableClearing => write!(f, "withdrawal_payable_clearing"),
            Self::WithdrawalPayableClearingReversal => {
                write!(f, "withdrawal_payable_clearing_reversal")
            }
            Self::IdentityVerificationFee => write!(f, "identity_verification_fee"),
            Self::PspIdentityVerificationFee => write!(f, "psp_identity_verification_fee"),
            Self::TaxFilingFee => write!(f, "tax_filing_fee"),
            Self::PspTaxFilingFee => write!(f, "psp_tax_filing_fee"),
            Self::PspServiceFee => write!(f, "psp_service_fee"),
            Self::RecipientWalletLoad => write!(f, "recipient_wallet_load"),
            Self::PspRecipientWalletLoad => write!(f, "psp_recipient_wallet_load"),
            Self::DisputeManagementFee => write!(f, "dispute_management_fee"),
            Self::PspDisputeManagementFee => write!(f, "psp_dispute_management_fee"),
            Self::PspClawbackSettlement => write!(f, "psp_clawback_settlement"),
            Self::PspClawbackSettlementReversal => write!(f, "psp_clawback_settlement_reversal"),
            Self::ClawbackReceivableSettlement => write!(f, "clawback_receivable_settlement"),
            Self::ClawbackReceivableSettlementReversal => {
                write!(f, "clawback_receivable_settlement_reversal")
            }
            Self::ClawbackFee => write!(f, "clawback_fee"),
            Self::PspClawbackFee => write!(f, "psp_clawback_fee"),
            Self::Deposit => write!(f, "deposit"),
            Self::WalletTransferIncoming => write!(f, "wallet_transfer_incoming"),
            Self::WalletTransferOutgoing => write!(f, "wallet_transfer_outgoing"),
            Self::SwapSource => write!(f, "swap_source"),
            Self::SwapTarget => write!(f, "swap_target"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
