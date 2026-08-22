pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostResolutionCenterCaseCreatedPayloadType {
    #[serde(rename = "resolution_center_case.created")]
    ResolutionCenterCaseCreated,
}
impl fmt::Display for PostResolutionCenterCaseCreatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ResolutionCenterCaseCreated => "resolution_center_case.created",
        };
        write!(f, "{}", s)
    }
}
