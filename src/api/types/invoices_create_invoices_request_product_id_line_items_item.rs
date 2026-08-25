pub use crate::prelude::*;

/// A single line item to include on the invoice, with a label, quantity, and unit price.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateInvoicesRequestProductIdLineItemsItem {
    /// The label or description for this line item.
    #[serde(default)]
    pub label: String,
    /// The quantity of this line item. Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    /// The unit price for this line item. Provided as a number in the specified currency. Eg: 10.43 for $10.43. Negative values represent a credit or deduction, as long as the line items still total a chargeable amount.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub unit_price: f64,
}

impl CreateInvoicesRequestProductIdLineItemsItem {
    pub fn builder() -> CreateInvoicesRequestProductIdLineItemsItemBuilder {
        <CreateInvoicesRequestProductIdLineItemsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInvoicesRequestProductIdLineItemsItemBuilder {
    label: Option<String>,
    quantity: Option<f64>,
    unit_price: Option<f64>,
}

impl CreateInvoicesRequestProductIdLineItemsItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: f64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn unit_price(mut self, value: f64) -> Self {
        self.unit_price = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateInvoicesRequestProductIdLineItemsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](CreateInvoicesRequestProductIdLineItemsItemBuilder::label)
    /// - [`unit_price`](CreateInvoicesRequestProductIdLineItemsItemBuilder::unit_price)
    pub fn build(self) -> Result<CreateInvoicesRequestProductIdLineItemsItem, BuildError> {
        Ok(CreateInvoicesRequestProductIdLineItemsItem {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            quantity: self.quantity,
            unit_price: self
                .unit_price
                .ok_or_else(|| BuildError::missing_field("unit_price"))?,
        })
    }
}
