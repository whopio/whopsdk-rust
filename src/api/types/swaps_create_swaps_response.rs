pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateSwapsResponse {
    /// Account ID that owns the wallet used for the swap.
    #[serde(default)]
    pub account_id: String,
    /// Fiat pairs only: amount of the source currency converted. Null while a stablecoin repayment is processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_in: Option<f64>,
    /// Fiat pairs only: amount credited in the destination currency. Null while a stablecoin repayment is processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_out: Option<f64>,
    /// Expected destination token amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_out_expected: Option<String>,
    /// Minimum destination amount after slippage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_out_min: Option<String>,
    /// Fiat pairs only: the source currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_token: Option<CreateSwapsResponseFromToken>,
    /// Swap ID. Poll `GET /swaps/:id` for status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub object: CreateSwapsResponseObject,
    /// Quoted exchange rate used to create the swap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    /// Swap status. Crypto swaps start `queued`; fiat conversions return `complete`, or `working` while a stablecoin repayment settles.
    pub status: CreateSwapsResponseStatus,
    /// Destination chain for the swap.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_chain: Option<String>,
    /// Fiat pairs only: the destination currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_token: Option<CreateSwapsResponseToToken>,
}

impl CreateSwapsResponse {
    pub fn builder() -> CreateSwapsResponseBuilder {
        <CreateSwapsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSwapsResponseBuilder {
    account_id: Option<String>,
    amount_in: Option<f64>,
    amount_out: Option<f64>,
    amount_out_expected: Option<String>,
    amount_out_min: Option<String>,
    from_token: Option<CreateSwapsResponseFromToken>,
    id: Option<String>,
    object: Option<CreateSwapsResponseObject>,
    rate: Option<String>,
    status: Option<CreateSwapsResponseStatus>,
    to_chain: Option<String>,
    to_token: Option<CreateSwapsResponseToToken>,
}

impl CreateSwapsResponseBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount_in(mut self, value: f64) -> Self {
        self.amount_in = Some(value);
        self
    }

    pub fn amount_out(mut self, value: f64) -> Self {
        self.amount_out = Some(value);
        self
    }

    pub fn amount_out_expected(mut self, value: impl Into<String>) -> Self {
        self.amount_out_expected = Some(value.into());
        self
    }

    pub fn amount_out_min(mut self, value: impl Into<String>) -> Self {
        self.amount_out_min = Some(value.into());
        self
    }

    pub fn from_token(mut self, value: CreateSwapsResponseFromToken) -> Self {
        self.from_token = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: CreateSwapsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn rate(mut self, value: impl Into<String>) -> Self {
        self.rate = Some(value.into());
        self
    }

    pub fn status(mut self, value: CreateSwapsResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn to_chain(mut self, value: impl Into<String>) -> Self {
        self.to_chain = Some(value.into());
        self
    }

    pub fn to_token(mut self, value: CreateSwapsResponseToToken) -> Self {
        self.to_token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSwapsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateSwapsResponseBuilder::account_id)
    /// - [`object`](CreateSwapsResponseBuilder::object)
    /// - [`status`](CreateSwapsResponseBuilder::status)
    pub fn build(self) -> Result<CreateSwapsResponse, BuildError> {
        Ok(CreateSwapsResponse {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            amount_in: self.amount_in,
            amount_out: self.amount_out,
            amount_out_expected: self.amount_out_expected,
            amount_out_min: self.amount_out_min,
            from_token: self.from_token,
            id: self.id,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            rate: self.rate,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            to_chain: self.to_chain,
            to_token: self.to_token,
        })
    }
}
