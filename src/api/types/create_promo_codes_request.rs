pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePromoCodesRequest {
    #[serde(default)]
    pub account_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount_off: f64,
    pub base_currency: CreatePromoCodesRequestBaseCurrency,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub churned_users_only: Option<bool>,
    #[serde(default)]
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_memberships_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(default)]
    pub new_users_only: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_per_customer: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default)]
    pub promo_duration_months: i64,
    pub promo_type: CreatePromoCodesRequestPromoType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stock: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unlimited_stock: Option<bool>,
}

impl CreatePromoCodesRequest {
    pub fn builder() -> CreatePromoCodesRequestBuilder {
        <CreatePromoCodesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePromoCodesRequestBuilder {
    account_id: Option<String>,
    amount_off: Option<f64>,
    base_currency: Option<CreatePromoCodesRequestBaseCurrency>,
    churned_users_only: Option<bool>,
    code: Option<String>,
    existing_memberships_only: Option<bool>,
    expires_at: Option<String>,
    new_users_only: Option<bool>,
    one_per_customer: Option<bool>,
    plan_ids: Option<Vec<String>>,
    product_id: Option<String>,
    promo_duration_months: Option<i64>,
    promo_type: Option<CreatePromoCodesRequestPromoType>,
    stock: Option<i64>,
    unlimited_stock: Option<bool>,
}

impl CreatePromoCodesRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount_off(mut self, value: f64) -> Self {
        self.amount_off = Some(value);
        self
    }

    pub fn base_currency(mut self, value: CreatePromoCodesRequestBaseCurrency) -> Self {
        self.base_currency = Some(value);
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

    pub fn existing_memberships_only(mut self, value: bool) -> Self {
        self.existing_memberships_only = Some(value);
        self
    }

    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
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

    pub fn plan_ids(mut self, value: Vec<String>) -> Self {
        self.plan_ids = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn promo_duration_months(mut self, value: i64) -> Self {
        self.promo_duration_months = Some(value);
        self
    }

    pub fn promo_type(mut self, value: CreatePromoCodesRequestPromoType) -> Self {
        self.promo_type = Some(value);
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

    /// Consumes the builder and constructs a [`CreatePromoCodesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreatePromoCodesRequestBuilder::account_id)
    /// - [`amount_off`](CreatePromoCodesRequestBuilder::amount_off)
    /// - [`base_currency`](CreatePromoCodesRequestBuilder::base_currency)
    /// - [`code`](CreatePromoCodesRequestBuilder::code)
    /// - [`new_users_only`](CreatePromoCodesRequestBuilder::new_users_only)
    /// - [`promo_duration_months`](CreatePromoCodesRequestBuilder::promo_duration_months)
    /// - [`promo_type`](CreatePromoCodesRequestBuilder::promo_type)
    pub fn build(self) -> Result<CreatePromoCodesRequest, BuildError> {
        Ok(CreatePromoCodesRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            amount_off: self
                .amount_off
                .ok_or_else(|| BuildError::missing_field("amount_off"))?,
            base_currency: self
                .base_currency
                .ok_or_else(|| BuildError::missing_field("base_currency"))?,
            churned_users_only: self.churned_users_only,
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            existing_memberships_only: self.existing_memberships_only,
            expires_at: self.expires_at,
            new_users_only: self
                .new_users_only
                .ok_or_else(|| BuildError::missing_field("new_users_only"))?,
            one_per_customer: self.one_per_customer,
            plan_ids: self.plan_ids,
            product_id: self.product_id,
            promo_duration_months: self
                .promo_duration_months
                .ok_or_else(|| BuildError::missing_field("promo_duration_months"))?,
            promo_type: self
                .promo_type
                .ok_or_else(|| BuildError::missing_field("promo_type"))?,
            stock: self.stock,
            unlimited_stock: self.unlimited_stock,
        })
    }
}
