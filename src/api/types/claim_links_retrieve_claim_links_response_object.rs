pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RetrieveClaimLinksResponseObject {
    #[serde(rename = "claim_link")]
    ClaimLink,
}
impl fmt::Display for RetrieveClaimLinksResponseObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ClaimLink => "claim_link",
        };
        write!(f, "{}", s)
    }
}
