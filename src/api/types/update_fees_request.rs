pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateFeesRequest {
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_deposit: Option<UpdateFeesRequestBankDeposit>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing: Option<UpdateFeesRequestBilling>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer: Option<UpdateFeesRequestBuyer>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_processing: Option<UpdateFeesRequestCardProcessing>,
    /// This platform's default markups for every account connected to it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_markups: Option<UpdateFeesRequestChildMarkups>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_border: Option<UpdateFeesRequestCrossBorder>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute: Option<UpdateFeesRequestDispute>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_alert: Option<UpdateFeesRequestDisputeAlert>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_alert_cdrn: Option<UpdateFeesRequestDisputeAlertCdrn>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_alert_ethoca: Option<UpdateFeesRequestDisputeAlertEthoca>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_alert_rdr: Option<UpdateFeesRequestDisputeAlertRdr>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_representment: Option<UpdateFeesRequestDisputeRepresentment>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foreign_exchange: Option<UpdateFeesRequestForeignExchange>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_screening: Option<UpdateFeesRequestFraudScreening>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_risk: Option<UpdateFeesRequestHighRisk>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace: Option<UpdateFeesRequestMarketplace>,
    /// Markups on this connected account, set by the platform it is connected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub markups: Option<UpdateFeesRequestMarkups>,
    /// Why the fees are changing, recorded with the change. Required when a Whop Verified Partner edits the fee schedule; ignored for markups.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orchestration: Option<UpdateFeesRequestOrchestration>,
    /// Changes to non-card payment method fees, keyed by payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<HashMap<String, UpdateFeesRequestPaymentMethodsValue>>,
    /// Changes to withdrawal fees, keyed by payout method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<HashMap<String, UpdateFeesRequestPayoutsValue>>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_auto_topup: Option<UpdateFeesRequestPendingAutoTopup>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_processing: Option<UpdateFeesRequestPlatformProcessing>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pool_payout: Option<UpdateFeesRequestPoolPayout>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revshare: Option<UpdateFeesRequestRevshare>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_calculation: Option<UpdateFeesRequestTaxCalculation>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_service: Option<UpdateFeesRequestTaxService>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_ds: Option<UpdateFeesRequestThreeDs>,
    /// The fields of a fee the caller may change. Only the keys sent are replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfers: Option<UpdateFeesRequestTransfers>,
}

impl UpdateFeesRequest {
    pub fn builder() -> UpdateFeesRequestBuilder {
        <UpdateFeesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFeesRequestBuilder {
    bank_deposit: Option<UpdateFeesRequestBankDeposit>,
    billing: Option<UpdateFeesRequestBilling>,
    buyer: Option<UpdateFeesRequestBuyer>,
    card_processing: Option<UpdateFeesRequestCardProcessing>,
    child_markups: Option<UpdateFeesRequestChildMarkups>,
    cross_border: Option<UpdateFeesRequestCrossBorder>,
    dispute: Option<UpdateFeesRequestDispute>,
    dispute_alert: Option<UpdateFeesRequestDisputeAlert>,
    dispute_alert_cdrn: Option<UpdateFeesRequestDisputeAlertCdrn>,
    dispute_alert_ethoca: Option<UpdateFeesRequestDisputeAlertEthoca>,
    dispute_alert_rdr: Option<UpdateFeesRequestDisputeAlertRdr>,
    dispute_representment: Option<UpdateFeesRequestDisputeRepresentment>,
    foreign_exchange: Option<UpdateFeesRequestForeignExchange>,
    fraud_screening: Option<UpdateFeesRequestFraudScreening>,
    high_risk: Option<UpdateFeesRequestHighRisk>,
    marketplace: Option<UpdateFeesRequestMarketplace>,
    markups: Option<UpdateFeesRequestMarkups>,
    notes: Option<String>,
    orchestration: Option<UpdateFeesRequestOrchestration>,
    payment_methods: Option<HashMap<String, UpdateFeesRequestPaymentMethodsValue>>,
    payouts: Option<HashMap<String, UpdateFeesRequestPayoutsValue>>,
    pending_auto_topup: Option<UpdateFeesRequestPendingAutoTopup>,
    platform_processing: Option<UpdateFeesRequestPlatformProcessing>,
    pool_payout: Option<UpdateFeesRequestPoolPayout>,
    revshare: Option<UpdateFeesRequestRevshare>,
    tax_calculation: Option<UpdateFeesRequestTaxCalculation>,
    tax_service: Option<UpdateFeesRequestTaxService>,
    three_ds: Option<UpdateFeesRequestThreeDs>,
    transfers: Option<UpdateFeesRequestTransfers>,
}

impl UpdateFeesRequestBuilder {
    pub fn bank_deposit(mut self, value: UpdateFeesRequestBankDeposit) -> Self {
        self.bank_deposit = Some(value);
        self
    }

    pub fn billing(mut self, value: UpdateFeesRequestBilling) -> Self {
        self.billing = Some(value);
        self
    }

    pub fn buyer(mut self, value: UpdateFeesRequestBuyer) -> Self {
        self.buyer = Some(value);
        self
    }

    pub fn card_processing(mut self, value: UpdateFeesRequestCardProcessing) -> Self {
        self.card_processing = Some(value);
        self
    }

    pub fn child_markups(mut self, value: UpdateFeesRequestChildMarkups) -> Self {
        self.child_markups = Some(value);
        self
    }

    pub fn cross_border(mut self, value: UpdateFeesRequestCrossBorder) -> Self {
        self.cross_border = Some(value);
        self
    }

    pub fn dispute(mut self, value: UpdateFeesRequestDispute) -> Self {
        self.dispute = Some(value);
        self
    }

    pub fn dispute_alert(mut self, value: UpdateFeesRequestDisputeAlert) -> Self {
        self.dispute_alert = Some(value);
        self
    }

    pub fn dispute_alert_cdrn(mut self, value: UpdateFeesRequestDisputeAlertCdrn) -> Self {
        self.dispute_alert_cdrn = Some(value);
        self
    }

    pub fn dispute_alert_ethoca(mut self, value: UpdateFeesRequestDisputeAlertEthoca) -> Self {
        self.dispute_alert_ethoca = Some(value);
        self
    }

    pub fn dispute_alert_rdr(mut self, value: UpdateFeesRequestDisputeAlertRdr) -> Self {
        self.dispute_alert_rdr = Some(value);
        self
    }

    pub fn dispute_representment(mut self, value: UpdateFeesRequestDisputeRepresentment) -> Self {
        self.dispute_representment = Some(value);
        self
    }

    pub fn foreign_exchange(mut self, value: UpdateFeesRequestForeignExchange) -> Self {
        self.foreign_exchange = Some(value);
        self
    }

    pub fn fraud_screening(mut self, value: UpdateFeesRequestFraudScreening) -> Self {
        self.fraud_screening = Some(value);
        self
    }

    pub fn high_risk(mut self, value: UpdateFeesRequestHighRisk) -> Self {
        self.high_risk = Some(value);
        self
    }

    pub fn marketplace(mut self, value: UpdateFeesRequestMarketplace) -> Self {
        self.marketplace = Some(value);
        self
    }

    pub fn markups(mut self, value: UpdateFeesRequestMarkups) -> Self {
        self.markups = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn orchestration(mut self, value: UpdateFeesRequestOrchestration) -> Self {
        self.orchestration = Some(value);
        self
    }

    pub fn payment_methods(
        mut self,
        value: HashMap<String, UpdateFeesRequestPaymentMethodsValue>,
    ) -> Self {
        self.payment_methods = Some(value);
        self
    }

    pub fn payouts(mut self, value: HashMap<String, UpdateFeesRequestPayoutsValue>) -> Self {
        self.payouts = Some(value);
        self
    }

    pub fn pending_auto_topup(mut self, value: UpdateFeesRequestPendingAutoTopup) -> Self {
        self.pending_auto_topup = Some(value);
        self
    }

    pub fn platform_processing(mut self, value: UpdateFeesRequestPlatformProcessing) -> Self {
        self.platform_processing = Some(value);
        self
    }

    pub fn pool_payout(mut self, value: UpdateFeesRequestPoolPayout) -> Self {
        self.pool_payout = Some(value);
        self
    }

    pub fn revshare(mut self, value: UpdateFeesRequestRevshare) -> Self {
        self.revshare = Some(value);
        self
    }

    pub fn tax_calculation(mut self, value: UpdateFeesRequestTaxCalculation) -> Self {
        self.tax_calculation = Some(value);
        self
    }

    pub fn tax_service(mut self, value: UpdateFeesRequestTaxService) -> Self {
        self.tax_service = Some(value);
        self
    }

    pub fn three_ds(mut self, value: UpdateFeesRequestThreeDs) -> Self {
        self.three_ds = Some(value);
        self
    }

    pub fn transfers(mut self, value: UpdateFeesRequestTransfers) -> Self {
        self.transfers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFeesRequest`].
    pub fn build(self) -> Result<UpdateFeesRequest, BuildError> {
        Ok(UpdateFeesRequest {
            bank_deposit: self.bank_deposit,
            billing: self.billing,
            buyer: self.buyer,
            card_processing: self.card_processing,
            child_markups: self.child_markups,
            cross_border: self.cross_border,
            dispute: self.dispute,
            dispute_alert: self.dispute_alert,
            dispute_alert_cdrn: self.dispute_alert_cdrn,
            dispute_alert_ethoca: self.dispute_alert_ethoca,
            dispute_alert_rdr: self.dispute_alert_rdr,
            dispute_representment: self.dispute_representment,
            foreign_exchange: self.foreign_exchange,
            fraud_screening: self.fraud_screening,
            high_risk: self.high_risk,
            marketplace: self.marketplace,
            markups: self.markups,
            notes: self.notes,
            orchestration: self.orchestration,
            payment_methods: self.payment_methods,
            payouts: self.payouts,
            pending_auto_topup: self.pending_auto_topup,
            platform_processing: self.platform_processing,
            pool_payout: self.pool_payout,
            revshare: self.revshare,
            tax_calculation: self.tax_calculation,
            tax_service: self.tax_service,
            three_ds: self.three_ds,
            transfers: self.transfers,
        })
    }
}
