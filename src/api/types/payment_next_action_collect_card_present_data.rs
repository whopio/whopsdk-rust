pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentNextActionCollectCardPresentData {
    /// The Stripe PaymentIntent client secret the Terminal SDK retrieves, collects and confirms this payment with. Treat it as a credential: together with a card-present connection token it is what a device needs to take the payment.
    #[serde(default)]
    pub client_secret: String,
}

impl PaymentNextActionCollectCardPresentData {
    pub fn builder() -> PaymentNextActionCollectCardPresentDataBuilder {
        <PaymentNextActionCollectCardPresentDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentNextActionCollectCardPresentDataBuilder {
    client_secret: Option<String>,
}

impl PaymentNextActionCollectCardPresentDataBuilder {
    pub fn client_secret(mut self, value: impl Into<String>) -> Self {
        self.client_secret = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentNextActionCollectCardPresentData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_secret`](PaymentNextActionCollectCardPresentDataBuilder::client_secret)
    pub fn build(self) -> Result<PaymentNextActionCollectCardPresentData, BuildError> {
        Ok(PaymentNextActionCollectCardPresentData {
            client_secret: self
                .client_secret
                .ok_or_else(|| BuildError::missing_field("client_secret"))?,
        })
    }
}
