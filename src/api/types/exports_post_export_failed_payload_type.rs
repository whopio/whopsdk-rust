pub use crate::prelude::*;

/// The webhook event type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostExportFailedPayloadType {
    #[serde(rename = "export.failed")]
    ExportFailed,
}
impl fmt::Display for PostExportFailedPayloadType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ExportFailed => "export.failed",
        };
        write!(f, "{}", s)
    }
}
