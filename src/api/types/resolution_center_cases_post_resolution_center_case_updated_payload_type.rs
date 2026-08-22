pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostResolutionCenterCaseUpdatedPayloadType {
    #[serde(rename = "resolution_center_case.updated")]
    ResolutionCenterCaseUpdated,
}
impl fmt::Display for PostResolutionCenterCaseUpdatedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ResolutionCenterCaseUpdated => "resolution_center_case.updated",
        };
        write!(f, "{}", s)
    }
}
