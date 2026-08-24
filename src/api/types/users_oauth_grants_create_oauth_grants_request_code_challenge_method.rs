pub use crate::prelude::*;

/// How `code_challenge` was derived. Only `S256` is accepted.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateOauthGrantsRequestCodeChallengeMethod {
    S256,
}
impl fmt::Display for CreateOauthGrantsRequestCodeChallengeMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::S256 => "S256",
        };
        write!(f, "{}", s)
    }
}
