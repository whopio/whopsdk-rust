pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListOauthGrantsRequestOrder {
    #[serde(rename = "created_at")]
    CreatedAt,
}
impl fmt::Display for ListOauthGrantsRequestOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::CreatedAt => "created_at",
        };
        write!(f, "{}", s)
    }
}
