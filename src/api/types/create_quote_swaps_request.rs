pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateQuoteSwapsRequest {
    /// Source token amount.
    #[serde(default)]
    pub amount: String,
    /// Source wallet address used for the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// Source chain name or chain ID. Defaults to the source token's chain when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_chain: Option<CreateQuoteSwapsRequestFromChain>,
    /// Source token contract address or ticker symbol, such as "USDT".
    #[serde(default)]
    pub from_token: String,
    /// Metadata to include with the quote response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Maximum slippage tolerance in basis points.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_bps: Option<i64>,
    /// Destination wallet address used for the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_address: Option<String>,
    /// Destination chain name or chain ID. Defaults to the destination token's chain when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_chain: Option<CreateQuoteSwapsRequestToChain>,
    /// Destination token contract address or ticker symbol, such as "XAUT".
    #[serde(default)]
    pub to_token: String,
}

impl CreateQuoteSwapsRequest {
    pub fn builder() -> CreateQuoteSwapsRequestBuilder {
        <CreateQuoteSwapsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateQuoteSwapsRequestBuilder {
    amount: Option<String>,
    from_address: Option<String>,
    from_chain: Option<CreateQuoteSwapsRequestFromChain>,
    from_token: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    slippage_bps: Option<i64>,
    to_address: Option<String>,
    to_chain: Option<CreateQuoteSwapsRequestToChain>,
    to_token: Option<String>,
}

impl CreateQuoteSwapsRequestBuilder {
    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn from_address(mut self, value: impl Into<String>) -> Self {
        self.from_address = Some(value.into());
        self
    }

    pub fn from_chain(mut self, value: CreateQuoteSwapsRequestFromChain) -> Self {
        self.from_chain = Some(value);
        self
    }

    pub fn from_token(mut self, value: impl Into<String>) -> Self {
        self.from_token = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn slippage_bps(mut self, value: i64) -> Self {
        self.slippage_bps = Some(value);
        self
    }

    pub fn to_address(mut self, value: impl Into<String>) -> Self {
        self.to_address = Some(value.into());
        self
    }

    pub fn to_chain(mut self, value: CreateQuoteSwapsRequestToChain) -> Self {
        self.to_chain = Some(value);
        self
    }

    pub fn to_token(mut self, value: impl Into<String>) -> Self {
        self.to_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateQuoteSwapsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CreateQuoteSwapsRequestBuilder::amount)
    /// - [`from_token`](CreateQuoteSwapsRequestBuilder::from_token)
    /// - [`to_token`](CreateQuoteSwapsRequestBuilder::to_token)
    pub fn build(self) -> Result<CreateQuoteSwapsRequest, BuildError> {
        Ok(CreateQuoteSwapsRequest {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            from_address: self.from_address,
            from_chain: self.from_chain,
            from_token: self
                .from_token
                .ok_or_else(|| BuildError::missing_field("from_token"))?,
            metadata: self.metadata,
            slippage_bps: self.slippage_bps,
            to_address: self.to_address,
            to_chain: self.to_chain,
            to_token: self
                .to_token
                .ok_or_else(|| BuildError::missing_field("to_token"))?,
        })
    }
}
