pub use crate::prelude::*;

/// A payment method named on this call rather than described by a confirmation token. `type` names it and the member named after it carries its details. Nothing is collected on this call: the payment is created first and the buyer or device then acts on the `next_action` the status endpoint serves. `card_present` (Tap to Pay): the reader collects against the payment with the client secret in `next_action`. `paypal`: the buyer approves on PayPal's page, reached through a `redirect` `next_action`, then returns to `return_url`; `member_id` or `email` names the buyer. Mutually exclusive with `confirmation_token` and `payment_method_id`, and with `member_id` except for `paypal`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreatePaymentsRequestPaymentMethod {
    /// Required when `type` is `card_present`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<CreatePaymentsRequestPaymentMethodCardPresent>,
    /// Options for a `paypal` payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<CreatePaymentsRequestPaymentMethodPaypal>,
    /// The kind of payment method; the member named after it carries its details.
    pub r#type: CreatePaymentsRequestPaymentMethodType,
}

impl CreatePaymentsRequestPaymentMethod {
    pub fn builder() -> CreatePaymentsRequestPaymentMethodBuilder {
        <CreatePaymentsRequestPaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsRequestPaymentMethodBuilder {
    card_present: Option<CreatePaymentsRequestPaymentMethodCardPresent>,
    paypal: Option<CreatePaymentsRequestPaymentMethodPaypal>,
    r#type: Option<CreatePaymentsRequestPaymentMethodType>,
}

impl CreatePaymentsRequestPaymentMethodBuilder {
    pub fn card_present(mut self, value: CreatePaymentsRequestPaymentMethodCardPresent) -> Self {
        self.card_present = Some(value);
        self
    }

    pub fn paypal(mut self, value: CreatePaymentsRequestPaymentMethodPaypal) -> Self {
        self.paypal = Some(value);
        self
    }

    pub fn r#type(mut self, value: CreatePaymentsRequestPaymentMethodType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsRequestPaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CreatePaymentsRequestPaymentMethodBuilder::r#type)
    pub fn build(self) -> Result<CreatePaymentsRequestPaymentMethod, BuildError> {
        Ok(CreatePaymentsRequestPaymentMethod {
            card_present: self.card_present,
            paypal: self.paypal,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
