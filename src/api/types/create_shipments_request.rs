pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateShipmentsRequest {
    /// The unique identifier of the account, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The payment to attach the shipment to, prefixed `pay_`.
    #[serde(default)]
    pub payment_id: String,
    /// The carrier-assigned tracking number.
    #[serde(default)]
    pub tracking_number: String,
}

impl CreateShipmentsRequest {
    pub fn builder() -> CreateShipmentsRequestBuilder {
        <CreateShipmentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateShipmentsRequestBuilder {
    account_id: Option<String>,
    payment_id: Option<String>,
    tracking_number: Option<String>,
}

impl CreateShipmentsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn tracking_number(mut self, value: impl Into<String>) -> Self {
        self.tracking_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateShipmentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_id`](CreateShipmentsRequestBuilder::payment_id)
    /// - [`tracking_number`](CreateShipmentsRequestBuilder::tracking_number)
    pub fn build(self) -> Result<CreateShipmentsRequest, BuildError> {
        Ok(CreateShipmentsRequest {
            account_id: self.account_id,
            payment_id: self
                .payment_id
                .ok_or_else(|| BuildError::missing_field("payment_id"))?,
            tracking_number: self
                .tracking_number
                .ok_or_else(|| BuildError::missing_field("tracking_number"))?,
        })
    }
}
