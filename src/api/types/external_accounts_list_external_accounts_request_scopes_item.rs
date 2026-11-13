pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListExternalAccountsRequestScopesItem {
    #[serde(rename = "advertise")]
    Advertise,
}
impl fmt::Display for ListExternalAccountsRequestScopesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Advertise => "advertise",
        };
        write!(f, "{}", s)
    }
}
