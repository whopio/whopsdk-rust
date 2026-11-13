pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dispute {
    /// Whether the dispute can be conceded to the customer with `POST /disputes/{id}/accept`. True only for a chargeback still awaiting a response, before `evidence_due_at`, whose payment processor supports accepting disputes. Inquiries are never acceptable.
    #[serde(default)]
    pub acceptable: bool,
    /// The account the dispute was filed against, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The disputed amount, in whole units of `currency`.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The customer who filed the dispute.
    #[serde(default)]
    pub buyer: DisputeBuyer,
    /// When the dispute was opened, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Three-letter ISO currency code of the disputed amount.
    #[serde(default)]
    pub currency: String,
    /// The evidence packet sent to the processor to contest the dispute.
    #[serde(default)]
    pub evidence: DisputeEvidence,
    /// The deadline to submit evidence, as an ISO 8601 timestamp. `null` when the network already auto-resolved the dispute (Visa RDR) with no evidence round, or when the processor hasn't reported a deadline for this dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_due_at: Option<String>,
    /// Whether `evidence` can still be changed and submitted.
    #[serde(default)]
    pub evidence_editable: bool,
    /// Why evidence can no longer be edited. `null` while `evidence_editable` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_locked_reason: Option<DisputeEvidenceLockedReason>,
    /// When the evidence was submitted to the processor, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_submitted_at: Option<String>,
    /// The dispute fee charged to the account, in the currency it was collected in. `null` when `fee_charged` is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<Money>,
    /// Whether Whop charged the account a dispute fee for this one. `false` when the payment was fully refunded before the dispute was opened, for a Visa RDR resolution, and for an inquiry.
    #[serde(default)]
    pub fee_charged: bool,
    /// Dispute ID, prefixed `dspt_`.
    #[serde(default)]
    pub id: String,
    /// Whether this is a pre-dispute inquiry rather than a formal chargeback. Inquiries follow the same lifecycle but move no funds unless one escalates.
    #[serde(default)]
    pub inquiry: bool,
    #[serde(default)]
    pub issuer_comments: Vec<DisputeIssuerComment>,
    #[serde(default)]
    pub line_items: Vec<ReceiptLineItem>,
    /// The payment being disputed.
    #[serde(default)]
    pub payment: DisputePayment,
    /// The product the disputed payment was for, prefixed `prod_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Why the customer says they are disputing, normalized across processors and card networks. `other` covers a processor reason Whop has not categorized yet.
    pub reason: DisputeReason,
    /// The raw card-network or processor reason code, such as `10.4`. Informational only — `reason` is not derived from it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    /// `needs_response` awaits evidence (after a processor refused a filing, `submission_failure_reason` says why); `generating_response` has evidence with the processor, not yet accepted; `under_review` is with the processor; `won`/`lost` are the rulings; `prevented` settles like `lost`. `warning_` statuses are the inquiry tiers. Past `evidence_due_at` an open one reports `under_review`.
    pub status: DisputeStatus,
    /// Why the payment processor refused the last filing, in plain English, while the dispute is back in `needs_response`. Fix what it names and submit again. Only on retrieve: `dispute.updated` reports the move back to `needs_response`, then read this here.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_failure_reason: Option<String>,
    /// When the dispute was last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// The variant the disputed payment was made on, prefixed `plan_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
}

impl Dispute {
    pub fn builder() -> DisputeBuilder {
        <DisputeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeBuilder {
    acceptable: Option<bool>,
    account_id: Option<String>,
    amount: Option<f64>,
    buyer: Option<DisputeBuyer>,
    created_at: Option<String>,
    currency: Option<String>,
    evidence: Option<DisputeEvidence>,
    evidence_due_at: Option<String>,
    evidence_editable: Option<bool>,
    evidence_locked_reason: Option<DisputeEvidenceLockedReason>,
    evidence_submitted_at: Option<String>,
    fee: Option<Money>,
    fee_charged: Option<bool>,
    id: Option<String>,
    inquiry: Option<bool>,
    issuer_comments: Option<Vec<DisputeIssuerComment>>,
    line_items: Option<Vec<ReceiptLineItem>>,
    payment: Option<DisputePayment>,
    product_id: Option<String>,
    reason: Option<DisputeReason>,
    reason_code: Option<String>,
    status: Option<DisputeStatus>,
    submission_failure_reason: Option<String>,
    updated_at: Option<String>,
    variant_id: Option<String>,
}

impl DisputeBuilder {
    pub fn acceptable(mut self, value: bool) -> Self {
        self.acceptable = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn buyer(mut self, value: DisputeBuyer) -> Self {
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

    pub fn evidence(mut self, value: DisputeEvidence) -> Self {
        self.evidence = Some(value);
        self
    }

    pub fn evidence_due_at(mut self, value: impl Into<String>) -> Self {
        self.evidence_due_at = Some(value.into());
        self
    }

    pub fn evidence_editable(mut self, value: bool) -> Self {
        self.evidence_editable = Some(value);
        self
    }

    pub fn evidence_locked_reason(mut self, value: DisputeEvidenceLockedReason) -> Self {
        self.evidence_locked_reason = Some(value);
        self
    }

    pub fn evidence_submitted_at(mut self, value: impl Into<String>) -> Self {
        self.evidence_submitted_at = Some(value.into());
        self
    }

    pub fn fee(mut self, value: Money) -> Self {
        self.fee = Some(value);
        self
    }

    pub fn fee_charged(mut self, value: bool) -> Self {
        self.fee_charged = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn inquiry(mut self, value: bool) -> Self {
        self.inquiry = Some(value);
        self
    }

    pub fn issuer_comments(mut self, value: Vec<DisputeIssuerComment>) -> Self {
        self.issuer_comments = Some(value);
        self
    }

    pub fn line_items(mut self, value: Vec<ReceiptLineItem>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn payment(mut self, value: DisputePayment) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: DisputeReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn reason_code(mut self, value: impl Into<String>) -> Self {
        self.reason_code = Some(value.into());
        self
    }

    pub fn status(mut self, value: DisputeStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn submission_failure_reason(mut self, value: impl Into<String>) -> Self {
        self.submission_failure_reason = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Dispute`].
    /// This method will fail if any of the following fields are not set:
    /// - [`acceptable`](DisputeBuilder::acceptable)
    /// - [`amount`](DisputeBuilder::amount)
    /// - [`buyer`](DisputeBuilder::buyer)
    /// - [`created_at`](DisputeBuilder::created_at)
    /// - [`currency`](DisputeBuilder::currency)
    /// - [`evidence`](DisputeBuilder::evidence)
    /// - [`evidence_editable`](DisputeBuilder::evidence_editable)
    /// - [`fee_charged`](DisputeBuilder::fee_charged)
    /// - [`id`](DisputeBuilder::id)
    /// - [`inquiry`](DisputeBuilder::inquiry)
    /// - [`issuer_comments`](DisputeBuilder::issuer_comments)
    /// - [`line_items`](DisputeBuilder::line_items)
    /// - [`payment`](DisputeBuilder::payment)
    /// - [`reason`](DisputeBuilder::reason)
    /// - [`status`](DisputeBuilder::status)
    /// - [`updated_at`](DisputeBuilder::updated_at)
    pub fn build(self) -> Result<Dispute, BuildError> {
        Ok(Dispute {
            acceptable: self
                .acceptable
                .ok_or_else(|| BuildError::missing_field("acceptable"))?,
            account_id: self.account_id,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            buyer: self
                .buyer
                .ok_or_else(|| BuildError::missing_field("buyer"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            evidence: self
                .evidence
                .ok_or_else(|| BuildError::missing_field("evidence"))?,
            evidence_due_at: self.evidence_due_at,
            evidence_editable: self
                .evidence_editable
                .ok_or_else(|| BuildError::missing_field("evidence_editable"))?,
            evidence_locked_reason: self.evidence_locked_reason,
            evidence_submitted_at: self.evidence_submitted_at,
            fee: self.fee,
            fee_charged: self
                .fee_charged
                .ok_or_else(|| BuildError::missing_field("fee_charged"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            inquiry: self
                .inquiry
                .ok_or_else(|| BuildError::missing_field("inquiry"))?,
            issuer_comments: self
                .issuer_comments
                .ok_or_else(|| BuildError::missing_field("issuer_comments"))?,
            line_items: self
                .line_items
                .ok_or_else(|| BuildError::missing_field("line_items"))?,
            payment: self
                .payment
                .ok_or_else(|| BuildError::missing_field("payment"))?,
            product_id: self.product_id,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            reason_code: self.reason_code,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            submission_failure_reason: self.submission_failure_reason,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            variant_id: self.variant_id,
        })
    }
}
