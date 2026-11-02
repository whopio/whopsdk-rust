pub use crate::prelude::*;

/// A dispute is a chargeback or payment challenge filed against a company, including evidence and response status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisputeLegacy {
    /// A log of IP-based access activity for the customer on Whop, submitted as evidence in the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_activity_log: Option<String>,
    /// The disputed amount in the specified currency, formatted as a decimal.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The customer's billing address from their payment details, submitted as evidence in the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<String>,
    /// The cancellation policy document uploaded as dispute evidence. Null if no cancellation policy has been provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_attachment: Option<DisputeLegacyCancellationPolicyAttachment>,
    /// A text disclosure describing the company's cancellation policy, submitted as dispute evidence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_disclosure: Option<String>,
    /// The company that the dispute was filed against.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<DisputeLegacyCompany>,
    /// The datetime the dispute was created.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_at: Option<DateTime<FixedOffset>>,
    /// The three-letter ISO currency code for the disputed amount.
    pub currency: Currencies,
    /// Evidence of customer communication or product usage, uploaded as a dispute attachment. Null if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_communication_attachment: Option<DisputeLegacyCustomerCommunicationAttachment>,
    /// The customer's email address from their payment details, included in the evidence packet sent to the payment processor. Editable before submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,
    /// The customer's full name from their payment details, included in the evidence packet sent to the payment processor. Editable before submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// Whether the dispute evidence can still be edited and submitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    /// The unique identifier for the dispute.
    #[serde(default)]
    pub id: String,
    /// The deadline by which dispute evidence must be submitted. Null if no response deadline is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub needs_response_by: Option<DateTime<FixedOffset>>,
    /// Additional freeform notes submitted by the company as part of the dispute evidence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// The original payment that was disputed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<DisputeLegacyPayment>,
    /// The plan associated with the disputed payment. Null if the dispute is not linked to a specific plan.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<DisputeLegacyPlan>,
    /// The product associated with the disputed payment. Null if the dispute is not linked to a specific product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<DisputeLegacyProduct>,
    /// A description of the product or service provided, submitted as dispute evidence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// A human-readable reason for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The card network reason code for the dispute. Null when the payment processor did not provide one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    /// The refund policy document uploaded as dispute evidence. Null if no refund policy has been provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_attachment: Option<DisputeLegacyRefundPolicyAttachment>,
    /// A text disclosure describing the company's refund policy, submitted as dispute evidence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_disclosure: Option<String>,
    /// An explanation from the company for why a refund was refused, submitted as dispute evidence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_explanation: Option<String>,
    /// The date when the product or service was delivered to the customer, submitted as dispute evidence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_date: Option<String>,
    /// The current status of the dispute lifecycle, such as needs_response, under_review, won, or lost.
    pub status: DisputeStatuses,
    /// An additional attachment that does not fit into the standard evidence categories. Null if not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_attachment: Option<DisputeLegacyUncategorizedAttachment>,
    /// Whether the dispute was automatically resolved through Visa Rapid Dispute Resolution (RDR).
    #[serde(default)]
    pub visa_rdr: bool,
}

impl DisputeLegacy {
    pub fn builder() -> DisputeLegacyBuilder {
        <DisputeLegacyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeLegacyBuilder {
    access_activity_log: Option<String>,
    amount: Option<f64>,
    billing_address: Option<String>,
    cancellation_policy_attachment: Option<DisputeLegacyCancellationPolicyAttachment>,
    cancellation_policy_disclosure: Option<String>,
    company: Option<DisputeLegacyCompany>,
    created_at: Option<DateTime<FixedOffset>>,
    currency: Option<Currencies>,
    customer_communication_attachment: Option<DisputeLegacyCustomerCommunicationAttachment>,
    customer_email_address: Option<String>,
    customer_name: Option<String>,
    editable: Option<bool>,
    id: Option<String>,
    needs_response_by: Option<DateTime<FixedOffset>>,
    notes: Option<String>,
    payment: Option<DisputeLegacyPayment>,
    plan: Option<DisputeLegacyPlan>,
    product: Option<DisputeLegacyProduct>,
    product_description: Option<String>,
    reason: Option<String>,
    reason_code: Option<String>,
    refund_policy_attachment: Option<DisputeLegacyRefundPolicyAttachment>,
    refund_policy_disclosure: Option<String>,
    refund_refusal_explanation: Option<String>,
    service_date: Option<String>,
    status: Option<DisputeStatuses>,
    uncategorized_attachment: Option<DisputeLegacyUncategorizedAttachment>,
    visa_rdr: Option<bool>,
}

impl DisputeLegacyBuilder {
    pub fn access_activity_log(mut self, value: impl Into<String>) -> Self {
        self.access_activity_log = Some(value.into());
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn billing_address(mut self, value: impl Into<String>) -> Self {
        self.billing_address = Some(value.into());
        self
    }

    pub fn cancellation_policy_attachment(
        mut self,
        value: DisputeLegacyCancellationPolicyAttachment,
    ) -> Self {
        self.cancellation_policy_attachment = Some(value);
        self
    }

    pub fn cancellation_policy_disclosure(mut self, value: impl Into<String>) -> Self {
        self.cancellation_policy_disclosure = Some(value.into());
        self
    }

    pub fn company(mut self, value: DisputeLegacyCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn currency(mut self, value: Currencies) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn customer_communication_attachment(
        mut self,
        value: DisputeLegacyCustomerCommunicationAttachment,
    ) -> Self {
        self.customer_communication_attachment = Some(value);
        self
    }

    pub fn customer_email_address(mut self, value: impl Into<String>) -> Self {
        self.customer_email_address = Some(value.into());
        self
    }

    pub fn customer_name(mut self, value: impl Into<String>) -> Self {
        self.customer_name = Some(value.into());
        self
    }

    pub fn editable(mut self, value: bool) -> Self {
        self.editable = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn needs_response_by(mut self, value: DateTime<FixedOffset>) -> Self {
        self.needs_response_by = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn payment(mut self, value: DisputeLegacyPayment) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn plan(mut self, value: DisputeLegacyPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn product(mut self, value: DisputeLegacyProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn product_description(mut self, value: impl Into<String>) -> Self {
        self.product_description = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn reason_code(mut self, value: impl Into<String>) -> Self {
        self.reason_code = Some(value.into());
        self
    }

    pub fn refund_policy_attachment(mut self, value: DisputeLegacyRefundPolicyAttachment) -> Self {
        self.refund_policy_attachment = Some(value);
        self
    }

    pub fn refund_policy_disclosure(mut self, value: impl Into<String>) -> Self {
        self.refund_policy_disclosure = Some(value.into());
        self
    }

    pub fn refund_refusal_explanation(mut self, value: impl Into<String>) -> Self {
        self.refund_refusal_explanation = Some(value.into());
        self
    }

    pub fn service_date(mut self, value: impl Into<String>) -> Self {
        self.service_date = Some(value.into());
        self
    }

    pub fn status(mut self, value: DisputeStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn uncategorized_attachment(mut self, value: DisputeLegacyUncategorizedAttachment) -> Self {
        self.uncategorized_attachment = Some(value);
        self
    }

    pub fn visa_rdr(mut self, value: bool) -> Self {
        self.visa_rdr = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisputeLegacy`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](DisputeLegacyBuilder::amount)
    /// - [`currency`](DisputeLegacyBuilder::currency)
    /// - [`id`](DisputeLegacyBuilder::id)
    /// - [`status`](DisputeLegacyBuilder::status)
    /// - [`visa_rdr`](DisputeLegacyBuilder::visa_rdr)
    pub fn build(self) -> Result<DisputeLegacy, BuildError> {
        Ok(DisputeLegacy {
            access_activity_log: self.access_activity_log,
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            billing_address: self.billing_address,
            cancellation_policy_attachment: self.cancellation_policy_attachment,
            cancellation_policy_disclosure: self.cancellation_policy_disclosure,
            company: self.company,
            created_at: self.created_at,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            customer_communication_attachment: self.customer_communication_attachment,
            customer_email_address: self.customer_email_address,
            customer_name: self.customer_name,
            editable: self.editable,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            needs_response_by: self.needs_response_by,
            notes: self.notes,
            payment: self.payment,
            plan: self.plan,
            product: self.product,
            product_description: self.product_description,
            reason: self.reason,
            reason_code: self.reason_code,
            refund_policy_attachment: self.refund_policy_attachment,
            refund_policy_disclosure: self.refund_policy_disclosure,
            refund_refusal_explanation: self.refund_refusal_explanation,
            service_date: self.service_date,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            uncategorized_attachment: self.uncategorized_attachment,
            visa_rdr: self
                .visa_rdr
                .ok_or_else(|| BuildError::missing_field("visa_rdr"))?,
        })
    }
}
