pub use crate::prelude::*;

/// The OAuth response type. Only `code` is accepted; defaults to `code`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateOauthGrantsRequestResponseType {
    #[serde(rename = "code")]
    Code,
}
impl fmt::Display for CreateOauthGrantsRequestResponseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Code => "code",
        };
        write!(f, "{}", s)
    }
}
