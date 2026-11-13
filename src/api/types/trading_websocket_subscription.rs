pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TradingWebsocketSubscription {
    /// The live update stream this subscription opens.
    pub channel: TradingWebsocketSubscriptionChannel,
    /// JSON subscription message to send unchanged over the Hyperliquid WebSocket.
    #[serde(default)]
    pub message: String,
}

impl TradingWebsocketSubscription {
    pub fn builder() -> TradingWebsocketSubscriptionBuilder {
        <TradingWebsocketSubscriptionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TradingWebsocketSubscriptionBuilder {
    channel: Option<TradingWebsocketSubscriptionChannel>,
    message: Option<String>,
}

impl TradingWebsocketSubscriptionBuilder {
    pub fn channel(mut self, value: TradingWebsocketSubscriptionChannel) -> Self {
        self.channel = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TradingWebsocketSubscription`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel`](TradingWebsocketSubscriptionBuilder::channel)
    /// - [`message`](TradingWebsocketSubscriptionBuilder::message)
    pub fn build(self) -> Result<TradingWebsocketSubscription, BuildError> {
        Ok(TradingWebsocketSubscription {
            channel: self
                .channel
                .ok_or_else(|| BuildError::missing_field("channel"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
