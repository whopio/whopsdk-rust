pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostMembershipActivatedPayloadType {
    #[serde(rename = "membership.activated")]
    MembershipActivated,
}
impl fmt::Display for PostMembershipActivatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MembershipActivated => "membership.activated",
        };
        write!(f, "{}", s)
    }
}
