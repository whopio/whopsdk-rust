pub use crate::prelude::*;

/// Connects or disconnects the Triple Whale integration. Requires the `ad_campaign:create` scope. Connecting requires a shop domain to report spend against — either an explicit `shop_domain` (required for any merchant without a connected Shopify store, e.g. WooCommerce, a custom checkout, or a white-label platform's merchant) or a Shopify store connected on the Fulfillment page.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesRequestAdsTripleWhaleIntegration {
    /// A Triple Whale Data-In API key with the `Ads: Write` scope, validated against Triple Whale before it is stored. Pass `null` to disconnect. Connecting for the first time backfills the account's existing ad spend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// The exact shop domain configured in Triple Whale's Settings → Store (for Shopify this is the `.myshopify.com` domain; for a custom sales platform it's whatever domain Triple Whale assigned when the shop was set up there). Validated against Triple Whale — the API key must have access to it — before it is stored. Omit to fall back to a connected Shopify store's domain; there is no way to clear a stored value, only to overwrite it with a new domain.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_domain: Option<String>,
}

impl UpdatePreferencesRequestAdsTripleWhaleIntegration {
    pub fn builder() -> UpdatePreferencesRequestAdsTripleWhaleIntegrationBuilder {
        <UpdatePreferencesRequestAdsTripleWhaleIntegrationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesRequestAdsTripleWhaleIntegrationBuilder {
    api_key: Option<String>,
    shop_domain: Option<String>,
}

impl UpdatePreferencesRequestAdsTripleWhaleIntegrationBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    pub fn shop_domain(mut self, value: impl Into<String>) -> Self {
        self.shop_domain = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePreferencesRequestAdsTripleWhaleIntegration`].
    pub fn build(self) -> Result<UpdatePreferencesRequestAdsTripleWhaleIntegration, BuildError> {
        Ok(UpdatePreferencesRequestAdsTripleWhaleIntegration {
            api_key: self.api_key,
            shop_domain: self.shop_domain,
        })
    }
}
