pub use crate::prelude::*;

/// How the asset was created. Always `generated`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MediaAssetSource {
    #[serde(rename = "generated")]
    Generated,
}
impl fmt::Display for MediaAssetSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Generated => "generated",
        };
        write!(f, "{}", s)
    }
}
