use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionUrls {
    pub api: String,
    pub vault: String,
}
impl Default for ProductionUrls {
    fn default() -> Self {
        Self {
            api: "https://api.whop.com/api/v1".to_string(),
            vault: "https://vault-api.whop.com/api/v1".to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxUrls {
    pub api: String,
    pub vault: String,
}
impl Default for SandboxUrls {
    fn default() -> Self {
        Self {
            api: "https://sandbox-api.whop.com/api/v1".to_string(),
            vault: "https://sandbox-vault-api.whop.com/api/v1".to_string(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Environment {
    Production(ProductionUrls),
    Sandbox(SandboxUrls),
}
impl Environment {
    pub fn production() -> Self {
        Self::Production(ProductionUrls::default())
    }

    pub fn sandbox() -> Self {
        Self::Sandbox(SandboxUrls::default())
    }

    pub fn url(&self) -> &str {
        match self {
            Self::Production(urls) => &urls.api,
            Self::Sandbox(urls) => &urls.api,
        }
    }

    pub fn api_url(&self) -> &str {
        match self {
            Self::Production(urls) => &urls.api,
            Self::Sandbox(urls) => &urls.api,
        }
    }

    pub fn vault_url(&self) -> &str {
        match self {
            Self::Production(urls) => &urls.vault,
            Self::Sandbox(urls) => &urls.vault,
        }
    }
}
impl Default for Environment {
    fn default() -> Self {
        Self::Production(ProductionUrls::default())
    }
}
