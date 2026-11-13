pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProductPublicVariant {
    /// Number of days between recurring charges, such as 30 for monthly or 365 for annual. `null` for one-time variants.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub billing_period: Option<f64>,
    /// Billing model for this variant: `one_time` or `renewal`.
    pub billing_type: ProductPublicVariantBillingType,
    /// Access duration in days for expiration-based variants. `null` for variants without an expiration.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub expiration_days: Option<f64>,
    /// Variant ID, prefixed `plan_`.
    #[serde(default)]
    pub id: String,
    /// What checkout charges up front. `amount` is `"0.00"` when the first charge is free, such as a trial.
    #[serde(default)]
    pub initial_price: Money,
    /// The recurring charge every `billing_period` days. `amount` is `"0.00"` for one-time variants.
    #[serde(default)]
    pub renewal_price: Money,
    /// Variant display name shown to customers. `null` if no title has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Free trial days before the first renewal charge. `null` if no trial is configured or the signed-in user has already used a trial on this variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub trial_period_days: Option<f64>,
    /// Whether the variant has unlimited stock.
    #[serde(default)]
    pub unlimited_stock: bool,
    /// Where this variant can be seen. `visible` variants appear on the product page.
    pub visibility: ProductPublicVariantVisibility,
}

impl ProductPublicVariant {
    pub fn builder() -> ProductPublicVariantBuilder {
        <ProductPublicVariantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductPublicVariantBuilder {
    billing_period: Option<f64>,
    billing_type: Option<ProductPublicVariantBillingType>,
    expiration_days: Option<f64>,
    id: Option<String>,
    initial_price: Option<Money>,
    renewal_price: Option<Money>,
    title: Option<String>,
    trial_period_days: Option<f64>,
    unlimited_stock: Option<bool>,
    visibility: Option<ProductPublicVariantVisibility>,
}

impl ProductPublicVariantBuilder {
    pub fn billing_period(mut self, value: f64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn billing_type(mut self, value: ProductPublicVariantBillingType) -> Self {
        self.billing_type = Some(value);
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

    pub fn renewal_price(mut self, value: Money) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn trial_period_days(mut self, value: f64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    pub fn unlimited_stock(mut self, value: bool) -> Self {
        self.unlimited_stock = Some(value);
        self
    }

    pub fn visibility(mut self, value: ProductPublicVariantVisibility) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProductPublicVariant`].
    /// This method will fail if any of the following fields are not set:
    /// - [`billing_type`](ProductPublicVariantBuilder::billing_type)
    /// - [`id`](ProductPublicVariantBuilder::id)
    /// - [`initial_price`](ProductPublicVariantBuilder::initial_price)
    /// - [`renewal_price`](ProductPublicVariantBuilder::renewal_price)
    /// - [`unlimited_stock`](ProductPublicVariantBuilder::unlimited_stock)
    /// - [`visibility`](ProductPublicVariantBuilder::visibility)
    pub fn build(self) -> Result<ProductPublicVariant, BuildError> {
        Ok(ProductPublicVariant {
            billing_period: self.billing_period,
            billing_type: self
                .billing_type
                .ok_or_else(|| BuildError::missing_field("billing_type"))?,
            expiration_days: self.expiration_days,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            initial_price: self
                .initial_price
                .ok_or_else(|| BuildError::missing_field("initial_price"))?,
            renewal_price: self
                .renewal_price
                .ok_or_else(|| BuildError::missing_field("renewal_price"))?,
            title: self.title,
            trial_period_days: self.trial_period_days,
            unlimited_stock: self
                .unlimited_stock
                .ok_or_else(|| BuildError::missing_field("unlimited_stock"))?,
            visibility: self
                .visibility
                .ok_or_else(|| BuildError::missing_field("visibility"))?,
        })
    }
}
