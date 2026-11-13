pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TradingOrder {
    /// Client order ID, prefixed `trdcloid_`, or `null` when the order was placed without one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    /// When the order was placed, as an ISO 8601 timestamp, or `null` when the provider omits it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Hyperliquid-specific order details. Present on Hyperliquid orders, otherwise `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hyperliquid: Option<TradingHyperliquidOrder>,
    /// Trading order ID, prefixed `trdord_`.
    #[serde(default)]
    pub id: String,
    /// Market symbol on the provider, such as `ETH`.
    #[serde(default)]
    pub market: String,
    pub object: TradingOrderObject,
    pub order_type: TradingOrderOrderType,
    /// Size when the order was placed, as a decimal string, or `null` when the provider omits it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_size: Option<String>,
    /// Limit price in USD.
    #[serde(default)]
    pub price: Money,
    /// The provider's own order ID, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_order_id: Option<String>,
    pub side: TradingOrderSide,
    /// Remaining order size as a decimal string.
    #[serde(default)]
    pub size: String,
    pub status: TradingOrderStatus,
    /// When the status last changed, as an ISO 8601 timestamp, or `null` when the provider omits it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_updated_at: Option<String>,
    /// How long the order stays active. `null` when the provider omits it or reports a policy outside the supported values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TradingOrderTimeInForce>,
}

impl TradingOrder {
    pub fn builder() -> TradingOrderBuilder {
        <TradingOrderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradingOrderBuilder {
    client_order_id: Option<String>,
    created_at: Option<String>,
    hyperliquid: Option<TradingHyperliquidOrder>,
    id: Option<String>,
    market: Option<String>,
    object: Option<TradingOrderObject>,
    order_type: Option<TradingOrderOrderType>,
    original_size: Option<String>,
    price: Option<Money>,
    provider_order_id: Option<String>,
    side: Option<TradingOrderSide>,
    size: Option<String>,
    status: Option<TradingOrderStatus>,
    status_updated_at: Option<String>,
    time_in_force: Option<TradingOrderTimeInForce>,
}

impl TradingOrderBuilder {
    pub fn client_order_id(mut self, value: impl Into<String>) -> Self {
        self.client_order_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn hyperliquid(mut self, value: TradingHyperliquidOrder) -> Self {
        self.hyperliquid = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn market(mut self, value: impl Into<String>) -> Self {
        self.market = Some(value.into());
        self
    }

    pub fn object(mut self, value: TradingOrderObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn order_type(mut self, value: TradingOrderOrderType) -> Self {
        self.order_type = Some(value);
        self
    }

    pub fn original_size(mut self, value: impl Into<String>) -> Self {
        self.original_size = Some(value.into());
        self
    }

    pub fn price(mut self, value: Money) -> Self {
        self.price = Some(value);
        self
    }

    pub fn provider_order_id(mut self, value: impl Into<String>) -> Self {
        self.provider_order_id = Some(value.into());
        self
    }

    pub fn side(mut self, value: TradingOrderSide) -> Self {
        self.side = Some(value);
        self
    }

    pub fn size(mut self, value: impl Into<String>) -> Self {
        self.size = Some(value.into());
        self
    }

    pub fn status(mut self, value: TradingOrderStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_updated_at(mut self, value: impl Into<String>) -> Self {
        self.status_updated_at = Some(value.into());
        self
    }

    pub fn time_in_force(mut self, value: TradingOrderTimeInForce) -> Self {
        self.time_in_force = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TradingOrder`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TradingOrderBuilder::id)
    /// - [`market`](TradingOrderBuilder::market)
    /// - [`object`](TradingOrderBuilder::object)
    /// - [`order_type`](TradingOrderBuilder::order_type)
    /// - [`price`](TradingOrderBuilder::price)
    /// - [`side`](TradingOrderBuilder::side)
    /// - [`size`](TradingOrderBuilder::size)
    /// - [`status`](TradingOrderBuilder::status)
    pub fn build(self) -> Result<TradingOrder, BuildError> {
        Ok(TradingOrder {
            client_order_id: self.client_order_id,
            created_at: self.created_at,
            hyperliquid: self.hyperliquid,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            market: self
                .market
                .ok_or_else(|| BuildError::missing_field("market"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            order_type: self
                .order_type
                .ok_or_else(|| BuildError::missing_field("order_type"))?,
            original_size: self.original_size,
            price: self
                .price
                .ok_or_else(|| BuildError::missing_field("price"))?,
            provider_order_id: self.provider_order_id,
            side: self.side.ok_or_else(|| BuildError::missing_field("side"))?,
            size: self.size.ok_or_else(|| BuildError::missing_field("size"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            status_updated_at: self.status_updated_at,
            time_in_force: self.time_in_force,
        })
    }
}
