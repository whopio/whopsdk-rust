pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InvoiceLineItem {
    /// Text shown under the item on the invoice, its email, and its PDF, read from the variant's `description`. A custom charge sets it with the item's `description`. A product variant keeps its own. Null when the variant has none or has since been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The promo discount applied to this item, in the payment's `currency`. Null when no discount was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount: Option<Money>,
    /// Line item ID, prefixed `li_`. Null when the payment predates item snapshots and the item is read from the payment's variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The item's name as shown at checkout (the product title, else the variant title).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// The product the variant belongs to, prefixed `prod_`. Null for a variant with no product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// How many units were bought.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub quantity: f64,
    /// `unit_amount` times `quantity`, before discounts and tax, in the payment's `currency`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal: Option<Money>,
    /// The tax collected on this item, in the payment's `currency`. Null when no tax was recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<Money>,
    /// The saved price of one unit before discounts, in the payment's `currency`. Creating or replacing recurring items includes the upfront fee and first billing period. Renewal invoices exclude the upfront fee. Other edits preserve saved prices. Null when the payment recorded no price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<Money>,
    /// The variant bought, prefixed `plan_`. Null when the variant has since been deleted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
}

impl InvoiceLineItem {
    pub fn builder() -> InvoiceLineItemBuilder {
        <InvoiceLineItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceLineItemBuilder {
    description: Option<String>,
    discount_amount: Option<Money>,
    id: Option<String>,
    label: Option<String>,
    product_id: Option<String>,
    quantity: Option<f64>,
    subtotal: Option<Money>,
    tax_amount: Option<Money>,
    unit_amount: Option<Money>,
    variant_id: Option<String>,
}

impl InvoiceLineItemBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn discount_amount(mut self, value: Money) -> Self {
        self.discount_amount = Some(value);
        self
    }

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

    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn subtotal(mut self, value: Money) -> Self {
        self.subtotal = Some(value);
        self
    }

    pub fn tax_amount(mut self, value: Money) -> Self {
        self.tax_amount = Some(value);
        self
    }

    pub fn unit_amount(mut self, value: Money) -> Self {
        self.unit_amount = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceLineItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`quantity`](InvoiceLineItemBuilder::quantity)
    pub fn build(self) -> Result<InvoiceLineItem, BuildError> {
        Ok(InvoiceLineItem {
            description: self.description,
            discount_amount: self.discount_amount,
            id: self.id,
            label: self.label,
            product_id: self.product_id,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            subtotal: self.subtotal,
            tax_amount: self.tax_amount,
            unit_amount: self.unit_amount,
            variant_id: self.variant_id,
        })
    }
}
