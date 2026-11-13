pub use crate::prelude::*;

/// Always `public-key`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PasskeyCredentialParameterType {
    #[serde(rename = "public-key")]
    PublicKey,
}
impl fmt::Display for PasskeyCredentialParameterType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::PublicKey => "public-key",
        };
        write!(f, "{}", s)
    }
}
