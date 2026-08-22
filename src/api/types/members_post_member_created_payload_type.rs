pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostMemberCreatedPayloadType {
    #[serde(rename = "member.created")]
    MemberCreated,
}
impl fmt::Display for PostMemberCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MemberCreated => "member.created",
        };
        write!(f, "{}", s)
    }
}
