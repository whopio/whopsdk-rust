pub use crate::prelude::*;

/// The saved payment method created by this setup intent. Null if the setup has not completed successfully.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateSetupIntentsResponsePaymentMethod {
    /// The card data associated with the payment method, if its a debit or credit card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CreateSetupIntentsResponsePaymentMethodCard>,
    /// The datetime the payment token was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The unique identifier for the payment token.
    #[serde(default)]
    pub id: String,
    /// The mailing address associated with the payment method's user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<CreateSetupIntentsResponsePaymentMethodMailingAddress>,
    /// The payment method type of the payment method
    pub payment_method_type: PaymentMethodTypes,
}

impl CreateSetupIntentsResponsePaymentMethod {
    pub fn builder() -> CreateSetupIntentsResponsePaymentMethodBuilder {
        <CreateSetupIntentsResponsePaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSetupIntentsResponsePaymentMethodBuilder {
    card: Option<CreateSetupIntentsResponsePaymentMethodCard>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    mailing_address: Option<CreateSetupIntentsResponsePaymentMethodMailingAddress>,
    payment_method_type: Option<PaymentMethodTypes>,
}

impl CreateSetupIntentsResponsePaymentMethodBuilder {
    pub fn card(mut self, value: CreateSetupIntentsResponsePaymentMethodCard) -> Self {
        self.card = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn mailing_address(
        mut self,
        value: CreateSetupIntentsResponsePaymentMethodMailingAddress,
    ) -> Self {
        self.mailing_address = Some(value);
        self
    }

    pub fn payment_method_type(mut self, value: PaymentMethodTypes) -> Self {
        self.payment_method_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateSetupIntentsResponsePaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](CreateSetupIntentsResponsePaymentMethodBuilder::created_at)
    /// - [`id`](CreateSetupIntentsResponsePaymentMethodBuilder::id)
    /// - [`payment_method_type`](CreateSetupIntentsResponsePaymentMethodBuilder::payment_method_type)
    pub fn build(self) -> Result<CreateSetupIntentsResponsePaymentMethod, BuildError> {
        Ok(CreateSetupIntentsResponsePaymentMethod {
            card: self.card,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            mailing_address: self.mailing_address,
            payment_method_type: self
                .payment_method_type
                .ok_or_else(|| BuildError::missing_field("payment_method_type"))?,
        })
    }
}
