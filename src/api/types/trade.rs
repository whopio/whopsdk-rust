pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Trade {
    /// The account that owns the trading wallet, prefixed `biz_`. `null` when a user owns it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The USDT0 that a deposit, a spot buy, or a perpetual order short of trading balance, sends from the wallet, or the USDC that a withdraw sends back, in native units, as a decimal string. A withdraw of `all` shows what it sent once it finishes. `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// The average fill price of the order, in USD, as a decimal string. For spot, the price the swap was quoted at. `null` when it is not known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_price: Option<String>,
    /// The caller's own ID for the trade, unique for the account. `null` when none was sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// When the trade ended as completed or failed, as an ISO 8601 timestamp. `null` before that.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// When the trade was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// ID Whop sends to the exchange with this trade's order, so the exchange's order updates can be matched to the trade. On Hyperliquid, the order's `cloid` reported in `orderUpdates`, a `0x`-prefixed 32-character hex string. Not the caller's `client_order_id`. `null` for a deposit or withdraw.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_order_id: Option<String>,
    /// Why the trade failed, or `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<TradeFailureCode>,
    /// The size the order filled, in units of the market, as a decimal string. For a spot buy, the amount the swap was quoted to deliver. `null` until it fills.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filled_size: Option<String>,
    /// Where the money is when the trade did not complete as asked: `wallet`, `trading_account`, or `unknown` while a person checks it. `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funds_location: Option<TradeFundsLocation>,
    /// Trade ID, prefixed `tint_`.
    #[serde(default)]
    pub id: String,
    /// `perpetual` or `spot` for an order, `perpetual` for a close. `null` for a deposit or withdraw.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<TradeInstrumentType>,
    /// The cross leverage set before an opening perpetual order. `null` for a spot order, close, deposit, withdraw or reduce-only order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<i64>,
    /// The market, for example `BTC` for a perpetual or `XAUT` for spot. `null` for a deposit or withdraw.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    /// Free-form string-to-string annotations stored on the trade.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    pub object: TradeObject,
    /// `market`, `limit`, `take_profit` or `stop_loss` for an order. `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<TradeOrderType>,
    /// For a take-profit or stop-loss sent with an order, that order's trade, prefixed `tint_`. `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// The limit price of the order in USD. A market order's price is the mid price moved by `slippage_bps`; a take-profit or stop-loss's is the worst price its market order may fill at once triggered. `null` for other types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<Money>,
    /// Whether the order can only reduce a position. `null` for other types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// `buy` or `sell` for an order. `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<TradeSide>,
    /// The order size in units of the market, as a decimal string. `null` for other types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// `pending` until the trade starts. `processing` while it runs, also while an order that did not fill sends the money it bridged in back to the wallet. `completed` when it did what was asked. `failed` when it did not; `funds_location` says where the money is. `in_review` when the outcome is unknown and a person checks it.
    pub status: TradeStatus,
    /// `partial_fill` when the order filled only part of its size. `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<TradeStatusDetail>,
    /// What a live trade is doing now: `depositing` while the wallet's USDT0 is being sent to the bridge, `landing` once sent and until it reaches the trading balance, `placing` while the order or close is with the venue, `withdrawing` while USDC heads back to the wallet, `returning` while money an unfilled order bridged in goes back. A spot order is `quoting` while the swap is priced, `swapping` while it runs onchain, and `crediting` until the new balance shows in the wallet. `null` once the trade is no longer pending or processing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub step: Option<TradeStep>,
    /// The price in USD at which a take-profit or stop-loss sends its market order. `null` for other orders and types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<Money>,
    /// `order` places one order: a perpetual order trades from the trading balance, a spot order swaps between USDT0 and the asset in the wallet. `deposit` bridges USDT0 from the wallet to the trading balance. `withdraw` sends USDC from the trading balance back to the wallet. `close` closes the whole position in one market with a market reduce-only order; when the account is then flat, a `withdraw` trade sends the trading balance back to the wallet.
    pub r#type: TradeType,
    /// When the trade last changed, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// The user who owns the trading wallet, prefixed `user_`. `null` when an account owns it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl Trade {
    pub fn builder() -> TradeBuilder {
        <TradeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradeBuilder {
    account_id: Option<String>,
    amount: Option<String>,
    average_price: Option<String>,
    client_order_id: Option<String>,
    completed_at: Option<String>,
    created_at: Option<String>,
    external_order_id: Option<String>,
    failure_code: Option<TradeFailureCode>,
    filled_size: Option<String>,
    funds_location: Option<TradeFundsLocation>,
    id: Option<String>,
    instrument_type: Option<TradeInstrumentType>,
    leverage: Option<i64>,
    market: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    object: Option<TradeObject>,
    order_type: Option<TradeOrderType>,
    parent_id: Option<String>,
    price: Option<Money>,
    reduce_only: Option<bool>,
    side: Option<TradeSide>,
    size: Option<String>,
    status: Option<TradeStatus>,
    status_detail: Option<TradeStatusDetail>,
    step: Option<TradeStep>,
    trigger_price: Option<Money>,
    r#type: Option<TradeType>,
    updated_at: Option<String>,
    user_id: Option<String>,
}

impl TradeBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn average_price(mut self, value: impl Into<String>) -> Self {
        self.average_price = Some(value.into());
        self
    }

    pub fn client_order_id(mut self, value: impl Into<String>) -> Self {
        self.client_order_id = Some(value.into());
        self
    }

    pub fn completed_at(mut self, value: impl Into<String>) -> Self {
        self.completed_at = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn external_order_id(mut self, value: impl Into<String>) -> Self {
        self.external_order_id = Some(value.into());
        self
    }

    pub fn failure_code(mut self, value: TradeFailureCode) -> Self {
        self.failure_code = Some(value);
        self
    }

    pub fn filled_size(mut self, value: impl Into<String>) -> Self {
        self.filled_size = Some(value.into());
        self
    }

    pub fn funds_location(mut self, value: TradeFundsLocation) -> Self {
        self.funds_location = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn instrument_type(mut self, value: TradeInstrumentType) -> Self {
        self.instrument_type = Some(value);
        self
    }

    pub fn leverage(mut self, value: i64) -> Self {
        self.leverage = Some(value);
        self
    }

    pub fn market(mut self, value: impl Into<String>) -> Self {
        self.market = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn object(mut self, value: TradeObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn order_type(mut self, value: TradeOrderType) -> Self {
        self.order_type = Some(value);
        self
    }

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn price(mut self, value: Money) -> Self {
        self.price = Some(value);
        self
    }

    pub fn reduce_only(mut self, value: bool) -> Self {
        self.reduce_only = Some(value);
        self
    }

    pub fn side(mut self, value: TradeSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn size(mut self, value: impl Into<String>) -> Self {
        self.size = Some(value.into());
        self
    }

    pub fn status(mut self, value: TradeStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_detail(mut self, value: TradeStatusDetail) -> Self {
        self.status_detail = Some(value);
        self
    }

    pub fn step(mut self, value: TradeStep) -> Self {
        self.step = Some(value);
        self
    }

    pub fn trigger_price(mut self, value: Money) -> Self {
        self.trigger_price = Some(value);
        self
    }

    pub fn r#type(mut self, value: TradeType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Trade`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](TradeBuilder::created_at)
    /// - [`id`](TradeBuilder::id)
    /// - [`metadata`](TradeBuilder::metadata)
    /// - [`object`](TradeBuilder::object)
    /// - [`status`](TradeBuilder::status)
    /// - [`r#type`](TradeBuilder::r#type)
    /// - [`updated_at`](TradeBuilder::updated_at)
    pub fn build(self) -> Result<Trade, BuildError> {
        Ok(Trade {
            account_id: self.account_id,
            amount: self.amount,
            average_price: self.average_price,
            client_order_id: self.client_order_id,
            completed_at: self.completed_at,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            external_order_id: self.external_order_id,
            failure_code: self.failure_code,
            filled_size: self.filled_size,
            funds_location: self.funds_location,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            instrument_type: self.instrument_type,
            leverage: self.leverage,
            market: self.market,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            order_type: self.order_type,
            parent_id: self.parent_id,
            price: self.price,
            reduce_only: self.reduce_only,
            side: self.side,
            size: self.size,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            status_detail: self.status_detail,
            step: self.step,
            trigger_price: self.trigger_price,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user_id: self.user_id,
        })
    }
}
