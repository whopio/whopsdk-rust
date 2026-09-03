pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateDepositsRequest {
    /// Amount to prefill on hosted deposit page.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount: Option<f64>,
    /// Account ID to fund, `biz_` or `user_`. Any business resolves without authentication; a user account resolves only for that same authenticated user.
    #[serde(default)]
    pub destination: String,
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

    /// Consumes the builder and constructs a [`CreateDepositsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`destination`](CreateDepositsRequestBuilder::destination)
    pub fn build(self) -> Result<CreateDepositsRequest, BuildError> {
        Ok(CreateDepositsRequest {
            amount: self.amount,
            destination: self
                .destination
                .ok_or_else(|| BuildError::missing_field("destination"))?,
        })
    }
}
