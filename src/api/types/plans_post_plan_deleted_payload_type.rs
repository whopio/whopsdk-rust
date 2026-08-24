pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPlanDeletedPayloadType {
    #[serde(rename = "plan.deleted")]
    PlanDeleted,
}
impl fmt::Display for PostPlanDeletedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PlanDeleted => "plan.deleted",
        };
        write!(f, "{}", s)
    }
}
