pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentStatus {
    /// The payment this status describes, prefixed `pay_`.
    #[serde(default)]
    pub id: String,
    /// Details of the most recent failed attempt, or `null` when the payment has not failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_payment_error: Option<PaymentLastPaymentError>,
    /// What the buyer must do next while `status` is `requires_action`, otherwise `null`. `type` picks the shape and each variant carries only its own `data`, so switching on `type` gives you exactly that step's payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<PaymentNextAction>,
    /// Always `payment_status`.
    #[serde(default)]
    pub object: String,
    /// Present while `status` is `processing` on a settlement rail, otherwise `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub processing_details: Option<PaymentProcessingDetails>,
    /// Where to send the buyer once the payment reaches a resting state, or `null` to leave them where they are. Editable until they return — see the return_url operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// How far the payment has got. `requires_confirmation` — nothing attempted yet, or the last attempt failed and can be retried. `requires_action` — the buyer has a step outstanding; see `next_action`. `confirming` — the buyer has done their part and the processor is deciding. `processing` — the money is moving; see `processing_details`. `succeeded` — collected. `canceled` — voided or written off.
    pub status: PaymentStatusStatus,
}

impl PaymentStatus {
    pub fn builder() -> PaymentStatusBuilder {
        <PaymentStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentStatusBuilder {
    id: Option<String>,
    last_payment_error: Option<PaymentLastPaymentError>,
    next_action: Option<PaymentNextAction>,
    object: Option<String>,
    processing_details: Option<PaymentProcessingDetails>,
    return_url: Option<String>,
    status: Option<PaymentStatusStatus>,
}

impl PaymentStatusBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_payment_error(mut self, value: PaymentLastPaymentError) -> Self {
        self.last_payment_error = Some(value);
        self
    }

    pub fn next_action(mut self, value: PaymentNextAction) -> Self {
        self.next_action = Some(value);
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn processing_details(mut self, value: PaymentProcessingDetails) -> Self {
        self.processing_details = Some(value);
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: PaymentStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaymentStatusBuilder::id)
    /// - [`object`](PaymentStatusBuilder::object)
    /// - [`status`](PaymentStatusBuilder::status)
    pub fn build(self) -> Result<PaymentStatus, BuildError> {
        Ok(PaymentStatus {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_payment_error: self.last_payment_error,
            next_action: self.next_action,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            processing_details: self.processing_details,
            return_url: self.return_url,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
