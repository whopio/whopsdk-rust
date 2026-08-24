pub use crate::prelude::*;

/// Scopes the app asks a user to grant when they authorize it, shown on the consent screen.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AppRequiredScopesItem {
    #[serde(rename = "read_user")]
    ReadUser,
}
impl fmt::Display for AppRequiredScopesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ReadUser => "read_user",
        };
        write!(f, "{}", s)
    }
}
