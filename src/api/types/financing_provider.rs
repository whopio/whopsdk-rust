pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinancingProvider {
    /// The merchant action that turns this provider on when `status` is `needs_action`; `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<FinancingProviderAction>,
    #[serde(default)]
    pub currencies: Vec<FinancingProviderCurrency>,
    /// The dispute rates behind a `dispute_rate_near_limit` or `dispute_rate_exceeded` reason; `null` for every other reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<FinancingProviderDetails>,
    /// The financing provider.
    pub provider: FinancingProviderProvider,
    /// When the provider received or decided on this account, as an ISO 8601 timestamp. Set only for providers that review merchants themselves (Splitit); `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_reviewed_at: Option<String>,
    /// Why the provider has this status, when known. On `active` it is `dispute_rate_near_limit` when the provider is close to being turned off. On `inactive` it names what keeps it off, including `not_included` when the account's approval doesn't cover it, `turned_off` when it was switched off after setup, `setup_failed`, and `declined` when the provider turned the account down without a reason Whop recognizes. More reasons may be added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<FinancingProviderReason>,
    /// Where the provider stands for this account: `eligible` (included if the account applies), `needs_action` (waiting on the merchant, see `action`), `in_review` (waiting on Whop's review, or on the provider's own when `reason` is `provider_review`), `active` (offered at checkout), or `inactive` (not offered, see `reason`).
    pub status: FinancingProviderStatus,
}

impl FinancingProvider {
    pub fn builder() -> FinancingProviderBuilder {
        <FinancingProviderBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FinancingProviderBuilder {
    action: Option<FinancingProviderAction>,
    currencies: Option<Vec<FinancingProviderCurrency>>,
    details: Option<FinancingProviderDetails>,
    provider: Option<FinancingProviderProvider>,
    provider_reviewed_at: Option<String>,
    reason: Option<FinancingProviderReason>,
    status: Option<FinancingProviderStatus>,
}

impl FinancingProviderBuilder {
    pub fn action(mut self, value: FinancingProviderAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn currencies(mut self, value: Vec<FinancingProviderCurrency>) -> Self {
        self.currencies = Some(value);
        self
    }

    pub fn details(mut self, value: FinancingProviderDetails) -> Self {
        self.details = Some(value);
        self
    }

    pub fn provider(mut self, value: FinancingProviderProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn provider_reviewed_at(mut self, value: impl Into<String>) -> Self {
        self.provider_reviewed_at = Some(value.into());
        self
    }

    pub fn reason(mut self, value: FinancingProviderReason) -> Self {
        self.reason = Some(value);
        self
    }

    pub fn status(mut self, value: FinancingProviderStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FinancingProvider`].
    /// This method will fail if any of the following fields are not set:
    /// - [`currencies`](FinancingProviderBuilder::currencies)
    /// - [`provider`](FinancingProviderBuilder::provider)
    /// - [`status`](FinancingProviderBuilder::status)
    pub fn build(self) -> Result<FinancingProvider, BuildError> {
        Ok(FinancingProvider {
            action: self.action,
            currencies: self
                .currencies
                .ok_or_else(|| BuildError::missing_field("currencies"))?,
            details: self.details,
            provider: self
                .provider
                .ok_or_else(|| BuildError::missing_field("provider"))?,
            provider_reviewed_at: self.provider_reviewed_at,
            reason: self.reason,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
