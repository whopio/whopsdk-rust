pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductPublicPlan {
    /// Number of days between recurring charges, such as 30 for monthly or 365 for annual. `null` for one-time plans.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub billing_period: Option<f64>,
    /// Access duration in days for expiration-based plans. `null` for plans without an expiration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub expiration_days: Option<f64>,
    /// Plan ID, prefixed `plan_`.
    #[serde(default)]
    pub id: String,
    /// What checkout charges up front. `amount` is `"0.00"` when the first charge is free, such as a trial.
    #[serde(default)]
    pub initial_price: Money,
    /// Billing model for this plan: `one_time` or `renewal`.
    pub plan_type: ProductPublicPlanPlanType,
    /// The recurring charge every `billing_period` days. `amount` is `"0.00"` for one-time plans.
    #[serde(default)]
    pub renewal_price: Money,
    /// Plan display name shown to customers. `null` if no title has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Whether the plan has unlimited stock.
    #[serde(default)]
    pub unlimited_stock: bool,
    /// Where this plan can be seen. `visible` plans appear on the product page.
    pub visibility: ProductPublicPlanVisibility,
}

impl ProductPublicPlan {
    pub fn builder() -> ProductPublicPlanBuilder {
        <ProductPublicPlanBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductPublicPlanBuilder {
    billing_period: Option<f64>,
    expiration_days: Option<f64>,
    id: Option<String>,
    initial_price: Option<Money>,
    plan_type: Option<ProductPublicPlanPlanType>,
    renewal_price: Option<Money>,
    title: Option<String>,
    unlimited_stock: Option<bool>,
    visibility: Option<ProductPublicPlanVisibility>,
}

impl ProductPublicPlanBuilder {
    pub fn billing_period(mut self, value: f64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn expiration_days(mut self, value: f64) -> Self {
        self.expiration_days = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn initial_price(mut self, value: Money) -> Self {
        self.initial_price = Some(value);
        self
    }

    pub fn plan_type(mut self, value: ProductPublicPlanPlanType) -> Self {
        self.plan_type = Some(value);
        self
    }

    pub fn renewal_price(mut self, value: Money) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn unlimited_stock(mut self, value: bool) -> Self {
        self.unlimited_stock = Some(value);
        self
    }

    pub fn visibility(mut self, value: ProductPublicPlanVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProductPublicPlan`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProductPublicPlanBuilder::id)
    /// - [`initial_price`](ProductPublicPlanBuilder::initial_price)
    /// - [`plan_type`](ProductPublicPlanBuilder::plan_type)
    /// - [`renewal_price`](ProductPublicPlanBuilder::renewal_price)
    /// - [`unlimited_stock`](ProductPublicPlanBuilder::unlimited_stock)
    /// - [`visibility`](ProductPublicPlanBuilder::visibility)
    pub fn build(self) -> Result<ProductPublicPlan, BuildError> {
        Ok(ProductPublicPlan {
            billing_period: self.billing_period,
            expiration_days: self.expiration_days,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            initial_price: self
                .initial_price
                .ok_or_else(|| BuildError::missing_field("initial_price"))?,
            plan_type: self
                .plan_type
                .ok_or_else(|| BuildError::missing_field("plan_type"))?,
            renewal_price: self
                .renewal_price
                .ok_or_else(|| BuildError::missing_field("renewal_price"))?,
            title: self.title,
            unlimited_stock: self
                .unlimited_stock
                .ok_or_else(|| BuildError::missing_field("unlimited_stock"))?,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
        })
    }
}
