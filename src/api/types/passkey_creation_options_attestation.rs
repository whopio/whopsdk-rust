pub use crate::prelude::*;

/// Whop does not ask for attestation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PasskeyCreationOptionsAttestation {
    #[serde(rename = "none")]
    None,
}
impl fmt::Display for PasskeyCreationOptionsAttestation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::None => "none",
        };
        write!(f, "{}", s)
    }
}
