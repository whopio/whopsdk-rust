pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateInvoicesRequestItemsItem {
    /// Text shown under a custom charge on the invoice, its email, and its PDF, up to 1,000 characters. Line breaks are kept. Only for an item with `variant`, where it is saved as that variant's `description` and overrides `variant.description`. An item with `variant_id` shows its variant's own description and rejects this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// Find or create a variant inline by its attributes, using the payment API properties. Requires plan:create.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<CreateInvoicesRequestItemsItemVariant>,
    /// An existing variant belonging to the account, identified by its plan_ ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
}

impl CreateInvoicesRequestItemsItem {
    pub fn builder() -> CreateInvoicesRequestItemsItemBuilder {
        <CreateInvoicesRequestItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInvoicesRequestItemsItemBuilder {
    description: Option<String>,
    quantity: Option<i64>,
    variant: Option<CreateInvoicesRequestItemsItemVariant>,
    variant_id: Option<String>,
}

impl CreateInvoicesRequestItemsItemBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn variant(mut self, value: CreateInvoicesRequestItemsItemVariant) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateInvoicesRequestItemsItem`].
    pub fn build(self) -> Result<CreateInvoicesRequestItemsItem, BuildError> {
        Ok(CreateInvoicesRequestItemsItem {
            description: self.description,
            quantity: self.quantity,
            variant: self.variant,
            variant_id: self.variant_id,
        })
    }
}
