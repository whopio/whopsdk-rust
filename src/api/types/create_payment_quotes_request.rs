pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePaymentQuotesRequest {
    /// The buyer's billing address. Where tax is calculated when no shipping address is given, and the address a tax registration belongs to. A seller that collects tax on this purchase needs the buyer located by a `country` here, on `shipping_address`, a `presentment_country`, or an `ip_address`; without one the quote is refused with `quote_location_required`. Only the keys you supply are kept. The payment that consumes the quote must put the buyer in the same place, by country, state and postal code, through its own `shipping_address` or its confirmation token's billing address, or it is refused with `quote_mismatch`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreatePaymentQuotesRequestAddress>,
    /// The buyer's IP address, when your server makes the call on their behalf. Locates the buyer when neither address carries a country and no `presentment_country` is sent. A quote located this way (`located_by` is `ip_address`) is a preview: a payment refuses it with `quote_preview_only`, so quote again with the buyer's address before paying. Also where `presentment_currency` `auto` and `recommended_currencies` find the buyer's local currency, and where a payment method must be able to collect it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// What the buyer is purchasing. Each entry names an existing variant by `variant_id`. Several entries form a cart, which requires every variant to be compatible, belong to this account, and use the same currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<CreatePaymentQuotesRequestLineItemsItem>>,
    /// The country to price the purchase for, as an ISO 3166-1 alpha-2 code such as `JP`, when the buyer chose one: a store's country picker. Prices in the currency `auto` would pick for a buyer in that country, so send it instead of `presentment_currency`, never with it. Whether a payment method can collect that currency is still judged where the buyer is: by `ip_address`, and by this country only without one. With no address that carries a country, tax is estimated for this country (`located_by` is `presentment_country`) at the country level, and a quote located this way is a preview a payment refuses with `quote_preview_only`. Send it only once the buyer has chosen a country: without it, an `ip_address` estimates more precisely, such as for a US state. For a country Whop has no currency for, the quote is priced as `auto` for the buyer's `ip_address`, and tax is still estimated for the country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentment_country: Option<String>,
    /// The currency to price and charge the purchase in. Omit it, or send null, to price in the variants' own currency. `auto` prices in the currency of the country Whop places the buyer's `ip_address` in when the purchase can be converted into it and a payment method can collect it, and in the variants' own currency otherwise, including when no `ip_address` is sent or Whop cannot place it in a country. To choose that country instead, send `presentment_country` in place of this. A three-letter ISO 4217 code, such as `eur`, prices in that currency or is refused: with `presentment_currency_unsupported` when the purchase cannot be converted into it (adaptive pricing is off for the variant, the variant is not a one-time purchase, or `plan` describes a variant that does not exist yet), and with `presentment_currency_not_payable` when no payment method can collect it. A converted quote states every amount in this currency at an `exchange_rate` fixed until `expires_at`, and the payment that consumes it is charged in this currency at that rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentment_currency: Option<String>,
    /// Where physical goods ship. When present it is where tax is calculated; omit it for digital goods. Only the keys you supply are kept. The payment that consumes the quote must ship to the same place, by country, state and postal code, or it is refused with `quote_mismatch`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<CreatePaymentQuotesRequestShippingAddress>,
    /// The buyer's tax registration, for a business purchase. One entry. Prices the purchase as business-to-business where that applies (EU reverse charge, for one) and requires an `address` to belong to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<CreatePaymentQuotesRequestTaxIdsItem>>,
    /// The account the purchase belongs to, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// The promo code as the buyer typed it, matched within the account regardless of case and surrounding spaces, as checkout matches it. It must be valid for the variant. Send it or `promo_code_id`, not both; an empty or whitespace-only string counts as not sent. A code the account does not have, or one that is no longer active, is refused before anything is written, with the error code `promo_invalid`. A code this purchase cannot use, such as one with no uses left or one restricted to other variants, products or buyers, is refused with `promo_invalid` too, and the error's message says why.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code: Option<String>,
    /// An active promo code to apply, prefixed `promo_`. It must belong to the account and be valid for the variant. Send it or `promo_code`, not both. A code this purchase cannot use is refused with the error code `promo_invalid`, and the error's message says why.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code_id: Option<String>,
    /// The variant purchased, described by its attributes instead of an id: the variant with exactly these attributes is used, and one is created when none exists. Mutually exclusive with `variant_id` and `line_items`. Creating a variant requires `plan:create`; creating or updating a product requires the corresponding product permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<PaymentInputVariant>,
    /// The variant purchased, prefixed `plan_`. It must belong to the account. Mutually exclusive with `variant` and `line_items`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
}

impl CreatePaymentQuotesRequest {
    pub fn builder() -> CreatePaymentQuotesRequestBuilder {
        <CreatePaymentQuotesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentQuotesRequestBuilder {
    address: Option<CreatePaymentQuotesRequestAddress>,
    ip_address: Option<String>,
    line_items: Option<Vec<CreatePaymentQuotesRequestLineItemsItem>>,
    presentment_country: Option<String>,
    presentment_currency: Option<String>,
    shipping_address: Option<CreatePaymentQuotesRequestShippingAddress>,
    tax_ids: Option<Vec<CreatePaymentQuotesRequestTaxIdsItem>>,
    account_id: Option<String>,
    promo_code: Option<String>,
    promo_code_id: Option<String>,
    variant: Option<PaymentInputVariant>,
    variant_id: Option<String>,
}

impl CreatePaymentQuotesRequestBuilder {
    pub fn address(mut self, value: CreatePaymentQuotesRequestAddress) -> Self {
        self.address = Some(value);
        self
    }

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    pub fn line_items(mut self, value: Vec<CreatePaymentQuotesRequestLineItemsItem>) -> Self {
        self.line_items = Some(value);
        self
    }

    pub fn presentment_country(mut self, value: impl Into<String>) -> Self {
        self.presentment_country = Some(value.into());
        self
    }

    pub fn presentment_currency(mut self, value: impl Into<String>) -> Self {
        self.presentment_currency = Some(value.into());
        self
    }

    pub fn shipping_address(mut self, value: CreatePaymentQuotesRequestShippingAddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    pub fn tax_ids(mut self, value: Vec<CreatePaymentQuotesRequestTaxIdsItem>) -> Self {
        self.tax_ids = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn promo_code(mut self, value: impl Into<String>) -> Self {
        self.promo_code = Some(value.into());
        self
    }

    pub fn promo_code_id(mut self, value: impl Into<String>) -> Self {
        self.promo_code_id = Some(value.into());
        self
    }

    pub fn variant(mut self, value: PaymentInputVariant) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentQuotesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreatePaymentQuotesRequestBuilder::account_id)
    pub fn build(self) -> Result<CreatePaymentQuotesRequest, BuildError> {
        Ok(CreatePaymentQuotesRequest {
            address: self.address,
            ip_address: self.ip_address,
            line_items: self.line_items,
            presentment_country: self.presentment_country,
            presentment_currency: self.presentment_currency,
            shipping_address: self.shipping_address,
            tax_ids: self.tax_ids,
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            promo_code: self.promo_code,
            promo_code_id: self.promo_code_id,
            variant: self.variant,
            variant_id: self.variant_id,
        })
    }
}
