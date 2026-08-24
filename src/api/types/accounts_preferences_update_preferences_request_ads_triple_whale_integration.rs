pub use crate::prelude::*;

/// Connects or disconnects the Triple Whale integration. Requires a connected Shopify store, since Triple Whale keys spend records by Shopify shop.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesRequestAdsTripleWhaleIntegration {
    /// A Triple Whale Data-In API key with the `Data-In Write: Ads` scope, validated against Triple Whale before it is stored. Pass `null` to disconnect. Connecting for the first time backfills the account's existing ad spend.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
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
}

impl UpdatePreferencesRequestAdsTripleWhaleIntegrationBuilder {
    pub fn api_key(mut self, value: impl Into<String>) -> Self {
        self.api_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePreferencesRequestAdsTripleWhaleIntegration`].
    pub fn build(self) -> Result<UpdatePreferencesRequestAdsTripleWhaleIntegration, BuildError> {
        Ok(UpdatePreferencesRequestAdsTripleWhaleIntegration {
            api_key: self.api_key,
        })
    }
}
