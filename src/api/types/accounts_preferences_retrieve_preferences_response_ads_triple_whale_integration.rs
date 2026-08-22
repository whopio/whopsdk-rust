pub use crate::prelude::*;

/// The account's Triple Whale integration, which pushes Whop ad spend to Triple Whale's Data-In API so it reports as a `whop` channel.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePreferencesResponseAdsTripleWhaleIntegration {
    /// The leading characters of the stored Data-In API key, followed by asterisks. The full key is never returned. `null` when no key is stored.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_api_key: Option<String>,
    /// The connected Shopify store domain spend is reported for, such as `acme.myshopify.com`. `null` when no store is connected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_domain: Option<String>,
    /// Where the integration stands. `requires_shopify_store` means no Shopify store is connected — Triple Whale keys records by Shopify shop, so no spend is reported until one is.
    pub status: RetrievePreferencesResponseAdsTripleWhaleIntegrationStatus,
}

impl RetrievePreferencesResponseAdsTripleWhaleIntegration {
    pub fn builder() -> RetrievePreferencesResponseAdsTripleWhaleIntegrationBuilder {
        <RetrievePreferencesResponseAdsTripleWhaleIntegrationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePreferencesResponseAdsTripleWhaleIntegrationBuilder {
    masked_api_key: Option<String>,
    shop_domain: Option<String>,
    status: Option<RetrievePreferencesResponseAdsTripleWhaleIntegrationStatus>,
}

impl RetrievePreferencesResponseAdsTripleWhaleIntegrationBuilder {
    pub fn masked_api_key(mut self, value: impl Into<String>) -> Self {
        self.masked_api_key = Some(value.into());
        self
    }

    pub fn shop_domain(mut self, value: impl Into<String>) -> Self {
        self.shop_domain = Some(value.into());
        self
    }

    pub fn status(
        mut self,
        value: RetrievePreferencesResponseAdsTripleWhaleIntegrationStatus,
    ) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePreferencesResponseAdsTripleWhaleIntegration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](RetrievePreferencesResponseAdsTripleWhaleIntegrationBuilder::status)
    pub fn build(self) -> Result<RetrievePreferencesResponseAdsTripleWhaleIntegration, BuildError> {
        Ok(RetrievePreferencesResponseAdsTripleWhaleIntegration {
            masked_api_key: self.masked_api_key,
            shop_domain: self.shop_domain,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
