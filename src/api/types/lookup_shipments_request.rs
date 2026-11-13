pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LookupShipmentsRequest {
    /// The selling account, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// The buyer's current receipt email.
    #[serde(default)]
    pub email: String,
    /// The payment ID prefixed `pay_`, or the order number printed on the receipt (optional leading `#`). Supply this or `tracking_number`, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_number: Option<String>,
    /// The exact carrier tracking number. Supply this or `order_number`, not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

impl LookupShipmentsRequest {
    pub fn builder() -> LookupShipmentsRequestBuilder {
        <LookupShipmentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LookupShipmentsRequestBuilder {
    account_id: Option<String>,
    email: Option<String>,
    order_number: Option<String>,
    tracking_number: Option<String>,
}

impl LookupShipmentsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn order_number(mut self, value: impl Into<String>) -> Self {
        self.order_number = Some(value.into());
        self
    }

    pub fn tracking_number(mut self, value: impl Into<String>) -> Self {
        self.tracking_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LookupShipmentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](LookupShipmentsRequestBuilder::account_id)
    /// - [`email`](LookupShipmentsRequestBuilder::email)
    pub fn build(self) -> Result<LookupShipmentsRequest, BuildError> {
        Ok(LookupShipmentsRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            order_number: self.order_number,
            tracking_number: self.tracking_number,
        })
    }
}
