pub use crate::prelude::*;

/// `unsubscribed` once the member opts out of the account's marketing communications, `null` while they're subscribed. Only the member can change it.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MemberUnsubscribeStatus {
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
}
impl fmt::Display for MemberUnsubscribeStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Unsubscribed => "unsubscribed",
        };
        write!(f, "{}", s)
    }
}
