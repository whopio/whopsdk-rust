pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostMembershipTrialEndingSoonPayloadType {
    #[serde(rename = "membership.trial_ending_soon")]
    MembershipTrialEndingSoon,
}
impl fmt::Display for PostMembershipTrialEndingSoonPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MembershipTrialEndingSoon => "membership.trial_ending_soon",
        };
        write!(f, "{}", s)
    }
}
