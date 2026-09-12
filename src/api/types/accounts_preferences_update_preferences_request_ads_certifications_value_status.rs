pub use crate::prelude::*;

/// Must be `pending_information`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdatePreferencesRequestAdsCertificationsValueStatus {
    #[serde(rename = "pending_information")]
    PendingInformation,
}
impl fmt::Display for UpdatePreferencesRequestAdsCertificationsValueStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PendingInformation => "pending_information",
        };
        write!(f, "{}", s)
    }
}
