pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostExportCompletedPayloadType {
    #[serde(rename = "export.completed")]
    ExportCompleted,
}
impl fmt::Display for PostExportCompletedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ExportCompleted => "export.completed",
        };
        write!(f, "{}", s)
    }
}
