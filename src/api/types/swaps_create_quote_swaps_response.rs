pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateQuoteSwapsResponse {
    /// Source token amount used for the quote.
    #[serde(default)]
    pub amount_in: String,
    /// Estimated destination token amount.
    #[serde(default)]
    pub amount_out: String,
    /// Minimum destination amount after slippage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_out_min: Option<String>,
    /// Estimated bridge fee for cross-chain swaps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bridge_fee: Option<String>,
    /// Estimated time for the swap to complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_duration_seconds: Option<i64>,
    /// Whop fee in basis points.
    #[serde(default)]
    pub fee_bps: i64,
    /// Source wallet address used for the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_address: Option<String>,
    /// Resolved source token details.
    #[serde(default)]
    pub from_token: HashMap<String, serde_json::Value>,
    /// Metadata from the request.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    pub object: CreateQuoteSwapsResponseObject,
    /// Quoted exchange rate.
    #[serde(default)]
    pub rate: String,
    /// Whether the source token needs approval before swapping.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requires_token_approval: Option<bool>,
    /// Destination wallet address used for the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_address: Option<String>,
    /// Resolved destination token details.
    #[serde(default)]
    pub to_token: HashMap<String, serde_json::Value>,
}

impl CreateQuoteSwapsResponse {
    pub fn builder() -> CreateQuoteSwapsResponseBuilder {
        <CreateQuoteSwapsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateQuoteSwapsResponseBuilder {
    amount_in: Option<String>,
    amount_out: Option<String>,
    amount_out_min: Option<String>,
    bridge_fee: Option<String>,
    estimated_duration_seconds: Option<i64>,
    fee_bps: Option<i64>,
    from_address: Option<String>,
    from_token: Option<HashMap<String, serde_json::Value>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    object: Option<CreateQuoteSwapsResponseObject>,
    rate: Option<String>,
    requires_token_approval: Option<bool>,
    to_address: Option<String>,
    to_token: Option<HashMap<String, serde_json::Value>>,
}

impl CreateQuoteSwapsResponseBuilder {
    pub fn amount_in(mut self, value: impl Into<String>) -> Self {
        self.amount_in = Some(value.into());
        self
    }

    pub fn amount_out(mut self, value: impl Into<String>) -> Self {
        self.amount_out = Some(value.into());
        self
    }

    pub fn amount_out_min(mut self, value: impl Into<String>) -> Self {
        self.amount_out_min = Some(value.into());
        self
    }

    pub fn bridge_fee(mut self, value: impl Into<String>) -> Self {
        self.bridge_fee = Some(value.into());
        self
    }

    pub fn estimated_duration_seconds(mut self, value: i64) -> Self {
        self.estimated_duration_seconds = Some(value);
        self
    }

    pub fn fee_bps(mut self, value: i64) -> Self {
        self.fee_bps = Some(value);
        self
    }

    pub fn from_address(mut self, value: impl Into<String>) -> Self {
        self.from_address = Some(value.into());
        self
    }

    pub fn from_token(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.from_token = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn object(mut self, value: CreateQuoteSwapsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn rate(mut self, value: impl Into<String>) -> Self {
        self.rate = Some(value.into());
        self
    }

    pub fn requires_token_approval(mut self, value: bool) -> Self {
        self.requires_token_approval = Some(value);
        self
    }

    pub fn to_address(mut self, value: impl Into<String>) -> Self {
        self.to_address = Some(value.into());
        self
    }

    pub fn to_token(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.to_token = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateQuoteSwapsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_in`](CreateQuoteSwapsResponseBuilder::amount_in)
    /// - [`amount_out`](CreateQuoteSwapsResponseBuilder::amount_out)
    /// - [`fee_bps`](CreateQuoteSwapsResponseBuilder::fee_bps)
    /// - [`from_token`](CreateQuoteSwapsResponseBuilder::from_token)
    /// - [`metadata`](CreateQuoteSwapsResponseBuilder::metadata)
    /// - [`object`](CreateQuoteSwapsResponseBuilder::object)
    /// - [`rate`](CreateQuoteSwapsResponseBuilder::rate)
    /// - [`to_token`](CreateQuoteSwapsResponseBuilder::to_token)
    pub fn build(self) -> Result<CreateQuoteSwapsResponse, BuildError> {
        Ok(CreateQuoteSwapsResponse {
            amount_in: self
                .amount_in
                .ok_or_else(|| BuildError::missing_field("amount_in"))?,
            amount_out: self
                .amount_out
                .ok_or_else(|| BuildError::missing_field("amount_out"))?,
            amount_out_min: self.amount_out_min,
            bridge_fee: self.bridge_fee,
            estimated_duration_seconds: self.estimated_duration_seconds,
            fee_bps: self
                .fee_bps
                .ok_or_else(|| BuildError::missing_field("fee_bps"))?,
            from_address: self.from_address,
            from_token: self
                .from_token
                .ok_or_else(|| BuildError::missing_field("from_token"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
            requires_token_approval: self.requires_token_approval,
            to_address: self.to_address,
            to_token: self
                .to_token
                .ok_or_else(|| BuildError::missing_field("to_token"))?,
        })
    }
}
