pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostResolutionCenterCaseDecidedPayloadType {
    #[serde(rename = "resolution_center_case.decided")]
    ResolutionCenterCaseDecided,
}
impl fmt::Display for PostResolutionCenterCaseDecidedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ResolutionCenterCaseDecided => "resolution_center_case.decided",
        };
        write!(f, "{}", s)
    }
}
