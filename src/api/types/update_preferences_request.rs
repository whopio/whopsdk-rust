pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesRequest {
    /// How the account pays for Whop Ads spend. `primary` is charged first; `backup` covers the charge when the primary fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_payment_methods: Option<UpdatePreferencesRequestAdsPaymentMethods>,
    /// Lowercase ISO currency code, such as `usd` or `eur`, used to display ad spend and stats. Defaults to `usd`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_reporting_currency: Option<String>,
    /// IANA timezone (e.g. `America/New_York`) used to interpret campaign start/end times and to bucket reports. Cannot be cleared once set — pass a new value to change it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_scheduling_timezone: Option<String>,
    /// Connects or disconnects the Triple Whale integration. Requires a connected Shopify store, since Triple Whale keys spend records by Shopify shop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ads_triple_whale_integration: Option<UpdatePreferencesRequestAdsTripleWhaleIntegration>,
    /// Whether incoming funds are automatically moved to the account's cards balance. Requires a cards balance on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards_auto_top_up: Option<bool>,
    /// Whether Whop assembles and files the evidence response when this account's payments are disputed. Off by default; enabling it also opts the account into the success fee charged only on disputes it wins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute_fighter_enabled: Option<bool>,
}

impl UpdatePreferencesRequest {
    pub fn builder() -> UpdatePreferencesRequestBuilder {
        <UpdatePreferencesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesRequestBuilder {
    ads_payment_methods: Option<UpdatePreferencesRequestAdsPaymentMethods>,
    ads_reporting_currency: Option<String>,
    ads_scheduling_timezone: Option<String>,
    ads_triple_whale_integration: Option<UpdatePreferencesRequestAdsTripleWhaleIntegration>,
    cards_auto_top_up: Option<bool>,
    dispute_fighter_enabled: Option<bool>,
}

impl UpdatePreferencesRequestBuilder {
    pub fn ads_payment_methods(mut self, value: UpdatePreferencesRequestAdsPaymentMethods) -> Self {
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
        value: UpdatePreferencesRequestAdsTripleWhaleIntegration,
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

    /// Consumes the builder and constructs a [`UpdatePreferencesRequest`].
    pub fn build(self) -> Result<UpdatePreferencesRequest, BuildError> {
        Ok(UpdatePreferencesRequest {
            ads_payment_methods: self.ads_payment_methods,
            ads_reporting_currency: self.ads_reporting_currency,
            ads_scheduling_timezone: self.ads_scheduling_timezone,
            ads_triple_whale_integration: self.ads_triple_whale_integration,
            cards_auto_top_up: self.cards_auto_top_up,
            dispute_fighter_enabled: self.dispute_fighter_enabled,
        })
    }
}
