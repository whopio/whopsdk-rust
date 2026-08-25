pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutSessionItem {
    /// The seller's longer description of the item, or `null`. Multi-line text as the seller wrote it, suited to a collapsible details block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// How many days of access a one-time purchase grants, or `null` when access does not expire (a renewing plan's access follows its billing instead).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i64>,
    /// One line describing the item, or `null`. The checkout link's own description when the seller wrote one, otherwise the product's headline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    /// The item's square art image URL, or `null` when the seller uploaded none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// What this item is called, ready to render — the product's title, falling back to the seller's name. Never empty.
    #[serde(default)]
    pub name: String,
    /// The plan being purchased (`plan_…`). The plan is the price — sessions never carry client-asserted amounts.
    #[serde(default)]
    pub plan: String,
    /// How many of the plan the buyer is purchasing. At least 1.
    #[serde(default)]
    pub quantity: i64,
}

impl CheckoutSessionItem {
    pub fn builder() -> CheckoutSessionItemBuilder {
        <CheckoutSessionItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionItemBuilder {
    description: Option<String>,
    expiration_days: Option<i64>,
    headline: Option<String>,
    image_url: Option<String>,
    name: Option<String>,
    plan: Option<String>,
    quantity: Option<i64>,
}

impl CheckoutSessionItemBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn expiration_days(mut self, value: i64) -> Self {
        self.expiration_days = Some(value);
        self
    }

    pub fn headline(mut self, value: impl Into<String>) -> Self {
        self.headline = Some(value.into());
        self
    }

    pub fn image_url(mut self, value: impl Into<String>) -> Self {
        self.image_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn plan(mut self, value: impl Into<String>) -> Self {
        self.plan = Some(value.into());
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CheckoutSessionItemBuilder::name)
    /// - [`plan`](CheckoutSessionItemBuilder::plan)
    /// - [`quantity`](CheckoutSessionItemBuilder::quantity)
    pub fn build(self) -> Result<CheckoutSessionItem, BuildError> {
        Ok(CheckoutSessionItem {
            description: self.description,
            expiration_days: self.expiration_days,
            headline: self.headline,
            image_url: self.image_url,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
            quantity: self
                .quantity
                .ok_or_else(|| BuildError::missing_field("quantity"))?,
        })
    }
}
