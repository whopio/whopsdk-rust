pub use crate::prelude::*;

/// Where the integration stands. `requires_shop_domain` means no shop domain is configured — set `shop_domain` explicitly, or connect a Shopify store, before spend can be reported.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum UpdatePreferencesResponseAdsTripleWhaleIntegrationStatus {
    Connected,
    NotConnected,
    RequiresShopDomain,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for UpdatePreferencesResponseAdsTripleWhaleIntegrationStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Connected => serializer.serialize_str("connected"),
            Self::NotConnected => serializer.serialize_str("not_connected"),
            Self::RequiresShopDomain => serializer.serialize_str("requires_shop_domain"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for UpdatePreferencesResponseAdsTripleWhaleIntegrationStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "connected" => Ok(Self::Connected),
            "not_connected" => Ok(Self::NotConnected),
            "requires_shop_domain" => Ok(Self::RequiresShopDomain),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for UpdatePreferencesResponseAdsTripleWhaleIntegrationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Connected => write!(f, "connected"),
            Self::NotConnected => write!(f, "not_connected"),
            Self::RequiresShopDomain => write!(f, "requires_shop_domain"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
