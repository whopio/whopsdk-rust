pub use crate::prelude::*;

/// A line item on an invoice, representing a single charge with a label, quantity, and unit price.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct InvoiceListItemLineItemsItem {
    /// The label or description for this line item.
    #[serde(default)]
    pub label: String,
    /// The display order of this line item within the invoice.
    #[serde(default)]
    pub position: i64,
    /// The quantity of this line item.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub quantity: f64,
    /// The computed total for this line item (quantity * unit_price).
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total: f64,
    /// The unit price for this line item. Negative for a credit or deduction.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub unit_price: f64,
}

impl InvoiceListItemLineItemsItem {
    pub fn builder() -> InvoiceListItemLineItemsItemBuilder {
        <InvoiceListItemLineItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceListItemLineItemsItemBuilder {
    label: Option<String>,
    position: Option<i64>,
    quantity: Option<f64>,
    total: Option<f64>,
    unit_price: Option<f64>,
}

impl InvoiceListItemLineItemsItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn position(mut self, value: i64) -> Self {
        self.position = Some(value);
        self
    }

    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn total(mut self, value: f64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn unit_price(mut self, value: f64) -> Self {
        self.unit_price = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceListItemLineItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](InvoiceListItemLineItemsItemBuilder::label)
    /// - [`position`](InvoiceListItemLineItemsItemBuilder::position)
    /// - [`quantity`](InvoiceListItemLineItemsItemBuilder::quantity)
    /// - [`total`](InvoiceListItemLineItemsItemBuilder::total)
    /// - [`unit_price`](InvoiceListItemLineItemsItemBuilder::unit_price)
    pub fn build(self) -> Result<InvoiceListItemLineItemsItem, BuildError> {
        Ok(InvoiceListItemLineItemsItem {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            position: self
                .position
                .ok_or_else(|| BuildError::missing_field("position"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            unit_price: self
                .unit_price
                .ok_or_else(|| BuildError::missing_field("unit_price"))?,
        })
    }
}
