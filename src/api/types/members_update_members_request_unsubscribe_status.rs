pub use crate::prelude::*;

/// `unsubscribed` to opt out of the account's marketing, `null` to opt back in. Only the member can set it, with `user:notifications:update`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdateMembersRequestUnsubscribeStatus {
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
}
impl fmt::Display for UpdateMembersRequestUnsubscribeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Unsubscribed => "unsubscribed",
        };
        write!(f, "{}", s)
    }
}
