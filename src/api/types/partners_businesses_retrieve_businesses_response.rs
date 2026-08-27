pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrieveBusinessesResponse {
    /// Referred account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<RetrieveBusinessesResponseAccount>,
    /// When the partner business was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(default)]
    pub earnings_usd: RetrieveBusinessesResponseEarningsUsd,
    /// The partner who referred the business owner onto Whop (first tier). Null if there is no active first-tier partner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_tier_partner: Option<RetrieveBusinessesResponseFirstTierPartner>,
    /// Partner business ID.
    #[serde(default)]
    pub id: String,
    /// Which tier the caller earns on for this business: `first` (they referred the owner), `second` (they referred the first-tier partner), or `blueprint` (the business deployed a site from their blueprint).
    pub my_partner_tier: RetrieveBusinessesResponseMyPartnerTier,
    pub object: RetrieveBusinessesResponseObject,
    /// The owner of the referred business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<RetrieveBusinessesResponseOwner>,
    /// The referrer's commission rate for each income source, expressed as a fraction (0.3 = 30%).
    #[serde(default)]
    pub payout_percentages: RetrieveBusinessesResponsePayoutPercentages,
    /// When the referral expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub referral_expires_at: Option<DateTime<FixedOffset>>,
    /// When the referral became active.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub referral_started_at: Option<DateTime<FixedOffset>>,
    /// The second-tier partner who earns on this business (referred the first-tier partner). Null if there is no active second-tier partner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub second_tier_partner: Option<RetrieveBusinessesResponseSecondTierPartner>,
    /// Current referral status.
    pub status: RetrieveBusinessesResponseStatus,
    #[serde(default)]
    pub volume_usd: RetrieveBusinessesResponseVolumeUsd,
}

impl RetrieveBusinessesResponse {
    pub fn builder() -> RetrieveBusinessesResponseBuilder {
        <RetrieveBusinessesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBusinessesResponseBuilder {
    account: Option<RetrieveBusinessesResponseAccount>,
    created_at: Option<DateTime<FixedOffset>>,
    earnings_usd: Option<RetrieveBusinessesResponseEarningsUsd>,
    first_tier_partner: Option<RetrieveBusinessesResponseFirstTierPartner>,
    id: Option<String>,
    my_partner_tier: Option<RetrieveBusinessesResponseMyPartnerTier>,
    object: Option<RetrieveBusinessesResponseObject>,
    owner: Option<RetrieveBusinessesResponseOwner>,
    payout_percentages: Option<RetrieveBusinessesResponsePayoutPercentages>,
    referral_expires_at: Option<DateTime<FixedOffset>>,
    referral_started_at: Option<DateTime<FixedOffset>>,
    second_tier_partner: Option<RetrieveBusinessesResponseSecondTierPartner>,
    status: Option<RetrieveBusinessesResponseStatus>,
    volume_usd: Option<RetrieveBusinessesResponseVolumeUsd>,
}

impl RetrieveBusinessesResponseBuilder {
    pub fn account(mut self, value: RetrieveBusinessesResponseAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn earnings_usd(mut self, value: RetrieveBusinessesResponseEarningsUsd) -> Self {
        self.earnings_usd = Some(value);
        self
    }

    pub fn first_tier_partner(mut self, value: RetrieveBusinessesResponseFirstTierPartner) -> Self {
        self.first_tier_partner = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn my_partner_tier(mut self, value: RetrieveBusinessesResponseMyPartnerTier) -> Self {
        self.my_partner_tier = Some(value);
        self
    }

    pub fn object(mut self, value: RetrieveBusinessesResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn owner(mut self, value: RetrieveBusinessesResponseOwner) -> Self {
        self.owner = Some(value);
        self
    }

    pub fn payout_percentages(
        mut self,
        value: RetrieveBusinessesResponsePayoutPercentages,
    ) -> Self {
        self.payout_percentages = Some(value);
        self
    }

    pub fn referral_expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.referral_expires_at = Some(value);
        self
    }

    pub fn referral_started_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.referral_started_at = Some(value);
        self
    }

    pub fn second_tier_partner(
        mut self,
        value: RetrieveBusinessesResponseSecondTierPartner,
    ) -> Self {
        self.second_tier_partner = Some(value);
        self
    }

    pub fn status(mut self, value: RetrieveBusinessesResponseStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn volume_usd(mut self, value: RetrieveBusinessesResponseVolumeUsd) -> Self {
        self.volume_usd = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBusinessesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](RetrieveBusinessesResponseBuilder::created_at)
    /// - [`earnings_usd`](RetrieveBusinessesResponseBuilder::earnings_usd)
    /// - [`id`](RetrieveBusinessesResponseBuilder::id)
    /// - [`my_partner_tier`](RetrieveBusinessesResponseBuilder::my_partner_tier)
    /// - [`object`](RetrieveBusinessesResponseBuilder::object)
    /// - [`payout_percentages`](RetrieveBusinessesResponseBuilder::payout_percentages)
    /// - [`status`](RetrieveBusinessesResponseBuilder::status)
    /// - [`volume_usd`](RetrieveBusinessesResponseBuilder::volume_usd)
    pub fn build(self) -> Result<RetrieveBusinessesResponse, BuildError> {
        Ok(RetrieveBusinessesResponse {
            account: self.account,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            earnings_usd: self
                .earnings_usd
                .ok_or_else(|| BuildError::missing_field("earnings_usd"))?,
            first_tier_partner: self.first_tier_partner,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            my_partner_tier: self
                .my_partner_tier
                .ok_or_else(|| BuildError::missing_field("my_partner_tier"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            owner: self.owner,
            payout_percentages: self
                .payout_percentages
                .ok_or_else(|| BuildError::missing_field("payout_percentages"))?,
            referral_expires_at: self.referral_expires_at,
            referral_started_at: self.referral_started_at,
            second_tier_partner: self.second_tier_partner,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            volume_usd: self
                .volume_usd
                .ok_or_else(|| BuildError::missing_field("volume_usd"))?,
        })
    }
}
