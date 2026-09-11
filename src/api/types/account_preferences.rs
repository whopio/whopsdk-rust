pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AccountPreferences {
    /// The account's Whop Ads services and payment authorization agreement. `status` is `not_required`, `pending_signature` (a signature has been requested and campaign launch is blocked until it is provided), or `signed`. While pending, read the fields to answer from `GET /verifications/{id}` and sign by submitting them via `PATCH /verifications/{id}`.
    #[serde(default)]
    pub ads_agreement: HashMap<String, serde_json::Value>,
    /// How the account pays for Whop Ads spend. `primary` is charged first; `backup` covers the charge when it fails. Each entry has a `type` of `platform_balance` (id `ldgr_`) or `card` (id `payt_`), plus display fields so the configured source renders even for a viewer who doesn't own it. `backup` is `null` when only one method is configured. `null` until ads billing has been configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_payment_methods: Option<HashMap<String, serde_json::Value>>,
    /// Lowercase ISO currency code, such as `usd` or `eur`, used to display ad spend and stats. Defaults to `usd`.
    #[serde(default)]
    pub ads_reporting_currency: String,
    /// IANA timezone (e.g. `America/New_York`) used to interpret campaign start/end times and to bucket reports. Defaults to `America/New_York` until explicitly overridden.
    #[serde(default)]
    pub ads_scheduling_timezone: String,
    /// The account's Triple Whale integration, which pushes Whop ad spend to Triple Whale's Data-In API so it reports as a `whop` channel. `status` is `connected`, `not_connected`, or `requires_shopify_store` (Triple Whale keys records by Shopify shop, so spend only flows while a store is connected). `masked_api_key` shows the leading characters of the stored key; the full key is never returned. `shop_domain` is the Shopify store spend is reported for.
    #[serde(default)]
    pub ads_triple_whale_integration: HashMap<String, serde_json::Value>,
    /// Whether incoming funds are automatically moved to the account's cards balance. `false` when the account has no cards balance.
    #[serde(default)]
    pub cards_auto_top_up: bool,
    /// Whether Whop Card notifications reach this account's team. `true` by default, including when the account has no cards balance. Set it to `false` to stop every card email and push notification for the account — application status, verification and action-required alerts, card-ready alerts, declines, large charges, and cashback summaries. Cardholder onboarding invitations still send, because they carry the only link an invited cardholder can onboard with. Requesting a card is rejected while notifications are off, since the request reaches nobody. Cards on personal accounts are unaffected.
    #[serde(default)]
    pub cards_notifications: bool,
    /// Whether Whop assembles and files the evidence response when this account's payments are disputed. Off by default; enabling it also opts the account into the success fee charged only on disputes it wins.
    #[serde(default)]
    pub dispute_fighter_enabled: bool,
    /// Whether economic intelligence is enabled for the account.
    #[serde(default)]
    pub economic_intelligence: bool,
}

impl AccountPreferences {
    pub fn builder() -> AccountPreferencesBuilder {
        <AccountPreferencesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountPreferencesBuilder {
    ads_agreement: Option<HashMap<String, serde_json::Value>>,
    ads_payment_methods: Option<HashMap<String, serde_json::Value>>,
    ads_reporting_currency: Option<String>,
    ads_scheduling_timezone: Option<String>,
    ads_triple_whale_integration: Option<HashMap<String, serde_json::Value>>,
    cards_auto_top_up: Option<bool>,
    cards_notifications: Option<bool>,
    dispute_fighter_enabled: Option<bool>,
    economic_intelligence: Option<bool>,
}

impl AccountPreferencesBuilder {
    pub fn ads_agreement(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.ads_agreement = Some(value);
        self
    }

    pub fn ads_payment_methods(mut self, value: HashMap<String, serde_json::Value>) -> Self {
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
        value: HashMap<String, serde_json::Value>,
    ) -> Self {
        self.ads_triple_whale_integration = Some(value);
        self
    }

    pub fn cards_auto_top_up(mut self, value: bool) -> Self {
        self.cards_auto_top_up = Some(value);
        self
    }

    pub fn cards_notifications(mut self, value: bool) -> Self {
        self.cards_notifications = Some(value);
        self
    }

    pub fn dispute_fighter_enabled(mut self, value: bool) -> Self {
        self.dispute_fighter_enabled = Some(value);
        self
    }

    pub fn economic_intelligence(mut self, value: bool) -> Self {
        self.economic_intelligence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountPreferences`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ads_agreement`](AccountPreferencesBuilder::ads_agreement)
    /// - [`ads_reporting_currency`](AccountPreferencesBuilder::ads_reporting_currency)
    /// - [`ads_scheduling_timezone`](AccountPreferencesBuilder::ads_scheduling_timezone)
    /// - [`ads_triple_whale_integration`](AccountPreferencesBuilder::ads_triple_whale_integration)
    /// - [`cards_auto_top_up`](AccountPreferencesBuilder::cards_auto_top_up)
    /// - [`cards_notifications`](AccountPreferencesBuilder::cards_notifications)
    /// - [`dispute_fighter_enabled`](AccountPreferencesBuilder::dispute_fighter_enabled)
    /// - [`economic_intelligence`](AccountPreferencesBuilder::economic_intelligence)
    pub fn build(self) -> Result<AccountPreferences, BuildError> {
        Ok(AccountPreferences {
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
            cards_notifications: self
                .cards_notifications
                .ok_or_else(|| BuildError::missing_field("cards_notifications"))?,
            dispute_fighter_enabled: self
                .dispute_fighter_enabled
                .ok_or_else(|| BuildError::missing_field("dispute_fighter_enabled"))?,
            economic_intelligence: self
                .economic_intelligence
                .ok_or_else(|| BuildError::missing_field("economic_intelligence"))?,
        })
    }
}
