pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum LedgerActivityResourceOwnerOwner {
    LedgerActivityResourceOwnerOwnerLogoUrl(LedgerActivityResourceOwnerOwnerLogoUrl),

    LedgerActivityResourceOwnerOwnerName(LedgerActivityResourceOwnerOwnerName),
}

impl LedgerActivityResourceOwnerOwner {
    pub fn is_ledger_activity_resource_owner_owner_logo_url(&self) -> bool {
        matches!(self, Self::LedgerActivityResourceOwnerOwnerLogoUrl(_))
    }

    pub fn is_ledger_activity_resource_owner_owner_name(&self) -> bool {
        matches!(self, Self::LedgerActivityResourceOwnerOwnerName(_))
    }

    pub fn as_ledger_activity_resource_owner_owner_logo_url(
        &self,
    ) -> Option<&LedgerActivityResourceOwnerOwnerLogoUrl> {
        match self {
            Self::LedgerActivityResourceOwnerOwnerLogoUrl(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ledger_activity_resource_owner_owner_logo_url(
        self,
    ) -> Option<LedgerActivityResourceOwnerOwnerLogoUrl> {
        match self {
            Self::LedgerActivityResourceOwnerOwnerLogoUrl(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_ledger_activity_resource_owner_owner_name(
        &self,
    ) -> Option<&LedgerActivityResourceOwnerOwnerName> {
        match self {
            Self::LedgerActivityResourceOwnerOwnerName(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ledger_activity_resource_owner_owner_name(
        self,
    ) -> Option<LedgerActivityResourceOwnerOwnerName> {
        match self {
            Self::LedgerActivityResourceOwnerOwnerName(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for LedgerActivityResourceOwnerOwner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LedgerActivityResourceOwnerOwnerLogoUrl(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::LedgerActivityResourceOwnerOwnerName(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
