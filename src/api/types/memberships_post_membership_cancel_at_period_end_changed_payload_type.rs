pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostMembershipCancelAtPeriodEndChangedPayloadType {
    #[serde(rename = "membership.cancel_at_period_end_changed")]
    MembershipCancelAtPeriodEndChanged,
}
impl fmt::Display for PostMembershipCancelAtPeriodEndChangedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MembershipCancelAtPeriodEndChanged => "membership.cancel_at_period_end_changed",
        };
        write!(f, "{}", s)
    }
}
