pub use crate::prelude::*;

/// The status to move the recommendation to. Only `superseded` is accepted.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UpdateEconomicIntelligenceRequestStatus {
    #[serde(rename = "superseded")]
    Superseded,
}
impl fmt::Display for UpdateEconomicIntelligenceRequestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Superseded => "superseded",
        };
        write!(f, "{}", s)
    }
}
