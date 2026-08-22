pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RefundPaymentsRequest {
    /// The amount to refund. For multi-currency payments, this is in the charge currency (what the buyer paid). For single-currency, this is in the payment currency. If omitted, the full payment amount is refunded.
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
    partial_amount: Option<f64>,
}

impl RefundPaymentsRequestBuilder {
    pub fn partial_amount(mut self, value: f64) -> Self {
        self.partial_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RefundPaymentsRequest`].
    pub fn build(self) -> Result<RefundPaymentsRequest, BuildError> {
        Ok(RefundPaymentsRequest {
            partial_amount: self.partial_amount,
        })
    }
}
