pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTradesRequest {
    /// The account or user whose wallet trades, prefixed `biz_` or `user_`.
    #[serde(default)]
    pub account_id: String,
    /// Only for a deposit or withdraw, and required for both. For a deposit, the USDT0 to send from the wallet, with at most 6 decimals. For a withdraw, the USDC to send back, at least 1 with at most 6 decimals, or `all`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Only for an order. Your own ID for the trade, 1 to 128 characters, unique for the account. Sending it again with the same order returns the same trade; with a different order it answers `409`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// `perpetual` or `spot`. Required for an order. A spot order is a market order that swaps in the wallet and does not use the trading balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<CreateTradesRequestInstrumentType>,
    /// Only for a perpetual order. The cross leverage, from 1 to the market's maximum. Required unless `reduce_only`; not accepted on a reduce-only order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<i64>,
    /// The market: a perpetual such as `BTC` or `xyz:TSLA`, or for spot an asset the wallet can swap into, such as `BTC` or `XAUT`. Required for an order or close, and a close takes only a perpetual; not accepted for a deposit or withdraw.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    /// Only for an order. Free-form string-to-string annotations: up to 50 keys of at most 40 characters, values up to 500 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// Only for an order. The order value in USD, as a decimal string; the size is rounded down to the market's decimals. Send exactly one of `size` or `notional`; a spot buy takes `notional`, the USDT0 to spend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notional: Option<String>,
    /// `market` fills now within `slippage_bps` of the mid price, and anything unfilled is canceled; `limit` fills at `price` or better, and how long it stays on the book follows `time_in_force`. `take_profit` and `stop_loss` reduce the open perpetual position with a market order once the price crosses `trigger_price`; the trade stays `processing` until then. Required for an order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_type: Option<CreateTradesRequestOrderType>,
    /// Only for an order. The limit price in USD, as a decimal string Hyperliquid accepts without rounding. Required for a limit order; not accepted for a market order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Only for a perpetual order. Only reduce an open position; defaults to `false`. A reduce-only order takes no `leverage`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// `buy` or `sell`. Required for an order, except a `take_profit` or `stop_loss`, where it defaults to the side that reduces the position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<CreateTradesRequestSide>,
    /// Only for an order. The size in units of the market, as a decimal string, with at most the market's size decimals. Send exactly one of `size` or `notional`; a spot sell takes `size`. A perpetual order must be worth at least 10 USD, unless reduce-only. A `take_profit` or `stop_loss` takes `size` or nothing for the whole position, and a set `size` must be worth at least 10 USD at `trigger_price`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// Only for a market order or a close. The worst price as basis points from the mid price, from 1 to 1000; defaults to 100 for a perpetual, and to the swap route's own default for spot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slippage_bps: Option<i64>,
    /// Only for a perpetual order that opens or adds to a position. A stop-loss for the order's full size, armed once the order fills and sent with it as its own trade, whose `parent_id` is this trade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<CreateTradesRequestStopLoss>,
    /// Only for a perpetual order that opens or adds to a position. A take-profit for the order's full size, armed once the order fills and sent with it as its own trade, whose `parent_id` is this trade.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<CreateTradesRequestTakeProfit>,
    /// Only for a perpetual limit order. `good_til_canceled` rests on the book until it fills; `add_liquidity_only` is canceled instead of filling at once; `immediate_or_cancel` fills what it can now and cancels the rest. Defaults to `good_til_canceled`. The trade stays `processing` while the order rests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<CreateTradesRequestTimeInForce>,
    /// Required for a `take_profit` or `stop_loss`, and only for them. The USD price, as a decimal string Hyperliquid accepts without rounding, at which it sends a market order. Closing a long, a take-profit triggers above the current price and a stop-loss below; closing a short, the reverse. It is always reduce-only, needs an open position, and takes no `price`, `notional`, `leverage`, `time_in_force` or `slippage_bps`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    /// `order` places one order. A perpetual order trades from the trading balance, setting cross `leverage` first; when the balance is short, it bridges the USDT0 it needs from the wallet first. A spot order swaps between USDT0 and the asset in the wallet. `deposit` bridges `amount` USDT0 from the wallet to the trading balance. `withdraw` sends `amount` USDC (or `all`) from the trading balance back to the wallet. `close` closes the whole position in `market` with a market reduce-only order; when the account is then flat, a `withdraw` trade sends the trading balance back to the wallet.
    pub r#type: CreateTradesRequestType,
}

impl CreateTradesRequest {
    pub fn builder() -> CreateTradesRequestBuilder {
        <CreateTradesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTradesRequestBuilder {
    account_id: Option<String>,
    amount: Option<String>,
    client_order_id: Option<String>,
    instrument_type: Option<CreateTradesRequestInstrumentType>,
    leverage: Option<i64>,
    market: Option<String>,
    metadata: Option<HashMap<String, String>>,
    notional: Option<String>,
    order_type: Option<CreateTradesRequestOrderType>,
    price: Option<String>,
    reduce_only: Option<bool>,
    side: Option<CreateTradesRequestSide>,
    size: Option<String>,
    slippage_bps: Option<i64>,
    stop_loss: Option<CreateTradesRequestStopLoss>,
    take_profit: Option<CreateTradesRequestTakeProfit>,
    time_in_force: Option<CreateTradesRequestTimeInForce>,
    trigger_price: Option<String>,
    r#type: Option<CreateTradesRequestType>,
}

impl CreateTradesRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn client_order_id(mut self, value: impl Into<String>) -> Self {
        self.client_order_id = Some(value.into());
        self
    }

    pub fn instrument_type(mut self, value: CreateTradesRequestInstrumentType) -> Self {
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

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn notional(mut self, value: impl Into<String>) -> Self {
        self.notional = Some(value.into());
        self
    }

    pub fn order_type(mut self, value: CreateTradesRequestOrderType) -> Self {
        self.order_type = Some(value);
        self
    }

    pub fn price(mut self, value: impl Into<String>) -> Self {
        self.price = Some(value.into());
        self
    }

    pub fn reduce_only(mut self, value: bool) -> Self {
        self.reduce_only = Some(value);
        self
    }

    pub fn side(mut self, value: CreateTradesRequestSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn size(mut self, value: impl Into<String>) -> Self {
        self.size = Some(value.into());
        self
    }

    pub fn slippage_bps(mut self, value: i64) -> Self {
        self.slippage_bps = Some(value);
        self
    }

    pub fn stop_loss(mut self, value: CreateTradesRequestStopLoss) -> Self {
        self.stop_loss = Some(value);
        self
    }

    pub fn take_profit(mut self, value: CreateTradesRequestTakeProfit) -> Self {
        self.take_profit = Some(value);
        self
    }

    pub fn time_in_force(mut self, value: CreateTradesRequestTimeInForce) -> Self {
        self.time_in_force = Some(value);
        self
    }

    pub fn trigger_price(mut self, value: impl Into<String>) -> Self {
        self.trigger_price = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: CreateTradesRequestType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTradesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateTradesRequestBuilder::account_id)
    /// - [`r#type`](CreateTradesRequestBuilder::r#type)
    pub fn build(self) -> Result<CreateTradesRequest, BuildError> {
        Ok(CreateTradesRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            amount: self.amount,
            client_order_id: self.client_order_id,
            instrument_type: self.instrument_type,
            leverage: self.leverage,
            market: self.market,
            metadata: self.metadata,
            notional: self.notional,
            order_type: self.order_type,
            price: self.price,
            reduce_only: self.reduce_only,
            side: self.side,
            size: self.size,
            slippage_bps: self.slippage_bps,
            stop_loss: self.stop_loss,
            take_profit: self.take_profit,
            time_in_force: self.time_in_force,
            trigger_price: self.trigger_price,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
