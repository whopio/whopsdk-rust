pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ListEarningsResponseDataItemResourceSlugObject {
    #[serde(rename = "onboarding_reward")]
    OnboardingReward,
}
impl fmt::Display for ListEarningsResponseDataItemResourceSlugObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::OnboardingReward => "onboarding_reward",
        };
        write!(f, "{}", s)
    }
}
