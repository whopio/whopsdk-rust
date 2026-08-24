pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePaymentMethodDomainsRequest {
    /// Account to register the domain for (`biz_` tag). Defaults to the caller's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Hostname to register (e.g. `checkout.shinetime.example`).
    #[serde(default)]
    pub hostname: String,
}

impl CreatePaymentMethodDomainsRequest {
    pub fn builder() -> CreatePaymentMethodDomainsRequestBuilder {
        <CreatePaymentMethodDomainsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentMethodDomainsRequestBuilder {
    account_id: Option<String>,
    hostname: Option<String>,
}

impl CreatePaymentMethodDomainsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentMethodDomainsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`hostname`](CreatePaymentMethodDomainsRequestBuilder::hostname)
    pub fn build(self) -> Result<CreatePaymentMethodDomainsRequest, BuildError> {
        Ok(CreatePaymentMethodDomainsRequest {
            account_id: self.account_id,
            hostname: self
                .hostname
                .ok_or_else(|| BuildError::missing_field("hostname"))?,
        })
    }
}
