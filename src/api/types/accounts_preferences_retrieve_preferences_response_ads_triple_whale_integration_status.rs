pub use crate::prelude::*;

/// Where the integration stands. `requires_shopify_store` means no Shopify store is connected — Triple Whale keys records by Shopify shop, so no spend is reported until one is.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RetrievePreferencesResponseAdsTripleWhaleIntegrationStatus {
    Connected,
    NotConnected,
    RequiresShopifyStore,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for RetrievePreferencesResponseAdsTripleWhaleIntegrationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Connected => serializer.serialize_str("connected"),
            Self::NotConnected => serializer.serialize_str("not_connected"),
            Self::RequiresShopifyStore => serializer.serialize_str("requires_shopify_store"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for RetrievePreferencesResponseAdsTripleWhaleIntegrationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "connected" => Ok(Self::Connected),
            "not_connected" => Ok(Self::NotConnected),
            "requires_shopify_store" => Ok(Self::RequiresShopifyStore),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for RetrievePreferencesResponseAdsTripleWhaleIntegrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connected => write!(f, "connected"),
            Self::NotConnected => write!(f, "not_connected"),
            Self::RequiresShopifyStore => write!(f, "requires_shopify_store"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
