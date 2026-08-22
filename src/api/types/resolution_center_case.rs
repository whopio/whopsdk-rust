pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResolutionCenterCase {
    /// The account the case was filed against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<AccountSummary>,
    /// The amount in question, in whole units of `currency`.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    #[serde(default)]
    pub available_actions: Vec<ResolutionCenterCaseAvailableActionsItem>,
    /// The customer who opened the case.
    #[serde(default)]
    pub buyer: ResolutionBuyer,
    /// When the case was opened, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Three-letter ISO currency code of the amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Whether the customer has appealed a decision on this case.
    #[serde(default)]
    pub customer_appealed: bool,
    /// Whether Whop is involved — either reviewing the case, or waiting on the side named by `status` for something it asked for while reviewing.
    #[serde(default)]
    pub escalated: bool,
    /// Resolution center case ID, prefixed `reso_`.
    #[serde(default)]
    pub id: String,
    /// Who prevailed on the claim. `null` until the case closes. Read `refund` for whether any money actually moved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<ResolutionCenterCaseOutcome>,
    /// The payment the case was opened against.
    #[serde(default)]
    pub payment: ResolutionPayment,
    /// The plan the payment was made on, prefixed `plan_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// The product the payment was for, prefixed `prod_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// What the customer says went wrong. Shares the `/disputes` vocabulary, so a case that later becomes a chargeback reports the same complaint.
    pub reason: ResolutionCenterCaseReason,
    /// Whether money moved and off whose balance: `none`, `merchant`, or `platform` (Whop refunded the customer and the merchant kept the funds). Independent of `outcome` — a case the merchant won can still carry a platform refund. `null` while the case is open, and on older closed cases that predate this being recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<ResolutionCenterCaseRefund>,
    /// When the next response is due, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_due_at: Option<String>,
    /// Who the case is waiting on. `awaiting_merchant` and `awaiting_customer` name the side that owes a response, `under_review` means Whop is deciding, and `closed` means it is settled — read `outcome` for how.
    pub status: ResolutionCenterCaseStatus,
    /// When the case was last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl ResolutionCenterCase {
    pub fn builder() -> ResolutionCenterCaseBuilder {
        <ResolutionCenterCaseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionCenterCaseBuilder {
    account: Option<AccountSummary>,
    amount: Option<f64>,
    available_actions: Option<Vec<ResolutionCenterCaseAvailableActionsItem>>,
    buyer: Option<ResolutionBuyer>,
    created_at: Option<String>,
    currency: Option<String>,
    customer_appealed: Option<bool>,
    escalated: Option<bool>,
    id: Option<String>,
    outcome: Option<ResolutionCenterCaseOutcome>,
    payment: Option<ResolutionPayment>,
    plan_id: Option<String>,
    product_id: Option<String>,
    reason: Option<ResolutionCenterCaseReason>,
    refund: Option<ResolutionCenterCaseRefund>,
    response_due_at: Option<String>,
    status: Option<ResolutionCenterCaseStatus>,
    updated_at: Option<String>,
}

impl ResolutionCenterCaseBuilder {
    pub fn account(mut self, value: AccountSummary) -> Self {
        self.account = Some(value);
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn available_actions(
        mut self,
        value: Vec<ResolutionCenterCaseAvailableActionsItem>,
    ) -> Self {
        self.available_actions = Some(value);
        self
    }

    pub fn buyer(mut self, value: ResolutionBuyer) -> Self {
        self.buyer = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn customer_appealed(mut self, value: bool) -> Self {
        self.customer_appealed = Some(value);
        self
    }

    pub fn escalated(mut self, value: bool) -> Self {
        self.escalated = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn outcome(mut self, value: ResolutionCenterCaseOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn payment(mut self, value: ResolutionPayment) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: ResolutionCenterCaseReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn refund(mut self, value: ResolutionCenterCaseRefund) -> Self {
        self.refund = Some(value);
        self
    }

    pub fn response_due_at(mut self, value: impl Into<String>) -> Self {
        self.response_due_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: ResolutionCenterCaseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResolutionCenterCase`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](ResolutionCenterCaseBuilder::amount)
    /// - [`available_actions`](ResolutionCenterCaseBuilder::available_actions)
    /// - [`buyer`](ResolutionCenterCaseBuilder::buyer)
    /// - [`created_at`](ResolutionCenterCaseBuilder::created_at)
    /// - [`customer_appealed`](ResolutionCenterCaseBuilder::customer_appealed)
    /// - [`escalated`](ResolutionCenterCaseBuilder::escalated)
    /// - [`id`](ResolutionCenterCaseBuilder::id)
    /// - [`payment`](ResolutionCenterCaseBuilder::payment)
    /// - [`reason`](ResolutionCenterCaseBuilder::reason)
    /// - [`status`](ResolutionCenterCaseBuilder::status)
    /// - [`updated_at`](ResolutionCenterCaseBuilder::updated_at)
    pub fn build(self) -> Result<ResolutionCenterCase, BuildError> {
        Ok(ResolutionCenterCase {
            account: self.account,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            available_actions: self
                .available_actions
                .ok_or_else(|| BuildError::missing_field("available_actions"))?,
            buyer: self
                .buyer
                .ok_or_else(|| BuildError::missing_field("buyer"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self.currency,
            customer_appealed: self
                .customer_appealed
                .ok_or_else(|| BuildError::missing_field("customer_appealed"))?,
            escalated: self
                .escalated
                .ok_or_else(|| BuildError::missing_field("escalated"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            outcome: self.outcome,
            payment: self
                .payment
                .ok_or_else(|| BuildError::missing_field("payment"))?,
            plan_id: self.plan_id,
            product_id: self.product_id,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            refund: self.refund,
            response_due_at: self.response_due_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
