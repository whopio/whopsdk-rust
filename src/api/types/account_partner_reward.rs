pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountPartnerReward {
    /// Reward definition ID, prefixed `prwd_`. Progress and status apply to the containing account.
    #[serde(default)]
    pub id: String,
    /// Qualifying USD volume required to earn this reward.
    #[serde(default)]
    pub qualification_amount: Money,
    /// Qualifying USD volume for the reward’s activity accumulated by this account since attribution, calculated using the fulfillment rules.
    #[serde(default)]
    pub qualification_progress: Money,
    /// Activity that qualifies this account for the reward.
    pub qualification_type: AccountPartnerRewardQualificationType,
    /// USD amount credited for this reward. Uses the saved grant amount once fulfillment has started.
    #[serde(default)]
    pub reward_amount: Money,
    /// This account's reward state. Credited requires a posted ledger entry; processing includes a met requirement awaiting fulfillment. Reversing and reversed reflect a subsequent reward reversal.
    pub status: AccountPartnerRewardStatus,
}

impl AccountPartnerReward {
    pub fn builder() -> AccountPartnerRewardBuilder {
        <AccountPartnerRewardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountPartnerRewardBuilder {
    id: Option<String>,
    qualification_amount: Option<Money>,
    qualification_progress: Option<Money>,
    qualification_type: Option<AccountPartnerRewardQualificationType>,
    reward_amount: Option<Money>,
    status: Option<AccountPartnerRewardStatus>,
}

impl AccountPartnerRewardBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn qualification_amount(mut self, value: Money) -> Self {
        self.qualification_amount = Some(value);
        self
    }

    pub fn qualification_progress(mut self, value: Money) -> Self {
        self.qualification_progress = Some(value);
        self
    }

    pub fn qualification_type(mut self, value: AccountPartnerRewardQualificationType) -> Self {
        self.qualification_type = Some(value);
        self
    }

    pub fn reward_amount(mut self, value: Money) -> Self {
        self.reward_amount = Some(value);
        self
    }

    pub fn status(mut self, value: AccountPartnerRewardStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AccountPartnerReward`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AccountPartnerRewardBuilder::id)
    /// - [`qualification_amount`](AccountPartnerRewardBuilder::qualification_amount)
    /// - [`qualification_progress`](AccountPartnerRewardBuilder::qualification_progress)
    /// - [`qualification_type`](AccountPartnerRewardBuilder::qualification_type)
    /// - [`reward_amount`](AccountPartnerRewardBuilder::reward_amount)
    /// - [`status`](AccountPartnerRewardBuilder::status)
    pub fn build(self) -> Result<AccountPartnerReward, BuildError> {
        Ok(AccountPartnerReward {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            qualification_amount: self
                .qualification_amount
                .ok_or_else(|| BuildError::missing_field("qualification_amount"))?,
            qualification_progress: self
                .qualification_progress
                .ok_or_else(|| BuildError::missing_field("qualification_progress"))?,
            qualification_type: self
                .qualification_type
                .ok_or_else(|| BuildError::missing_field("qualification_type"))?,
            reward_amount: self
                .reward_amount
                .ok_or_else(|| BuildError::missing_field("reward_amount"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
