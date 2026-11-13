pub use crate::prelude::*;

/// Connects or disconnects the Triple Whale integration, or changes the shop it reports to. Requires the `ad_campaign:create` scope on your API key. Connecting requires a `shop_domain` to report spend against.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesRequestAdsTripleWhaleIntegration {
    /// A Triple Whale Data-In API key with the `Ads: Write` scope, validated against Triple Whale before it is stored. Pass `null` to disconnect. Connecting for the first time backfills the account's existing ad spend. Required unless you are only changing `shop_domain` on an already connected integration, in which case the stored key is reused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    /// The exact shop domain configured in Triple Whale's Settings → Store (for Shopify this is the `.myshopify.com` domain; for a custom sales platform it's whatever domain Triple Whale assigned when the shop was set up there). A leading `https://` and trailing `/` are stripped, and what remains must be a bare hostname with no path or spaces. Validated against Triple Whale — the API key must have access to it — before it is stored. Changing it on a connected integration backfills the account's ad spend onto the new shop. Omit to keep the stored domain; there is no way to clear a stored value, only to overwrite it with a new domain.
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
