pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostPlanCreatedPayloadType {
    #[serde(rename = "plan.created")]
    PlanCreated,
}
impl fmt::Display for PostPlanCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PlanCreated => "plan.created",
        };
        write!(f, "{}", s)
    }
}
