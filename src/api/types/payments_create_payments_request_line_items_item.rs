pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePaymentsRequestLineItemsItem {
    /// How many units of the variant to purchase. Defaults to 1; more than 1 requires the variant to allow multiple quantities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Find or create the variant for this entry by its attributes. Unknown properties return an error. Creating a variant requires `plan:create`, and creating or updating a product requires the matching product permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<CreatePaymentsRequestLineItemsItemVariant>,
    /// An existing variant to charge for, prefixed `plan_`. Mutually exclusive with `variant`. Each variant may appear once — use `quantity` for multiple units.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
}

impl CreatePaymentsRequestLineItemsItem {
    pub fn builder() -> CreatePaymentsRequestLineItemsItemBuilder {
        <CreatePaymentsRequestLineItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsRequestLineItemsItemBuilder {
    quantity: Option<i64>,
    variant: Option<CreatePaymentsRequestLineItemsItemVariant>,
    variant_id: Option<String>,
}

impl CreatePaymentsRequestLineItemsItemBuilder {
    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn variant(mut self, value: CreatePaymentsRequestLineItemsItemVariant) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsRequestLineItemsItem`].
    pub fn build(self) -> Result<CreatePaymentsRequestLineItemsItem, BuildError> {
        Ok(CreatePaymentsRequestLineItemsItem {
            quantity: self.quantity,
            variant: self.variant,
            variant_id: self.variant_id,
        })
    }
}
