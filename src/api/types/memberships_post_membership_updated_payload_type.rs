pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostMembershipUpdatedPayloadType {
    #[serde(rename = "membership.updated")]
    MembershipUpdated,
}
impl fmt::Display for PostMembershipUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MembershipUpdated => "membership.updated",
        };
        write!(f, "{}", s)
    }
}
