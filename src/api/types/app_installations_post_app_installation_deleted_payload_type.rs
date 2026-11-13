pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostAppInstallationDeletedPayloadType {
    #[serde(rename = "app_installation.deleted")]
    AppInstallationDeleted,
}
impl fmt::Display for PostAppInstallationDeletedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AppInstallationDeleted => "app_installation.deleted",
        };
        write!(f, "{}", s)
    }
}
