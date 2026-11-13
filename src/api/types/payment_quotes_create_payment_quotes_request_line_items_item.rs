pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePaymentQuotesRequestLineItemsItem {
    /// How many units of the variant to purchase. Defaults to 1; more than 1 requires the variant to allow multiple quantities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// An existing variant to charge for, prefixed `plan_`. Each variant may appear once — use `quantity` for multiple units.
    #[serde(default)]
    pub variant_id: String,
}

impl CreatePaymentQuotesRequestLineItemsItem {
    pub fn builder() -> CreatePaymentQuotesRequestLineItemsItemBuilder {
        <CreatePaymentQuotesRequestLineItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentQuotesRequestLineItemsItemBuilder {
    quantity: Option<i64>,
    variant_id: Option<String>,
}

impl CreatePaymentQuotesRequestLineItemsItemBuilder {
    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentQuotesRequestLineItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`variant_id`](CreatePaymentQuotesRequestLineItemsItemBuilder::variant_id)
    pub fn build(self) -> Result<CreatePaymentQuotesRequestLineItemsItem, BuildError> {
        Ok(CreatePaymentQuotesRequestLineItemsItem {
            quantity: self.quantity,
            variant_id: self
                .variant_id
                .ok_or_else(|| BuildError::missing_field("variant_id"))?,
        })
    }
}
