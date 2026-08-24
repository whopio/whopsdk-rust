pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum LedgerActivityResource {
    LedgerActivityResourceLogoUrl(LedgerActivityResourceLogoUrl),

    LedgerActivityResourceName(LedgerActivityResourceName),

    LedgerActivityResourceTwo(LedgerActivityResourceTwo),

    LedgerActivityResourceOwner(LedgerActivityResourceOwner),

    LedgerActivityResourceBank(LedgerActivityResourceBank),

    LedgerActivityResourceAccountReference(LedgerActivityResourceAccountReference),

    LedgerActivityResourceAuthorizedAt(LedgerActivityResourceAuthorizedAt),
}

impl LedgerActivityResource {
    pub fn is_ledger_activity_resource_logo_url(&self) -> bool {
        matches!(self, Self::LedgerActivityResourceLogoUrl(_))
    }

    pub fn is_ledger_activity_resource_name(&self) -> bool {
        matches!(self, Self::LedgerActivityResourceName(_))
    }

    pub fn is_ledger_activity_resource_two(&self) -> bool {
        matches!(self, Self::LedgerActivityResourceTwo(_))
    }

    pub fn is_ledger_activity_resource_owner(&self) -> bool {
        matches!(self, Self::LedgerActivityResourceOwner(_))
    }

    pub fn is_ledger_activity_resource_bank(&self) -> bool {
        matches!(self, Self::LedgerActivityResourceBank(_))
    }

    pub fn is_ledger_activity_resource_account_reference(&self) -> bool {
        matches!(self, Self::LedgerActivityResourceAccountReference(_))
    }

    pub fn is_ledger_activity_resource_authorized_at(&self) -> bool {
        matches!(self, Self::LedgerActivityResourceAuthorizedAt(_))
    }

    pub fn as_ledger_activity_resource_logo_url(&self) -> Option<&LedgerActivityResourceLogoUrl> {
        match self {
            Self::LedgerActivityResourceLogoUrl(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ledger_activity_resource_logo_url(self) -> Option<LedgerActivityResourceLogoUrl> {
        match self {
            Self::LedgerActivityResourceLogoUrl(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_ledger_activity_resource_name(&self) -> Option<&LedgerActivityResourceName> {
        match self {
            Self::LedgerActivityResourceName(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ledger_activity_resource_name(self) -> Option<LedgerActivityResourceName> {
        match self {
            Self::LedgerActivityResourceName(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_ledger_activity_resource_two(&self) -> Option<&LedgerActivityResourceTwo> {
        match self {
            Self::LedgerActivityResourceTwo(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ledger_activity_resource_two(self) -> Option<LedgerActivityResourceTwo> {
        match self {
            Self::LedgerActivityResourceTwo(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_ledger_activity_resource_owner(&self) -> Option<&LedgerActivityResourceOwner> {
        match self {
            Self::LedgerActivityResourceOwner(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ledger_activity_resource_owner(self) -> Option<LedgerActivityResourceOwner> {
        match self {
            Self::LedgerActivityResourceOwner(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_ledger_activity_resource_bank(&self) -> Option<&LedgerActivityResourceBank> {
        match self {
            Self::LedgerActivityResourceBank(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ledger_activity_resource_bank(self) -> Option<LedgerActivityResourceBank> {
        match self {
            Self::LedgerActivityResourceBank(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_ledger_activity_resource_account_reference(
        &self,
    ) -> Option<&LedgerActivityResourceAccountReference> {
        match self {
            Self::LedgerActivityResourceAccountReference(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ledger_activity_resource_account_reference(
        self,
    ) -> Option<LedgerActivityResourceAccountReference> {
        match self {
            Self::LedgerActivityResourceAccountReference(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_ledger_activity_resource_authorized_at(
        &self,
    ) -> Option<&LedgerActivityResourceAuthorizedAt> {
        match self {
            Self::LedgerActivityResourceAuthorizedAt(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_ledger_activity_resource_authorized_at(
        self,
    ) -> Option<LedgerActivityResourceAuthorizedAt> {
        match self {
            Self::LedgerActivityResourceAuthorizedAt(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for LedgerActivityResource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LedgerActivityResourceLogoUrl(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::LedgerActivityResourceName(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::LedgerActivityResourceTwo(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::LedgerActivityResourceOwner(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::LedgerActivityResourceBank(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::LedgerActivityResourceAccountReference(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::LedgerActivityResourceAuthorizedAt(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
