pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateEvidenceDisputeRequest {
    /// An IP access activity log showing the customer used the service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_activity_log: Option<String>,
    /// The billing address associated with the customer's payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<String>,
    /// A file upload containing the company's cancellation policy document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_attachment:
        Option<UpdateEvidenceDisputeRequestCancellationPolicyAttachment>,
    /// The company's cancellation policy text to submit as evidence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_disclosure: Option<String>,
    /// A file upload containing evidence of customer communication. Must be a JPEG, PNG, GIF, or PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_communication_attachment:
        Option<UpdateEvidenceDisputeRequestCustomerCommunicationAttachment>,
    /// The email address of the customer associated with the disputed payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,
    /// The full name of the customer associated with the disputed payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,
    /// Additional notes or context to submit as part of the dispute evidence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// A description of the product or service that was provided to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,
    /// A file upload containing the company's refund policy document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_attachment: Option<UpdateEvidenceDisputeRequestRefundPolicyAttachment>,
    /// The company's refund policy text to submit as evidence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_disclosure: Option<String>,
    /// An explanation of why the refund request was refused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_explanation: Option<String>,
    /// The date when the product or service was delivered to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_date: Option<String>,
    /// A file upload for evidence that does not fit into the other categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_attachment: Option<UpdateEvidenceDisputeRequestUncategorizedAttachment>,
}

impl UpdateEvidenceDisputeRequest {
    pub fn builder() -> UpdateEvidenceDisputeRequestBuilder {
        <UpdateEvidenceDisputeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateEvidenceDisputeRequestBuilder {
    access_activity_log: Option<String>,
    billing_address: Option<String>,
    cancellation_policy_attachment:
        Option<UpdateEvidenceDisputeRequestCancellationPolicyAttachment>,
    cancellation_policy_disclosure: Option<String>,
    customer_communication_attachment:
        Option<UpdateEvidenceDisputeRequestCustomerCommunicationAttachment>,
    customer_email_address: Option<String>,
    customer_name: Option<String>,
    notes: Option<String>,
    product_description: Option<String>,
    refund_policy_attachment: Option<UpdateEvidenceDisputeRequestRefundPolicyAttachment>,
    refund_policy_disclosure: Option<String>,
    refund_refusal_explanation: Option<String>,
    service_date: Option<String>,
    uncategorized_attachment: Option<UpdateEvidenceDisputeRequestUncategorizedAttachment>,
}

impl UpdateEvidenceDisputeRequestBuilder {
    pub fn access_activity_log(mut self, value: impl Into<String>) -> Self {
        self.access_activity_log = Some(value.into());
        self
    }

    pub fn billing_address(mut self, value: impl Into<String>) -> Self {
        self.billing_address = Some(value.into());
        self
    }

    pub fn cancellation_policy_attachment(
        mut self,
        value: UpdateEvidenceDisputeRequestCancellationPolicyAttachment,
    ) -> Self {
        self.cancellation_policy_attachment = Some(value);
        self
    }

    pub fn cancellation_policy_disclosure(mut self, value: impl Into<String>) -> Self {
        self.cancellation_policy_disclosure = Some(value.into());
        self
    }

    pub fn customer_communication_attachment(
        mut self,
        value: UpdateEvidenceDisputeRequestCustomerCommunicationAttachment,
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

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn product_description(mut self, value: impl Into<String>) -> Self {
        self.product_description = Some(value.into());
        self
    }

    pub fn refund_policy_attachment(
        mut self,
        value: UpdateEvidenceDisputeRequestRefundPolicyAttachment,
    ) -> Self {
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

    pub fn uncategorized_attachment(
        mut self,
        value: UpdateEvidenceDisputeRequestUncategorizedAttachment,
    ) -> Self {
        self.uncategorized_attachment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateEvidenceDisputeRequest`].
    pub fn build(self) -> Result<UpdateEvidenceDisputeRequest, BuildError> {
        Ok(UpdateEvidenceDisputeRequest {
            access_activity_log: self.access_activity_log,
            billing_address: self.billing_address,
            cancellation_policy_attachment: self.cancellation_policy_attachment,
            cancellation_policy_disclosure: self.cancellation_policy_disclosure,
            customer_communication_attachment: self.customer_communication_attachment,
            customer_email_address: self.customer_email_address,
            customer_name: self.customer_name,
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
