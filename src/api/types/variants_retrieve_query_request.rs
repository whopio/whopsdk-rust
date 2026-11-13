pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VariantsRetrieveQueryRequest {
    /// The currency to state each variant's prices in. Omit it, or send null, for each variant's own currency. `auto` states them in the currency of the country Whop places the buyer in: by `ip_address` when your server sends one, and by the request's own IP address otherwise. To choose that country instead, send `presentment_country` in place of this. A three-letter ISO 4217 code, such as `eur`, states them in that currency. A variant converts only where a payment quote for it would be priced in that currency, and stays in its own currency otherwise: a renewing variant, one with adaptive pricing off, a currency no payment method can collect, or `auto` for a buyer Whop cannot place in a country. Converted prices are an estimate at the current exchange rate. Nothing is held: the payment quote the buyer pays from fixes the rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentment_currency: Option<String>,
    /// The buyer's IP address, when your server reads on their behalf. Needs an API key; any other caller is placed by its own request. Where `presentment_currency` `auto` finds the buyer's local currency, and where a payment method must be able to collect it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The country to state each variant's prices for, as an ISO 3166-1 alpha-2 code such as `JP`, when the buyer chose one: a store's country picker. States them in the currency `auto` would for a buyer in that country, so send it instead of `presentment_currency`, never with it. Whether a payment method can collect that currency is still judged where the buyer is: by `ip_address`, else by the request's own IP address. A country Whop has no currency for is ignored, and the prices are stated as `auto` would state them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentment_country: Option<String>,
}

impl VariantsRetrieveQueryRequest {
    pub fn builder() -> VariantsRetrieveQueryRequestBuilder {
        <VariantsRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VariantsRetrieveQueryRequestBuilder {
    presentment_currency: Option<String>,
    ip_address: Option<String>,
    presentment_country: Option<String>,
}

impl VariantsRetrieveQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`VariantsRetrieveQueryRequest`].
    pub fn build(self) -> Result<VariantsRetrieveQueryRequest, BuildError> {
        Ok(VariantsRetrieveQueryRequest {
            presentment_currency: self.presentment_currency,
            ip_address: self.ip_address,
            presentment_country: self.presentment_country,
        })
    }
}
