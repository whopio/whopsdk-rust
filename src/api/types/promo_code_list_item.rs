pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromoCodeListItem {
    /// Discount amount. Percentage discounts are represented as a decimal fraction.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount_off: f64,
    /// Whether the promo code is restricted to churned customers.
    #[serde(default)]
    pub churned_users_only: bool,
    /// Code entered at checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// When the promo code was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Currency used for a fixed-amount discount.
    pub currency: PromoCodeListItemCurrency,
    /// How long the discount applies.
    pub duration: PromoCodeListItemDuration,
    /// Whether the promo code applies only to existing memberships.
    #[serde(default)]
    pub existing_memberships_only: bool,
    /// When the promo code expires, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Promo code ID, prefixed `promo_`.
    #[serde(default)]
    pub id: String,
    /// Custom key-value metadata stored on the promo code.
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    /// Whether the promo code is restricted to new customers.
    #[serde(default)]
    pub new_users_only: bool,
    /// Whether each customer may redeem the promo code only once.
    #[serde(default)]
    pub one_per_customer: bool,
    /// Product the promo code is restricted to, or `null` when it is not product-scoped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<PromoCodeProduct>,
    /// Billing intervals the discount applies to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_duration_months: Option<i64>,
    /// Whether the discount is percentage-based or a fixed amount.
    pub promo_type: PromoCodeListItemPromoType,
    /// Promo code lifecycle status.
    pub status: PromoCodeListItemStatus,
    /// Maximum uses when stock is limited.
    #[serde(default)]
    pub stock: i64,
    /// Whether the promo code has no redemption limit.
    #[serde(default)]
    pub unlimited_stock: bool,
    /// When the promo code was updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// Memberships that used the promo code.
    #[serde(default)]
    pub uses: i64,
}

impl PromoCodeListItem {
    pub fn builder() -> PromoCodeListItemBuilder {
        <PromoCodeListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromoCodeListItemBuilder {
    amount_off: Option<f64>,
    churned_users_only: Option<bool>,
    code: Option<String>,
    created_at: Option<String>,
    currency: Option<PromoCodeListItemCurrency>,
    duration: Option<PromoCodeListItemDuration>,
    existing_memberships_only: Option<bool>,
    expires_at: Option<String>,
    id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    new_users_only: Option<bool>,
    one_per_customer: Option<bool>,
    product: Option<PromoCodeProduct>,
    promo_duration_months: Option<i64>,
    promo_type: Option<PromoCodeListItemPromoType>,
    status: Option<PromoCodeListItemStatus>,
    stock: Option<i64>,
    unlimited_stock: Option<bool>,
    updated_at: Option<String>,
    uses: Option<i64>,
}

impl PromoCodeListItemBuilder {
    pub fn amount_off(mut self, value: f64) -> Self {
        self.amount_off = Some(value);
        self
    }

    pub fn churned_users_only(mut self, value: bool) -> Self {
        self.churned_users_only = Some(value);
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn currency(mut self, value: PromoCodeListItemCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn duration(mut self, value: PromoCodeListItemDuration) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn existing_memberships_only(mut self, value: bool) -> Self {
        self.existing_memberships_only = Some(value);
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn new_users_only(mut self, value: bool) -> Self {
        self.new_users_only = Some(value);
        self
    }

    pub fn one_per_customer(mut self, value: bool) -> Self {
        self.one_per_customer = Some(value);
        self
    }

    pub fn product(mut self, value: PromoCodeProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn promo_duration_months(mut self, value: i64) -> Self {
        self.promo_duration_months = Some(value);
        self
    }

    pub fn promo_type(mut self, value: PromoCodeListItemPromoType) -> Self {
        self.promo_type = Some(value);
        self
    }

    pub fn status(mut self, value: PromoCodeListItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn stock(mut self, value: i64) -> Self {
        self.stock = Some(value);
        self
    }

    pub fn unlimited_stock(mut self, value: bool) -> Self {
        self.unlimited_stock = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn uses(mut self, value: i64) -> Self {
        self.uses = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromoCodeListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_off`](PromoCodeListItemBuilder::amount_off)
    /// - [`churned_users_only`](PromoCodeListItemBuilder::churned_users_only)
    /// - [`created_at`](PromoCodeListItemBuilder::created_at)
    /// - [`currency`](PromoCodeListItemBuilder::currency)
    /// - [`duration`](PromoCodeListItemBuilder::duration)
    /// - [`existing_memberships_only`](PromoCodeListItemBuilder::existing_memberships_only)
    /// - [`id`](PromoCodeListItemBuilder::id)
    /// - [`metadata`](PromoCodeListItemBuilder::metadata)
    /// - [`new_users_only`](PromoCodeListItemBuilder::new_users_only)
    /// - [`one_per_customer`](PromoCodeListItemBuilder::one_per_customer)
    /// - [`promo_type`](PromoCodeListItemBuilder::promo_type)
    /// - [`status`](PromoCodeListItemBuilder::status)
    /// - [`stock`](PromoCodeListItemBuilder::stock)
    /// - [`unlimited_stock`](PromoCodeListItemBuilder::unlimited_stock)
    /// - [`updated_at`](PromoCodeListItemBuilder::updated_at)
    /// - [`uses`](PromoCodeListItemBuilder::uses)
    pub fn build(self) -> Result<PromoCodeListItem, BuildError> {
        Ok(PromoCodeListItem {
            amount_off: self
                .amount_off
                .ok_or_else(|| BuildError::missing_field("amount_off"))?,
            churned_users_only: self
                .churned_users_only
                .ok_or_else(|| BuildError::missing_field("churned_users_only"))?,
            code: self.code,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            duration: self
                .duration
                .ok_or_else(|| BuildError::missing_field("duration"))?,
            existing_memberships_only: self
                .existing_memberships_only
                .ok_or_else(|| BuildError::missing_field("existing_memberships_only"))?,
            expires_at: self.expires_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self
                .metadata
                .ok_or_else(|| BuildError::missing_field("metadata"))?,
            new_users_only: self
                .new_users_only
                .ok_or_else(|| BuildError::missing_field("new_users_only"))?,
            one_per_customer: self
                .one_per_customer
                .ok_or_else(|| BuildError::missing_field("one_per_customer"))?,
            product: self.product,
            promo_duration_months: self.promo_duration_months,
            promo_type: self
                .promo_type
                .ok_or_else(|| BuildError::missing_field("promo_type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            stock: self
                .stock
                .ok_or_else(|| BuildError::missing_field("stock"))?,
            unlimited_stock: self
                .unlimited_stock
                .ok_or_else(|| BuildError::missing_field("unlimited_stock"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            uses: self.uses.ok_or_else(|| BuildError::missing_field("uses"))?,
        })
    }
}
