pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dispute {
    /// The account the dispute was filed against, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The disputed amount, in whole units of `currency`.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The customer who filed the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer: Option<DisputeBuyer>,
    /// When the dispute was opened, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Three-letter ISO currency code of the disputed amount.
    #[serde(default)]
    pub currency: String,
    /// The evidence packet sent to the processor to contest the dispute.
    #[serde(default)]
    pub evidence: DisputeEvidence,
    /// The deadline to submit evidence, as an ISO 8601 timestamp. Whop reserves the last 24 hours before the processor's own cutoff to forward the submission.
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
    /// The AI-generated representment document filed with the processor on the seller's behalf, once ready. Null until generation completes, and for disputes not using Whop Dispute Fighter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_response_attachment: Option<DisputeAttachment>,
    /// Dispute ID, prefixed `dspt_`.
    #[serde(default)]
    pub id: String,
    /// Whether this is a pre-dispute inquiry rather than a formal chargeback. Inquiries follow the same lifecycle but move no funds unless one escalates.
    #[serde(default)]
    pub inquiry: bool,
    #[serde(default)]
    pub issuer_comments: Vec<DisputeIssuerComment>,
    /// The payment being disputed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<DisputePayment>,
    /// The plan the disputed payment was made on, prefixed `plan_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// The product the disputed payment was for, prefixed `prod_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Whether Visa Rapid Dispute Resolution settled this automatically. These refund the customer without an evidence round.
    #[serde(default)]
    pub rapid_dispute_resolution: bool,
    /// Why the customer says they are disputing, normalized across card networks. `other` covers a code Whop has not categorized yet — read `reason_code` for the raw value.
    pub reason: DisputeReason,
    /// The raw card-network or processor reason code, such as `10.4`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    /// Where the dispute stands. `needs_response` is awaiting evidence, `under_review` is with the processor, `won` returned the funds to the seller, `lost` returned them to the customer, and `closed` ended without a ruling. A dispute past its `evidence_due_at` reports `under_review` — the window to respond has closed.
    pub status: DisputeStatus,
    /// When the dispute was last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl Dispute {
    pub fn builder() -> DisputeBuilder {
        <DisputeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeBuilder {
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
    generated_response_attachment: Option<DisputeAttachment>,
    id: Option<String>,
    inquiry: Option<bool>,
    issuer_comments: Option<Vec<DisputeIssuerComment>>,
    payment: Option<DisputePayment>,
    plan_id: Option<String>,
    product_id: Option<String>,
    rapid_dispute_resolution: Option<bool>,
    reason: Option<DisputeReason>,
    reason_code: Option<String>,
    status: Option<DisputeStatus>,
    updated_at: Option<String>,
}

impl DisputeBuilder {
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

    pub fn generated_response_attachment(mut self, value: DisputeAttachment) -> Self {
        self.generated_response_attachment = Some(value);
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

    pub fn payment(mut self, value: DisputePayment) -> Self {
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

    pub fn rapid_dispute_resolution(mut self, value: bool) -> Self {
        self.rapid_dispute_resolution = Some(value);
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

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Dispute`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](DisputeBuilder::amount)
    /// - [`created_at`](DisputeBuilder::created_at)
    /// - [`currency`](DisputeBuilder::currency)
    /// - [`evidence`](DisputeBuilder::evidence)
    /// - [`evidence_editable`](DisputeBuilder::evidence_editable)
    /// - [`id`](DisputeBuilder::id)
    /// - [`inquiry`](DisputeBuilder::inquiry)
    /// - [`issuer_comments`](DisputeBuilder::issuer_comments)
    /// - [`rapid_dispute_resolution`](DisputeBuilder::rapid_dispute_resolution)
    /// - [`reason`](DisputeBuilder::reason)
    /// - [`status`](DisputeBuilder::status)
    /// - [`updated_at`](DisputeBuilder::updated_at)
    pub fn build(self) -> Result<Dispute, BuildError> {
        Ok(Dispute {
            account_id: self.account_id,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            buyer: self.buyer,
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
            generated_response_attachment: self.generated_response_attachment,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            inquiry: self
                .inquiry
                .ok_or_else(|| BuildError::missing_field("inquiry"))?,
            issuer_comments: self
                .issuer_comments
                .ok_or_else(|| BuildError::missing_field("issuer_comments"))?,
            payment: self.payment,
            plan_id: self.plan_id,
            product_id: self.product_id,
            rapid_dispute_resolution: self
                .rapid_dispute_resolution
                .ok_or_else(|| BuildError::missing_field("rapid_dispute_resolution"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            reason_code: self.reason_code,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
