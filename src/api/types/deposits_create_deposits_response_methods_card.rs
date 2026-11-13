pub use crate::prelude::*;

/// The charge to a saved payment method. Null unless `payment_method_id` was passed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateDepositsResponseMethodsCard {
    /// The custom metadata attached to the payment the charge created. Null if none was passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
    /// The payment the charge created, prefixed `pay_`. Retrieve it, or listen for `payment.succeeded`, to confirm the funds landed.
    #[serde(default)]
    pub payment_id: String,
    /// The saved payment method that was charged, prefixed `payt_`.
    #[serde(default)]
    pub payment_method_id: String,
}

impl CreateDepositsResponseMethodsCard {
    pub fn builder() -> CreateDepositsResponseMethodsCardBuilder {
        <CreateDepositsResponseMethodsCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDepositsResponseMethodsCardBuilder {
    metadata: Option<HashMap<String, Option<String>>>,
    payment_id: Option<String>,
    payment_method_id: Option<String>,
}

impl CreateDepositsResponseMethodsCardBuilder {
    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn payment_method_id(mut self, value: impl Into<String>) -> Self {
        self.payment_method_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateDepositsResponseMethodsCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_id`](CreateDepositsResponseMethodsCardBuilder::payment_id)
    /// - [`payment_method_id`](CreateDepositsResponseMethodsCardBuilder::payment_method_id)
    pub fn build(self) -> Result<CreateDepositsResponseMethodsCard, BuildError> {
        Ok(CreateDepositsResponseMethodsCard {
            metadata: self.metadata,
            payment_id: self
                .payment_id
                .ok_or_else(|| BuildError::missing_field("payment_id"))?,
            payment_method_id: self
                .payment_method_id
                .ok_or_else(|| BuildError::missing_field("payment_method_id"))?,
        })
    }
}
