pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OnboardingReward {
    /// When the reward stops accepting new claims and qualifying volume, as an ISO 8601 timestamp. Null when it does not expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Onboarding reward ID, prefixed `onbr_`.
    #[serde(default)]
    pub id: String,
    /// How many businesses can earn this reward in total. Null when unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    /// Partner whose link attributed this reward.
    #[serde(default)]
    pub partner: UserSummary,
    /// Required qualifying volume. Null for an immediate reward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_amount: Option<Money>,
    /// Income source whose volume qualifies the business. Null for an immediate reward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_income_source: Option<OnboardingRewardQualificationIncomeSource>,
    /// Whether the attributed business met the requirement. Null before a business claims the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_met: Option<bool>,
    /// Qualifying volume accumulated by the attributed business. Null before a business claims the link and for immediate rewards.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_progress: Option<Money>,
    /// How many rewards are still unclaimed. For rewards with a qualification, a business claims one only when it meets the requirement, so this can reach zero while other businesses are still working toward it. Null when unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remaining_redemptions: Option<i64>,
    /// Reward value delivered after qualification.
    #[serde(default)]
    pub reward_amount: Money,
    /// How the reward is delivered.
    pub reward_type: OnboardingRewardRewardType,
    /// Whether the reward was credited to the attributed business. Null before a business claims the link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rewarded: Option<bool>,
    /// Whether the reward can still be claimed: `available`, `fully_claimed`, `expired`, or `unavailable`.
    pub status: OnboardingRewardStatus,
}

impl OnboardingReward {
    pub fn builder() -> OnboardingRewardBuilder {
        <OnboardingRewardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OnboardingRewardBuilder {
    expires_at: Option<String>,
    id: Option<String>,
    max_redemptions: Option<i64>,
    partner: Option<UserSummary>,
    qualification_amount: Option<Money>,
    qualification_income_source: Option<OnboardingRewardQualificationIncomeSource>,
    qualification_met: Option<bool>,
    qualification_progress: Option<Money>,
    remaining_redemptions: Option<i64>,
    reward_amount: Option<Money>,
    reward_type: Option<OnboardingRewardRewardType>,
    rewarded: Option<bool>,
    status: Option<OnboardingRewardStatus>,
}

impl OnboardingRewardBuilder {
    pub fn expires_at(mut self, value: impl Into<String>) -> Self {
        self.expires_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn max_redemptions(mut self, value: i64) -> Self {
        self.max_redemptions = Some(value);
        self
    }

    pub fn partner(mut self, value: UserSummary) -> Self {
        self.partner = Some(value);
        self
    }

    pub fn qualification_amount(mut self, value: Money) -> Self {
        self.qualification_amount = Some(value);
        self
    }

    pub fn qualification_income_source(
        mut self,
        value: OnboardingRewardQualificationIncomeSource,
    ) -> Self {
        self.qualification_income_source = Some(value);
        self
    }

    pub fn qualification_met(mut self, value: bool) -> Self {
        self.qualification_met = Some(value);
        self
    }

    pub fn qualification_progress(mut self, value: Money) -> Self {
        self.qualification_progress = Some(value);
        self
    }

    pub fn remaining_redemptions(mut self, value: i64) -> Self {
        self.remaining_redemptions = Some(value);
        self
    }

    pub fn reward_amount(mut self, value: Money) -> Self {
        self.reward_amount = Some(value);
        self
    }

    pub fn reward_type(mut self, value: OnboardingRewardRewardType) -> Self {
        self.reward_type = Some(value);
        self
    }

    pub fn rewarded(mut self, value: bool) -> Self {
        self.rewarded = Some(value);
        self
    }

    pub fn status(mut self, value: OnboardingRewardStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OnboardingReward`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](OnboardingRewardBuilder::id)
    /// - [`partner`](OnboardingRewardBuilder::partner)
    /// - [`reward_amount`](OnboardingRewardBuilder::reward_amount)
    /// - [`reward_type`](OnboardingRewardBuilder::reward_type)
    /// - [`status`](OnboardingRewardBuilder::status)
    pub fn build(self) -> Result<OnboardingReward, BuildError> {
        Ok(OnboardingReward {
            expires_at: self.expires_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            max_redemptions: self.max_redemptions,
            partner: self
                .partner
                .ok_or_else(|| BuildError::missing_field("partner"))?,
            qualification_amount: self.qualification_amount,
            qualification_income_source: self.qualification_income_source,
            qualification_met: self.qualification_met,
            qualification_progress: self.qualification_progress,
            remaining_redemptions: self.remaining_redemptions,
            reward_amount: self
                .reward_amount
                .ok_or_else(|| BuildError::missing_field("reward_amount"))?,
            reward_type: self
                .reward_type
                .ok_or_else(|| BuildError::missing_field("reward_type"))?,
            rewarded: self.rewarded,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
