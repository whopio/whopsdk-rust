pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ConnectSocialAccountsRequestScopesItem {
    #[serde(rename = "advertise")]
    Advertise,
}
impl fmt::Display for ConnectSocialAccountsRequestScopesItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Advertise => "advertise",
        };
        write!(f, "{}", s)
    }
}
