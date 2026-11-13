pub use crate::prelude::*;

/// Find or create the variant for this entry by its attributes. Unknown properties return an error. Creating a variant requires `plan:create`, and creating or updating a product requires the matching product permission.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreatePaymentsRequestLineItemsItemVariant {
    /// Whether more than one unit can be purchased in an order. Renewal variants cannot enable it. When `false`, checkouts, carts, and invoices reject a quantity above 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_multiple_quantity: Option<bool>,
    /// Application fee the platform collects per unit, in the variant currency (5.00 means $5.00 for USD). Must be positive and below the initial price for one-time variants or renewal price for recurring variants. Paid to the parent account alongside other processing fees; collection is capped to remaining proceeds. Applies to subsequent payments on recurring variants. Only valid for subaccounts with a parent account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<f64>,
    /// Recurring billing interval in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_period: Option<i64>,
    /// Billing model for the variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_type: Option<CreatePaymentsRequestLineItemsItemVariantBillingType>,
    /// Currency code for the variant prices.
    pub currency: CreatePaymentsRequestLineItemsItemVariantCurrency,
    /// Variant description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Days until access expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_days: Option<i64>,
    /// Create a new variant instead of reusing a matching variant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_create_new_plan: Option<bool>,
    /// Additional amount charged on the first purchase, in the variant currency. For recurring variants without a trial, the first charge includes this amount plus renewal_price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initial_price: Option<f64>,
    /// Internal notes for the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_notes: Option<String>,
    /// Whether the variant's price includes tax: `inclusive` (tax is included in the price), `exclusive` (tax is added on top), or `unspecified`. Omit it or pass `null` to use the account's tax setting. On an account enrolled in the Whop tax service, `inclusive` or `exclusive` replaces the account's setting and `unspecified` keeps it. Otherwise it applies only where the account is the merchant of record: `inclusive` or `exclusive` replaces the account's setting when the account collects tax, and `unspecified` means the variant collects no tax. Where Whop is the merchant of record, the account's setting applies. Variants that differ only in this value are different variants.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_tax_type: Option<CreatePaymentsRequestLineItemsItemVariantOverrideTaxType>,
    /// Find or create a product by external identifier. Mutually exclusive with product_id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<CreatePaymentsRequestLineItemsItemVariantProduct>,
    /// Existing product ID belonging to the account, prefixed `prod_`. Mutually exclusive with `product`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Recurring price in the variant currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renewal_price: Option<f64>,
    /// How many payments a recurring variant collects before it stops billing, from 2 to 115. Only valid for `renewal` variants. `null` bills until the subscription is canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_pay_required_payments: Option<i64>,
    /// Variant title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Free trial days before renewal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    /// Whether the variant is visible to customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<CreatePaymentsRequestLineItemsItemVariantVisibility>,
}

impl CreatePaymentsRequestLineItemsItemVariant {
    pub fn builder() -> CreatePaymentsRequestLineItemsItemVariantBuilder {
        <CreatePaymentsRequestLineItemsItemVariantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentsRequestLineItemsItemVariantBuilder {
    allow_multiple_quantity: Option<bool>,
    application_fee_amount: Option<f64>,
    billing_period: Option<i64>,
    billing_type: Option<CreatePaymentsRequestLineItemsItemVariantBillingType>,
    currency: Option<CreatePaymentsRequestLineItemsItemVariantCurrency>,
    description: Option<String>,
    expiration_days: Option<i64>,
    force_create_new_plan: Option<bool>,
    initial_price: Option<f64>,
    internal_notes: Option<String>,
    override_tax_type: Option<CreatePaymentsRequestLineItemsItemVariantOverrideTaxType>,
    product: Option<CreatePaymentsRequestLineItemsItemVariantProduct>,
    product_id: Option<String>,
    renewal_price: Option<f64>,
    split_pay_required_payments: Option<i64>,
    title: Option<String>,
    trial_period_days: Option<i64>,
    visibility: Option<CreatePaymentsRequestLineItemsItemVariantVisibility>,
}

impl CreatePaymentsRequestLineItemsItemVariantBuilder {
    pub fn allow_multiple_quantity(mut self, value: bool) -> Self {
        self.allow_multiple_quantity = Some(value);
        self
    }

    pub fn application_fee_amount(mut self, value: f64) -> Self {
        self.application_fee_amount = Some(value);
        self
    }

    pub fn billing_period(mut self, value: i64) -> Self {
        self.billing_period = Some(value);
        self
    }

    pub fn billing_type(
        mut self,
        value: CreatePaymentsRequestLineItemsItemVariantBillingType,
    ) -> Self {
        self.billing_type = Some(value);
        self
    }

    pub fn currency(mut self, value: CreatePaymentsRequestLineItemsItemVariantCurrency) -> Self {
        self.currency = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn expiration_days(mut self, value: i64) -> Self {
        self.expiration_days = Some(value);
        self
    }

    pub fn force_create_new_plan(mut self, value: bool) -> Self {
        self.force_create_new_plan = Some(value);
        self
    }

    pub fn initial_price(mut self, value: f64) -> Self {
        self.initial_price = Some(value);
        self
    }

    pub fn internal_notes(mut self, value: impl Into<String>) -> Self {
        self.internal_notes = Some(value.into());
        self
    }

    pub fn override_tax_type(
        mut self,
        value: CreatePaymentsRequestLineItemsItemVariantOverrideTaxType,
    ) -> Self {
        self.override_tax_type = Some(value);
        self
    }

    pub fn product(mut self, value: CreatePaymentsRequestLineItemsItemVariantProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn renewal_price(mut self, value: f64) -> Self {
        self.renewal_price = Some(value);
        self
    }

    pub fn split_pay_required_payments(mut self, value: i64) -> Self {
        self.split_pay_required_payments = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn trial_period_days(mut self, value: i64) -> Self {
        self.trial_period_days = Some(value);
        self
    }

    pub fn visibility(
        mut self,
        value: CreatePaymentsRequestLineItemsItemVariantVisibility,
    ) -> Self {
        self.visibility = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentsRequestLineItemsItemVariant`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currency`](CreatePaymentsRequestLineItemsItemVariantBuilder::currency)
    pub fn build(self) -> Result<CreatePaymentsRequestLineItemsItemVariant, BuildError> {
        Ok(CreatePaymentsRequestLineItemsItemVariant {
            allow_multiple_quantity: self.allow_multiple_quantity,
            application_fee_amount: self.application_fee_amount,
            billing_period: self.billing_period,
            billing_type: self.billing_type,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            description: self.description,
            expiration_days: self.expiration_days,
            force_create_new_plan: self.force_create_new_plan,
            initial_price: self.initial_price,
            internal_notes: self.internal_notes,
            override_tax_type: self.override_tax_type,
            product: self.product,
            product_id: self.product_id,
            renewal_price: self.renewal_price,
            split_pay_required_payments: self.split_pay_required_payments,
            title: self.title,
            trial_period_days: self.trial_period_days,
            visibility: self.visibility,
        })
    }
}
