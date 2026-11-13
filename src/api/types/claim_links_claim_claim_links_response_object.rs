pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ClaimClaimLinksResponseObject {
    #[serde(rename = "claim_link")]
    ClaimLink,
}
impl fmt::Display for ClaimClaimLinksResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ClaimLink => "claim_link",
        };
        write!(f, "{}", s)
    }
}
