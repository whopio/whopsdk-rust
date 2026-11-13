pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostAppInstallationCreatedPayloadType {
    #[serde(rename = "app_installation.created")]
    AppInstallationCreated,
}
impl fmt::Display for PostAppInstallationCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::AppInstallationCreated => "app_installation.created",
        };
        write!(f, "{}", s)
    }
}
