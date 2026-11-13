pub use crate::prelude::*;

/// Options for a `paypal` payment method.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePaymentsRequestPaymentMethodPaypal {
    /// Has PayPal ask the buyer for a shipping address. Tax is recalculated for each address the buyer picks before they approve, so the total they approve is the total charged, and the address they confirm is returned on the payment as `shipping_address`. Cannot be combined with `shipping_address`. Defaults to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_shipping: Option<bool>,
}

impl CreatePaymentsRequestPaymentMethodPaypal {
    pub fn builder() -> CreatePaymentsRequestPaymentMethodPaypalBuilder {
        <CreatePaymentsRequestPaymentMethodPaypalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsRequestPaymentMethodPaypalBuilder {
    request_shipping: Option<bool>,
}

impl CreatePaymentsRequestPaymentMethodPaypalBuilder {
    pub fn request_shipping(mut self, value: bool) -> Self {
        self.request_shipping = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsRequestPaymentMethodPaypal`].
    pub fn build(self) -> Result<CreatePaymentsRequestPaymentMethodPaypal, BuildError> {
        Ok(CreatePaymentsRequestPaymentMethodPaypal {
            request_shipping: self.request_shipping,
        })
    }
}
