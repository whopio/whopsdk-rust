pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateDepositsRequest {
    /// Amount to prefill on hosted deposit page. Crypto deposits require a $10 minimum. With `payment_method_id`, the amount in USD to charge, and required.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// Account ID to fund, `biz_` or `user_`. Any business resolves without authentication; a user account resolves only for that same authenticated user.
    #[serde(default)]
    pub destination: String,
    /// Custom metadata to attach to the payment the charge creates. Ignored unless `payment_method_id` is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<CreateDepositsRequestMetadataValue>>>,
    /// SWIFT/BIC of the bank the funds are sent from, 8 or 11 characters. Fills each bank currency's `swift_bic` with the intermediary bank to route the wire through.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_bic: Option<String>,
    /// A saved payment method of the business to charge for `amount`, prefixed `payt_`. Save one with Create setup intent and `owner_id` set to the account. Only for a `biz_` destination, and requires authentication with the `payment:charge` scope. A declined charge fails the request with 400.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_id: Option<String>,
}

impl CreateDepositsRequest {
    pub fn builder() -> CreateDepositsRequestBuilder {
        <CreateDepositsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDepositsRequestBuilder {
    amount: Option<f64>,
    destination: Option<String>,
    metadata: Option<HashMap<String, Option<CreateDepositsRequestMetadataValue>>>,
    origin_bic: Option<String>,
    payment_method_id: Option<String>,
}

impl CreateDepositsRequestBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn destination(mut self, value: impl Into<String>) -> Self {
        self.destination = Some(value.into());
        self
    }

    pub fn metadata(
        mut self,
        value: HashMap<String, Option<CreateDepositsRequestMetadataValue>>,
    ) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn origin_bic(mut self, value: impl Into<String>) -> Self {
        self.origin_bic = Some(value.into());
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateDepositsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](CreateDepositsRequestBuilder::destination)
    pub fn build(self) -> Result<CreateDepositsRequest, BuildError> {
        Ok(CreateDepositsRequest {
            amount: self.amount,
            destination: self
                .destination
                .ok_or_else(|| BuildError::missing_field("destination"))?,
            metadata: self.metadata,
            origin_bic: self.origin_bic,
            payment_method_id: self.payment_method_id,
        })
    }
}
