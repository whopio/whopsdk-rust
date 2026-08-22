pub use crate::prelude::*;

/// Account-level 3D Secure behavior. Set `mandate_challenge` to require cardholder verification on supported card payments, or `null` to use the standard checkout flow.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdateAccountsRequestThreeDsLevel {
    #[serde(rename = "mandate_challenge")]
    MandateChallenge,
}
impl fmt::Display for UpdateAccountsRequestThreeDsLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::MandateChallenge => "mandate_challenge",
        };
        write!(f, "{}", s)
    }
}
