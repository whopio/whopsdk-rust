pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ReceiptLineItem {
    /// Line item ID, prefixed `li_`. Null when the payment predates item snapshots and the item is read from the payment's variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The item's name as shown at checkout (the product title, else the variant title).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The product the variant belongs to, prefixed `prod_`. On a payment that predates item snapshots this falls back to the variant's product, so it can be set where the parent's own `product_id` is null. Null for a variant with no product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The product's current title, or `null` when the item has no product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_title: Option<String>,
    /// How many units were bought.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub quantity: f64,
    /// The variant's stock keeping unit, free text set by the seller (e.g. `TSHIRT-LARGE-BLUE`). Accounts with billing-reason SKU suffixes enabled receive it stamped with why the charge happened: `-S` for a first subscription charge, `-R` for a renewal, `-C` for a one-time purchase, `-U` when the payment's own metadata carries `upsell: true`, `-F` when nothing was charged. Other billing reasons carry the bare SKU. `null` when the variant has no SKU or has been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    /// The recorded amount for this item's full quantity, before discounts, tax, and fees, in its purchase currency. Returns `null` when no item amount was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<Money>,
    /// The variant bought, prefixed `plan_`. Null when the variant has since been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
    /// The variant's current title, or `null` when the variant has been deleted or has no title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_title: Option<String>,
}

impl ReceiptLineItem {
    pub fn builder() -> ReceiptLineItemBuilder {
        <ReceiptLineItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptLineItemBuilder {
    id: Option<String>,
    label: Option<String>,
    product_id: Option<String>,
    product_title: Option<String>,
    quantity: Option<f64>,
    sku: Option<String>,
    subtotal: Option<Money>,
    variant_id: Option<String>,
    variant_title: Option<String>,
}

impl ReceiptLineItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn product_title(mut self, value: impl Into<String>) -> Self {
        self.product_title = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn sku(mut self, value: impl Into<String>) -> Self {
        self.sku = Some(value.into());
        self
    }

    pub fn subtotal(mut self, value: Money) -> Self {
        self.subtotal = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    pub fn variant_title(mut self, value: impl Into<String>) -> Self {
        self.variant_title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReceiptLineItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quantity`](ReceiptLineItemBuilder::quantity)
    pub fn build(self) -> Result<ReceiptLineItem, BuildError> {
        Ok(ReceiptLineItem {
            id: self.id,
            label: self.label,
            product_id: self.product_id,
            product_title: self.product_title,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            sku: self.sku,
            subtotal: self.subtotal,
            variant_id: self.variant_id,
            variant_title: self.variant_title,
        })
    }
}
