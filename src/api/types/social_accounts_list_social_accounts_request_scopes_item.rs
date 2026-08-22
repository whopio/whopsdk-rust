pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListSocialAccountsRequestScopesItem {
    #[serde(rename = "advertise")]
    Advertise,
}
impl fmt::Display for ListSocialAccountsRequestScopesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Advertise => "advertise",
        };
        write!(f, "{}", s)
    }
}
