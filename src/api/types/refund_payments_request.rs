pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RefundPaymentsRequest {
    /// Your own key-value data to attach to the refund, returned on it and on its `refund.created` and `refund.updated` webhooks. At most 50 keys; keys up to 100 characters and values up to 500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
    /// The amount to refund, stated in this payment's `currency` like every other amount on it. When the buyer was billed in a different currency, it is converted at the payment's own exchange rate before the refund is issued. An amount that covers everything still refundable refunds the rest of the payment — omit it to refund the rest without having to work out what that is.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_amount: Option<f64>,
}

impl RefundPaymentsRequest {
    pub fn builder() -> RefundPaymentsRequestBuilder {
        <RefundPaymentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RefundPaymentsRequestBuilder {
    metadata: Option<HashMap<String, Option<String>>>,
    partial_amount: Option<f64>,
}

impl RefundPaymentsRequestBuilder {
    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn partial_amount(mut self, value: f64) -> Self {
        self.partial_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundPaymentsRequest`].
    pub fn build(self) -> Result<RefundPaymentsRequest, BuildError> {
        Ok(RefundPaymentsRequest {
            metadata: self.metadata,
            partial_amount: self.partial_amount,
        })
    }
}
