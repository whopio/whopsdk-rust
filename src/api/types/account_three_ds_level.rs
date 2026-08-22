pub use crate::prelude::*;

/// Account-level 3D Secure behavior. `mandate_challenge` requires cardholder verification on supported card payments; `null` uses the standard checkout flow.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AccountThreeDsLevel {
    #[serde(rename = "mandate_challenge")]
    MandateChallenge,
}
impl fmt::Display for AccountThreeDsLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MandateChallenge => "mandate_challenge",
        };
        write!(f, "{}", s)
    }
}
