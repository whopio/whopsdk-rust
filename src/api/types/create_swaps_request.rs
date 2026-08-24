pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSwapsRequest {
    /// Business or user account ID (biz_* / user_*).
    #[serde(default)]
    pub account_id: String,
    /// Source token amount. Required for crypto swaps. For fiat pairs: the amount of from_token to convert at the mid-market rate; omit (along with to_amount) to repay the full negative to_token balance instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Source chain name or chain ID. Defaults to the source token's chain when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_chain: Option<CreateSwapsRequestFromChain>,
    /// Source token contract address or ticker symbol, such as "USDT".
    #[serde(default)]
    pub from_token: String,
    /// Maximum slippage tolerance in basis points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_bps: Option<i64>,
    /// Fiat pairs only: sizes a partial repayment of the negative to_token balance, denominated in to_token. Must not exceed the debt. Mutually exclusive with amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_amount: Option<String>,
    /// Destination chain name or chain ID. Defaults to the destination token's chain when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_chain: Option<CreateSwapsRequestToChain>,
    /// Destination token contract address or ticker symbol, such as "XAUT".
    #[serde(default)]
    pub to_token: String,
}

impl CreateSwapsRequest {
    pub fn builder() -> CreateSwapsRequestBuilder {
        <CreateSwapsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSwapsRequestBuilder {
    account_id: Option<String>,
    amount: Option<String>,
    from_chain: Option<CreateSwapsRequestFromChain>,
    from_token: Option<String>,
    slippage_bps: Option<i64>,
    to_amount: Option<String>,
    to_chain: Option<CreateSwapsRequestToChain>,
    to_token: Option<String>,
}

impl CreateSwapsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn from_chain(mut self, value: CreateSwapsRequestFromChain) -> Self {
        self.from_chain = Some(value);
        self
    }

    pub fn from_token(mut self, value: impl Into<String>) -> Self {
        self.from_token = Some(value.into());
        self
    }

    pub fn slippage_bps(mut self, value: i64) -> Self {
        self.slippage_bps = Some(value);
        self
    }

    pub fn to_amount(mut self, value: impl Into<String>) -> Self {
        self.to_amount = Some(value.into());
        self
    }

    pub fn to_chain(mut self, value: CreateSwapsRequestToChain) -> Self {
        self.to_chain = Some(value);
        self
    }

    pub fn to_token(mut self, value: impl Into<String>) -> Self {
        self.to_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSwapsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateSwapsRequestBuilder::account_id)
    /// - [`from_token`](CreateSwapsRequestBuilder::from_token)
    /// - [`to_token`](CreateSwapsRequestBuilder::to_token)
    pub fn build(self) -> Result<CreateSwapsRequest, BuildError> {
        Ok(CreateSwapsRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            amount: self.amount,
            from_chain: self.from_chain,
            from_token: self
                .from_token
                .ok_or_else(|| BuildError::missing_field("from_token"))?,
            slippage_bps: self.slippage_bps,
            to_amount: self.to_amount,
            to_chain: self.to_chain,
            to_token: self
                .to_token
                .ok_or_else(|| BuildError::missing_field("to_token"))?,
        })
    }
}
