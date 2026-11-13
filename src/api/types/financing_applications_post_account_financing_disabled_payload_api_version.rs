pub use crate::prelude::*;

/// The API version for this webhook
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PostAccountFinancingDisabledPayloadApiVersion {
    #[serde(rename = "v1")]
    V1,
}
impl fmt::Display for PostAccountFinancingDisabledPayloadApiVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::V1 => "v1",
        };
        write!(f, "{}", s)
    }
}
