pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostMemberUpdatedPayloadType {
    #[serde(rename = "member.updated")]
    MemberUpdated,
}
impl fmt::Display for PostMemberUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MemberUpdated => "member.updated",
        };
        write!(f, "{}", s)
    }
}
