pub use crate::prelude::*;

/// Required when `type` is `card_present`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePaymentsRequestPaymentMethodCardPresent {
    /// Which Tap to Pay SDK collects the payment. Stripe offers Tap to Pay in different countries per platform, so eligibility is checked for the named one, as it is for the connection-token session.
    pub platform: CreatePaymentsRequestPaymentMethodCardPresentPlatform,
}

impl CreatePaymentsRequestPaymentMethodCardPresent {
    pub fn builder() -> CreatePaymentsRequestPaymentMethodCardPresentBuilder {
        <CreatePaymentsRequestPaymentMethodCardPresentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsRequestPaymentMethodCardPresentBuilder {
    platform: Option<CreatePaymentsRequestPaymentMethodCardPresentPlatform>,
}

impl CreatePaymentsRequestPaymentMethodCardPresentBuilder {
    pub fn platform(
        mut self,
        value: CreatePaymentsRequestPaymentMethodCardPresentPlatform,
    ) -> Self {
        self.platform = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsRequestPaymentMethodCardPresent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](CreatePaymentsRequestPaymentMethodCardPresentBuilder::platform)
    pub fn build(self) -> Result<CreatePaymentsRequestPaymentMethodCardPresent, BuildError> {
        Ok(CreatePaymentsRequestPaymentMethodCardPresent {
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
        })
    }
}
