pub use crate::prelude::*;

/// These are the scopes an app can request on behalf of a user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AppValidScopes {
    #[serde(rename = "read_user")]
    ReadUser,
}
impl fmt::Display for AppValidScopes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ReadUser => "read_user",
        };
        write!(f, "{}", s)
    }
}
