pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePreferencesResponse {
    /// The account's Whop Ads services and payment authorization agreement. While `pending_signature`, campaign launch is blocked; sign by answering `requested_information` via `PATCH /verifications/{id}`.
    pub ads_agreement: RetrievePreferencesResponseAdsAgreement,
    /// How the account pays for Whop Ads spend. `primary` is charged first; `backup` covers the charge when the primary fails. `null` until ads billing has been configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_payment_methods: Option<RetrievePreferencesResponseAdsPaymentMethods>,
    /// Lowercase ISO currency code, such as `usd` or `eur`, used to display ad spend and stats. Defaults to `usd`.
    #[serde(default)]
    pub ads_reporting_currency: String,
    /// IANA timezone (e.g. `America/New_York`) used to interpret campaign start/end times and to bucket reports. Defaults to `America/New_York` until explicitly overridden.
    #[serde(default)]
    pub ads_scheduling_timezone: String,
    /// The account's Triple Whale integration, which pushes Whop ad spend to Triple Whale's Data-In API so it reports as a `whop` channel.
    pub ads_triple_whale_integration: RetrievePreferencesResponseAdsTripleWhaleIntegration,
    /// Whether incoming funds are automatically moved to the account's cards balance. `false` when the account has no cards balance.
    #[serde(default)]
    pub cards_auto_top_up: bool,
    /// Whether Whop assembles and files the evidence response when this account's payments are disputed. Off by default; enabling it also opts the account into the success fee charged only on disputes it wins.
    #[serde(default)]
    pub dispute_fighter_enabled: bool,
}

impl RetrievePreferencesResponse {
    pub fn builder() -> RetrievePreferencesResponseBuilder {
        <RetrievePreferencesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePreferencesResponseBuilder {
    ads_agreement: Option<RetrievePreferencesResponseAdsAgreement>,
    ads_payment_methods: Option<RetrievePreferencesResponseAdsPaymentMethods>,
    ads_reporting_currency: Option<String>,
    ads_scheduling_timezone: Option<String>,
    ads_triple_whale_integration: Option<RetrievePreferencesResponseAdsTripleWhaleIntegration>,
    cards_auto_top_up: Option<bool>,
    dispute_fighter_enabled: Option<bool>,
}

impl RetrievePreferencesResponseBuilder {
    pub fn ads_agreement(mut self, value: RetrievePreferencesResponseAdsAgreement) -> Self {
        self.ads_agreement = Some(value);
        self
    }

    pub fn ads_payment_methods(
        mut self,
        value: RetrievePreferencesResponseAdsPaymentMethods,
    ) -> Self {
        self.ads_payment_methods = Some(value);
        self
    }

    pub fn ads_reporting_currency(mut self, value: impl Into<String>) -> Self {
        self.ads_reporting_currency = Some(value.into());
        self
    }

    pub fn ads_scheduling_timezone(mut self, value: impl Into<String>) -> Self {
        self.ads_scheduling_timezone = Some(value.into());
        self
    }

    pub fn ads_triple_whale_integration(
        mut self,
        value: RetrievePreferencesResponseAdsTripleWhaleIntegration,
    ) -> Self {
        self.ads_triple_whale_integration = Some(value);
        self
    }

    pub fn cards_auto_top_up(mut self, value: bool) -> Self {
        self.cards_auto_top_up = Some(value);
        self
    }

    pub fn dispute_fighter_enabled(mut self, value: bool) -> Self {
        self.dispute_fighter_enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePreferencesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ads_agreement`](RetrievePreferencesResponseBuilder::ads_agreement)
    /// - [`ads_reporting_currency`](RetrievePreferencesResponseBuilder::ads_reporting_currency)
    /// - [`ads_scheduling_timezone`](RetrievePreferencesResponseBuilder::ads_scheduling_timezone)
    /// - [`ads_triple_whale_integration`](RetrievePreferencesResponseBuilder::ads_triple_whale_integration)
    /// - [`cards_auto_top_up`](RetrievePreferencesResponseBuilder::cards_auto_top_up)
    /// - [`dispute_fighter_enabled`](RetrievePreferencesResponseBuilder::dispute_fighter_enabled)
    pub fn build(self) -> Result<RetrievePreferencesResponse, BuildError> {
        Ok(RetrievePreferencesResponse {
            ads_agreement: self
                .ads_agreement
                .ok_or_else(|| BuildError::missing_field("ads_agreement"))?,
            ads_payment_methods: self.ads_payment_methods,
            ads_reporting_currency: self
                .ads_reporting_currency
                .ok_or_else(|| BuildError::missing_field("ads_reporting_currency"))?,
            ads_scheduling_timezone: self
                .ads_scheduling_timezone
                .ok_or_else(|| BuildError::missing_field("ads_scheduling_timezone"))?,
            ads_triple_whale_integration: self
                .ads_triple_whale_integration
                .ok_or_else(|| BuildError::missing_field("ads_triple_whale_integration"))?,
            cards_auto_top_up: self
                .cards_auto_top_up
                .ok_or_else(|| BuildError::missing_field("cards_auto_top_up"))?,
            dispute_fighter_enabled: self
                .dispute_fighter_enabled
                .ok_or_else(|| BuildError::missing_field("dispute_fighter_enabled"))?,
        })
    }
}
