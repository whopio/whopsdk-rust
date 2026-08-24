pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeEvidence {
    /// Log of the customer's access to the product, such as sign-in or download activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_activity_log: Option<String>,
    /// The billing address the customer provided at checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<String>,
    /// The cancellation policy document. Falls back to Whop's platform policy when the seller has not uploaded their own.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_attachment: Option<DisputeAttachment>,
    /// How the cancellation policy was shown to the customer before purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_disclosure: Option<String>,
    /// Correspondence with the customer, or proof they used the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_communication_attachment: Option<DisputeAttachment>,
    /// The email address the customer used at checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,
    /// The customer's name as given at checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    #[serde(default)]
    pub documents: Vec<DisputeEvidenceDocument>,
    /// Any additional context for the processor reviewing the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// What the customer purchased, in the seller's own words.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// The refund policy document. Falls back to Whop's platform policy when the seller has not uploaded their own.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_attachment: Option<DisputeAttachment>,
    /// How the refund policy was shown to the customer before purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_disclosure: Option<String>,
    /// Why a refund was refused, when one was requested and denied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_explanation: Option<String>,
    /// When the product or service was delivered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_date: Option<String>,
    /// Supporting evidence that does not fit the other categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_attachment: Option<DisputeAttachment>,
}

impl DisputeEvidence {
    pub fn builder() -> DisputeEvidenceBuilder {
        <DisputeEvidenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeEvidenceBuilder {
    access_activity_log: Option<String>,
    billing_address: Option<String>,
    cancellation_policy_attachment: Option<DisputeAttachment>,
    cancellation_policy_disclosure: Option<String>,
    customer_communication_attachment: Option<DisputeAttachment>,
    customer_email_address: Option<String>,
    customer_name: Option<String>,
    documents: Option<Vec<DisputeEvidenceDocument>>,
    notes: Option<String>,
    product_description: Option<String>,
    refund_policy_attachment: Option<DisputeAttachment>,
    refund_policy_disclosure: Option<String>,
    refund_refusal_explanation: Option<String>,
    service_date: Option<String>,
    uncategorized_attachment: Option<DisputeAttachment>,
}

impl DisputeEvidenceBuilder {
    pub fn access_activity_log(mut self, value: impl Into<String>) -> Self {
        self.access_activity_log = Some(value.into());
        self
    }

    pub fn billing_address(mut self, value: impl Into<String>) -> Self {
        self.billing_address = Some(value.into());
        self
    }

    pub fn cancellation_policy_attachment(mut self, value: DisputeAttachment) -> Self {
        self.cancellation_policy_attachment = Some(value);
        self
    }

    pub fn cancellation_policy_disclosure(mut self, value: impl Into<String>) -> Self {
        self.cancellation_policy_disclosure = Some(value.into());
        self
    }

    pub fn customer_communication_attachment(mut self, value: DisputeAttachment) -> Self {
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

    pub fn documents(mut self, value: Vec<DisputeEvidenceDocument>) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn product_description(mut self, value: impl Into<String>) -> Self {
        self.product_description = Some(value.into());
        self
    }

    pub fn refund_policy_attachment(mut self, value: DisputeAttachment) -> Self {
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

    pub fn uncategorized_attachment(mut self, value: DisputeAttachment) -> Self {
        self.uncategorized_attachment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DisputeEvidence`].
    /// This method will fail if any of the following fields are not set:
    /// - [`documents`](DisputeEvidenceBuilder::documents)
    pub fn build(self) -> Result<DisputeEvidence, BuildError> {
        Ok(DisputeEvidence {
            access_activity_log: self.access_activity_log,
            billing_address: self.billing_address,
            cancellation_policy_attachment: self.cancellation_policy_attachment,
            cancellation_policy_disclosure: self.cancellation_policy_disclosure,
            customer_communication_attachment: self.customer_communication_attachment,
            customer_email_address: self.customer_email_address,
            customer_name: self.customer_name,
            documents: self
                .documents
                .ok_or_else(|| BuildError::missing_field("documents"))?,
            notes: self.notes,
            product_description: self.product_description,
            refund_policy_attachment: self.refund_policy_attachment,
            refund_policy_disclosure: self.refund_policy_disclosure,
            refund_refusal_explanation: self.refund_refusal_explanation,
            service_date: self.service_date,
            uncategorized_attachment: self.uncategorized_attachment,
        })
    }
}
