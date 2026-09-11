pub use crate::prelude::*;

/// How the card runs. `whop_ai` means `prompt` is sent to Whop AI, which carries out every step.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EconomicIntelligenceExecutionType {
    #[serde(rename = "whop_ai")]
    WhopAi,
}
impl fmt::Display for EconomicIntelligenceExecutionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WhopAi => "whop_ai",
        };
        write!(f, "{}", s)
    }
}
