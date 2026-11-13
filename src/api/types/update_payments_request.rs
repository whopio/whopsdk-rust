pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePaymentsRequest {
    /// Where the buyer continues after completing an off-site step. An absolute https URL without credentials, at most 2,048 characters. Only for payments created with a `confirmation_token`, and only until the buyer has returned. Omit it to leave it unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// The complete new shipping address. It replaces the current address as a whole and is never merged with it, so send every field the address should have, including the ones that are not changing. Any field you leave out is cleared: sending only `city` leaves an address with nothing but a city. Pass null to remove the address, or omit `shipping_address` to leave it unchanged. It cannot change once a shipment exists for the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<UpdatePaymentsRequestShippingAddress>,
}

impl UpdatePaymentsRequest {
    pub fn builder() -> UpdatePaymentsRequestBuilder {
        <UpdatePaymentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePaymentsRequestBuilder {
    return_url: Option<String>,
    shipping_address: Option<UpdatePaymentsRequestShippingAddress>,
}

impl UpdatePaymentsRequestBuilder {
    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    pub fn shipping_address(mut self, value: UpdatePaymentsRequestShippingAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePaymentsRequest`].
    pub fn build(self) -> Result<UpdatePaymentsRequest, BuildError> {
        Ok(UpdatePaymentsRequest {
            return_url: self.return_url,
            shipping_address: self.shipping_address,
        })
    }
}
