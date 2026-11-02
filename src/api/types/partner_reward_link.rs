pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PartnerRewardLink {
    /// When the reward stops accepting new claims and qualifying volume, as an ISO 8601 timestamp. Null when it does not expire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Onboarding reward ID, prefixed `onbr_`.
    #[serde(default)]
    pub id: String,
    /// How many businesses can earn this reward in total. Null when unlimited.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,
    /// Promotion name used in the reward link. Null when the reward has no slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// What the partner earns when a referred business qualifies for this reward. Null when the reward pays the business only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partner_reward_amount: Option<Money>,
    /// Required qualifying volume. Null for an immediate reward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_amount: Option<Money>,
    /// Income source whose volume qualifies the business. Null for an immediate reward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualification_income_source: Option<PartnerRewardLinkQualificationIncomeSource>,
    /// Number of businesses that have claimed this reward. Rewards with a qualification count when the reward is granted; immediate rewards count when the business is attributed.
    #[serde(default)]
    pub redemptions: i64,
    /// Shareable partner URL that applies this promotion to a referred business. Null when the reward has no slug.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referral_url: Option<String>,
    /// Reward value delivered to the referred business after qualification.
    #[serde(default)]
    pub referred_business_reward_amount: Money,
    /// How the reward is delivered.
    pub reward_type: PartnerRewardLinkRewardType,
    /// Whether the reward can still be claimed: `available`, `fully_claimed`, `expired`, or `unavailable`.
    pub status: PartnerRewardLinkStatus,
}

impl PartnerRewardLink {
    pub fn builder() -> PartnerRewardLinkBuilder {
        <PartnerRewardLinkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerRewardLinkBuilder {
    expires_at: Option<String>,
    id: Option<String>,
    max_redemptions: Option<i64>,
    name: Option<String>,
    partner_reward_amount: Option<Money>,
    qualification_amount: Option<Money>,
    qualification_income_source: Option<PartnerRewardLinkQualificationIncomeSource>,
    redemptions: Option<i64>,
    referral_url: Option<String>,
    referred_business_reward_amount: Option<Money>,
    reward_type: Option<PartnerRewardLinkRewardType>,
    status: Option<PartnerRewardLinkStatus>,
}

impl PartnerRewardLinkBuilder {
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

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn partner_reward_amount(mut self, value: Money) -> Self {
        self.partner_reward_amount = Some(value);
        self
    }

    pub fn qualification_amount(mut self, value: Money) -> Self {
        self.qualification_amount = Some(value);
        self
    }

    pub fn qualification_income_source(
        mut self,
        value: PartnerRewardLinkQualificationIncomeSource,
    ) -> Self {
        self.qualification_income_source = Some(value);
        self
    }

    pub fn redemptions(mut self, value: i64) -> Self {
        self.redemptions = Some(value);
        self
    }

    pub fn referral_url(mut self, value: impl Into<String>) -> Self {
        self.referral_url = Some(value.into());
        self
    }

    pub fn referred_business_reward_amount(mut self, value: Money) -> Self {
        self.referred_business_reward_amount = Some(value);
        self
    }

    pub fn reward_type(mut self, value: PartnerRewardLinkRewardType) -> Self {
        self.reward_type = Some(value);
        self
    }

    pub fn status(mut self, value: PartnerRewardLinkStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartnerRewardLink`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PartnerRewardLinkBuilder::id)
    /// - [`redemptions`](PartnerRewardLinkBuilder::redemptions)
    /// - [`referred_business_reward_amount`](PartnerRewardLinkBuilder::referred_business_reward_amount)
    /// - [`reward_type`](PartnerRewardLinkBuilder::reward_type)
    /// - [`status`](PartnerRewardLinkBuilder::status)
    pub fn build(self) -> Result<PartnerRewardLink, BuildError> {
        Ok(PartnerRewardLink {
            expires_at: self.expires_at,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            max_redemptions: self.max_redemptions,
            name: self.name,
            partner_reward_amount: self.partner_reward_amount,
            qualification_amount: self.qualification_amount,
            qualification_income_source: self.qualification_income_source,
            redemptions: self
                .redemptions
                .ok_or_else(|| BuildError::missing_field("redemptions"))?,
            referral_url: self.referral_url,
            referred_business_reward_amount: self
                .referred_business_reward_amount
                .ok_or_else(|| BuildError::missing_field("referred_business_reward_amount"))?,
            reward_type: self
                .reward_type
                .ok_or_else(|| BuildError::missing_field("reward_type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
