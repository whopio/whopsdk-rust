pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentMethodDomain {
    /// ID of the account the domain is registered for, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// When the domain was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Hostname the checkout is served from (e.g. `checkout.example.com`).
    #[serde(default)]
    pub hostname: String,
    /// Payment method domain ID, prefixed `pmd_`.
    #[serde(default)]
    pub id: String,
    /// Wallet provider the domain is registered with.
    pub provider: PaymentMethodDomainProvider,
    /// Verification status. `pending` means the provider could not fetch the domain-association file yet; only `verified` domains show wallet payment methods at checkout.
    pub status: PaymentMethodDomainStatus,
    /// When the domain was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl PaymentMethodDomain {
    pub fn builder() -> PaymentMethodDomainBuilder {
        <PaymentMethodDomainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodDomainBuilder {
    account_id: Option<String>,
    created_at: Option<String>,
    hostname: Option<String>,
    id: Option<String>,
    provider: Option<PaymentMethodDomainProvider>,
    status: Option<PaymentMethodDomainStatus>,
    updated_at: Option<String>,
}

impl PaymentMethodDomainBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn provider(mut self, value: PaymentMethodDomainProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn status(mut self, value: PaymentMethodDomainStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodDomain`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](PaymentMethodDomainBuilder::created_at)
    /// - [`hostname`](PaymentMethodDomainBuilder::hostname)
    /// - [`id`](PaymentMethodDomainBuilder::id)
    /// - [`provider`](PaymentMethodDomainBuilder::provider)
    /// - [`status`](PaymentMethodDomainBuilder::status)
    /// - [`updated_at`](PaymentMethodDomainBuilder::updated_at)
    pub fn build(self) -> Result<PaymentMethodDomain, BuildError> {
        Ok(PaymentMethodDomain {
            account_id: self.account_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            hostname: self
                .hostname
                .ok_or_else(|| BuildError::missing_field("hostname"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
