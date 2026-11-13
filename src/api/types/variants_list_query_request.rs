pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VariantsListQueryRequest {
    /// The unique identifier of the account to list variants for. Required unless `product_ids` is provided for a public product-variant read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The sort direction for results. Defaults to descending.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListVariantsRequestDirection>,
    /// The field to sort results by. Defaults to created_at.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListVariantsRequestOrder>,
    /// Filter to only variants matching these release methods.
    #[serde(default)]
    pub release_methods: Vec<Option<String>>,
    /// Filter to only variants matching these visibility states.
    #[serde(default)]
    pub visibilities: Vec<Option<String>>,
    /// Filter to only variants matching these billing types.
    #[serde(default)]
    pub billing_types: Vec<Option<String>>,
    /// Filter to only variants belonging to these product identifiers. When `account_id` is omitted, this is required and the response is publicly readable: only visible, non-invoice variants are returned.
    #[serde(default)]
    pub product_ids: Vec<Option<String>>,
    /// Only return variants created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only return variants created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// The currency to state each variant's prices in. Omit it, or send null, for each variant's own currency. `auto` states them in the currency of the country Whop places the buyer in: by `ip_address` when your server sends one, and by the request's own IP address otherwise. To choose that country instead, send `presentment_country` in place of this. A three-letter ISO 4217 code, such as `eur`, states them in that currency. A variant converts only where a payment quote for it would be priced in that currency, and stays in its own currency otherwise: a renewing variant, one with adaptive pricing off, a currency no payment method can collect, or `auto` for a buyer Whop cannot place in a country. Converted prices are an estimate at the current exchange rate. Nothing is held: the payment quote the buyer pays from fixes the rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentment_currency: Option<String>,
    /// The buyer's IP address, when your server reads on their behalf. Needs an API key; any other caller is placed by its own request. Where `presentment_currency` `auto` finds the buyer's local currency, and where a payment method must be able to collect it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The country to state each variant's prices for, as an ISO 3166-1 alpha-2 code such as `JP`, when the buyer chose one: a store's country picker. States them in the currency `auto` would for a buyer in that country, so send it instead of `presentment_currency`, never with it. Whether a payment method can collect that currency is still judged where the buyer is: by `ip_address`, else by the request's own IP address. A country Whop has no currency for is ignored, and the prices are stated as `auto` would state them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentment_country: Option<String>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl VariantsListQueryRequest {
    pub fn builder() -> VariantsListQueryRequestBuilder {
        <VariantsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VariantsListQueryRequestBuilder {
    account_id: Option<String>,
    direction: Option<ListVariantsRequestDirection>,
    order: Option<ListVariantsRequestOrder>,
    release_methods: Option<Vec<Option<String>>>,
    visibilities: Option<Vec<Option<String>>>,
    billing_types: Option<Vec<Option<String>>>,
    product_ids: Option<Vec<Option<String>>>,
    created_before: Option<String>,
    created_after: Option<String>,
    presentment_currency: Option<String>,
    ip_address: Option<String>,
    presentment_country: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl VariantsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn direction(mut self, value: ListVariantsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn order(mut self, value: ListVariantsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn release_methods(mut self, value: Vec<Option<String>>) -> Self {
        self.release_methods = Some(value);
        self
    }

    pub fn visibilities(mut self, value: Vec<Option<String>>) -> Self {
        self.visibilities = Some(value);
        self
    }

    pub fn billing_types(mut self, value: Vec<Option<String>>) -> Self {
        self.billing_types = Some(value);
        self
    }

    pub fn product_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.product_ids = Some(value);
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn presentment_currency(mut self, value: impl Into<String>) -> Self {
        self.presentment_currency = Some(value.into());
        self
    }

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    pub fn presentment_country(mut self, value: impl Into<String>) -> Self {
        self.presentment_country = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VariantsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`release_methods`](VariantsListQueryRequestBuilder::release_methods)
    /// - [`visibilities`](VariantsListQueryRequestBuilder::visibilities)
    /// - [`billing_types`](VariantsListQueryRequestBuilder::billing_types)
    /// - [`product_ids`](VariantsListQueryRequestBuilder::product_ids)
    pub fn build(self) -> Result<VariantsListQueryRequest, BuildError> {
        Ok(VariantsListQueryRequest {
            account_id: self.account_id,
            direction: self.direction,
            order: self.order,
            release_methods: self
                .release_methods
                .ok_or_else(|| BuildError::missing_field("release_methods"))?,
            visibilities: self
                .visibilities
                .ok_or_else(|| BuildError::missing_field("visibilities"))?,
            billing_types: self
                .billing_types
                .ok_or_else(|| BuildError::missing_field("billing_types"))?,
            product_ids: self
                .product_ids
                .ok_or_else(|| BuildError::missing_field("product_ids"))?,
            created_before: self.created_before,
            created_after: self.created_after,
            presentment_currency: self.presentment_currency,
            ip_address: self.ip_address,
            presentment_country: self.presentment_country,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
