pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PartnerReferralReward {
    /// Activity that qualifies for this reward, when specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bot_qualification_type: Option<PartnerReferralRewardBotQualificationType>,
    /// Partner reward ID, prefixed `prwd_`.
    #[serde(default)]
    pub id: String,
    /// USD amount of qualifying activity required to earn the reward.
    #[serde(default)]
    pub qualification_amount: Money,
    /// Reward recipient's role, or null for a fixed user who is not the requesting partner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient: Option<PartnerReferralRewardRecipient>,
    /// Concrete recipient's user or business ID, or null until a business redeems the offer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_id: Option<String>,
    /// USD amount granted after qualification.
    #[serde(default)]
    pub reward_amount: Money,
}

impl PartnerReferralReward {
    pub fn builder() -> PartnerReferralRewardBuilder {
        <PartnerReferralRewardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerReferralRewardBuilder {
    bot_qualification_type: Option<PartnerReferralRewardBotQualificationType>,
    id: Option<String>,
    qualification_amount: Option<Money>,
    recipient: Option<PartnerReferralRewardRecipient>,
    recipient_id: Option<String>,
    reward_amount: Option<Money>,
}

impl PartnerReferralRewardBuilder {
    pub fn bot_qualification_type(
        mut self,
        value: PartnerReferralRewardBotQualificationType,
    ) -> Self {
        self.bot_qualification_type = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn qualification_amount(mut self, value: Money) -> Self {
        self.qualification_amount = Some(value);
        self
    }

    pub fn recipient(mut self, value: PartnerReferralRewardRecipient) -> Self {
        self.recipient = Some(value);
        self
    }

    pub fn recipient_id(mut self, value: impl Into<String>) -> Self {
        self.recipient_id = Some(value.into());
        self
    }

    pub fn reward_amount(mut self, value: Money) -> Self {
        self.reward_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartnerReferralReward`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PartnerReferralRewardBuilder::id)
    /// - [`qualification_amount`](PartnerReferralRewardBuilder::qualification_amount)
    /// - [`reward_amount`](PartnerReferralRewardBuilder::reward_amount)
    pub fn build(self) -> Result<PartnerReferralReward, BuildError> {
        Ok(PartnerReferralReward {
            bot_qualification_type: self.bot_qualification_type,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            qualification_amount: self
                .qualification_amount
                .ok_or_else(|| BuildError::missing_field("qualification_amount"))?,
            recipient: self.recipient,
            recipient_id: self.recipient_id,
            reward_amount: self
                .reward_amount
                .ok_or_else(|| BuildError::missing_field("reward_amount"))?,
        })
    }
}
