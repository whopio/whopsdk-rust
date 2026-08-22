pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostMembershipDeactivatedPayloadType {
    #[serde(rename = "membership.deactivated")]
    MembershipDeactivated,
}
impl fmt::Display for PostMembershipDeactivatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MembershipDeactivated => "membership.deactivated",
        };
        write!(f, "{}", s)
    }
}
