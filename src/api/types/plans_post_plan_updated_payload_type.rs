pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPlanUpdatedPayloadType {
    #[serde(rename = "plan.updated")]
    PlanUpdated,
}
impl fmt::Display for PostPlanUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PlanUpdated => "plan.updated",
        };
        write!(f, "{}", s)
    }
}
