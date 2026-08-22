pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FormCompanyAccountsResponse {
    /// Checkout session ID, prefixed `ch_`.
    #[serde(default)]
    pub checkout_session_id: String,
    /// Hosted checkout URL. Send the buyer here to pay for the formation; the filing is submitted once payment completes.
    #[serde(default)]
    pub checkout_url: String,
    /// Always `usd`.
    #[serde(default)]
    pub currency: String,
    /// Total due at checkout in USD cents.
    #[serde(default)]
    pub total: i64,
}

impl FormCompanyAccountsResponse {
    pub fn builder() -> FormCompanyAccountsResponseBuilder {
        <FormCompanyAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FormCompanyAccountsResponseBuilder {
    checkout_session_id: Option<String>,
    checkout_url: Option<String>,
    currency: Option<String>,
    total: Option<i64>,
}

impl FormCompanyAccountsResponseBuilder {
    pub fn checkout_session_id(mut self, value: impl Into<String>) -> Self {
        self.checkout_session_id = Some(value.into());
        self
    }

    pub fn checkout_url(mut self, value: impl Into<String>) -> Self {
        self.checkout_url = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FormCompanyAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`checkout_session_id`](FormCompanyAccountsResponseBuilder::checkout_session_id)
    /// - [`checkout_url`](FormCompanyAccountsResponseBuilder::checkout_url)
    /// - [`currency`](FormCompanyAccountsResponseBuilder::currency)
    /// - [`total`](FormCompanyAccountsResponseBuilder::total)
    pub fn build(self) -> Result<FormCompanyAccountsResponse, BuildError> {
        Ok(FormCompanyAccountsResponse {
            checkout_session_id: self
                .checkout_session_id
                .ok_or_else(|| BuildError::missing_field("checkout_session_id"))?,
            checkout_url: self
                .checkout_url
                .ok_or_else(|| BuildError::missing_field("checkout_url"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
