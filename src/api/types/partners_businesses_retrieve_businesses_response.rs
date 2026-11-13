pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrieveBusinessesResponse {
    /// Referred account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<RetrieveBusinessesResponseAccount>,
    /// The partner whose blueprint the business deployed. Null unless this is a blueprint referral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint_partner: Option<RetrieveBusinessesResponseBlueprintPartner>,
    /// Whether the caller can change the fees of this business. `true` for the Whop Verified Partner who referred it. Also `true` for a verified partner in the same team who manages that partner or can update every reporting line. Read and change the fees with the account fees endpoints.
    #[serde(default)]
    pub can_edit_fees: bool,
    /// CPA terms for the referring partner. Read with referring_account_id, the business's owner sees every rep's terms and a manager sees those of the reps below them, read-only. Omitted for other callers. An offer token is only for the referring partner and requires partner:commission_advance:accept permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_advance: Option<PartnerAdvance>,
    /// The most the referring organization advances per business, in USD. Returned only with commission_advance_rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_advance_cap: Option<Money>,
    /// Whether this referral meets the partner and ownership requirements for saving commission advance projections. Returned only to the referring partner; existing advance terms and update permissions still apply.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_advance_projections_allowed: Option<bool>,
    /// Estimated upfront advance per USD of annual card and BNPL payments, including advance share and remaining term. Multiply annual payments by this rate, cap at commission_advance_cap, and round down to cents. Uses observed commissions or, without history, a typical gross profit share adjusted for the business's US card fee, times the partner share. Null when projections or pricing are unavailable. Only returned to the referring partner. Qualification and final offers use actual activity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commission_advance_rate: Option<String>,
    /// When the partner business was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// How the partner closed this business, in their own words. Null until the partner saves one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deal_closed_reason: Option<String>,
    #[serde(default)]
    pub earnings_usd: RetrieveBusinessesResponseEarningsUsd,
    /// Estimated regular commission per USD of card and BNPL payments, as a decimal string (for example `0.004`). Once the advance qualifies, the rate its terms were sized with; before that, observed payout rates or, without payment history, reference US card fees. Multiply payments by this rate to estimate commissions, including those that resume after a commission advance term. `null` when pricing is unavailable or the response has neither `commission_advance_rate` nor `commission_advance`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_commission_rate: Option<String>,
    /// The partner who referred the business owner onto Whop (first tier). Null if there is no active first-tier partner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_tier_partner: Option<RetrieveBusinessesResponseFirstTierPartner>,
    /// Partner business ID.
    #[serde(default)]
    pub id: String,
    /// Which tier the caller earns on for this business: `first` (they referred the owner), `second` (they referred the first-tier partner), or `blueprint` (the business deployed a site from their blueprint).
    pub my_partner_tier: RetrieveBusinessesResponseMyPartnerTier,
    /// The caller's own cut of each income source on this business, as a fraction of the same base as `payout_percentages` (0.12 = 12%). A solo partner's cut is the whole rate. On a referring partner business's deal it is the caller's team share of the rate: a closer earns their share, a manager earns their share minus the share of the person below them, and a deal participant earns their cut. Null when the caller earns nothing on this business, as with an account API key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my_payout_percentages: Option<RetrieveBusinessesResponseMyPayoutPercentages>,
    pub object: RetrieveBusinessesResponseObject,
    /// The owner of the referred business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<RetrieveBusinessesResponseOwner>,
    /// The commission rate this referral pays for each income source, as a fraction (0.3 = 30%). A solo partner earns it directly; the team of a referring partner business splits it by their shares.
    #[serde(default)]
    pub payout_percentages: RetrieveBusinessesResponsePayoutPercentages,
    /// Outstanding invites to claim ownership of this business. Present only to the first-tier partner's whop.com session, or the Whop CLI or MCP with the `partner:handoff` scope, and empty once every invite has been accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_owner_invites: Option<Vec<RetrieveBusinessesResponsePendingOwnerInvitesItem>>,
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
    /// The partner business whose team member made this referral, the one its commission is paid to. Null for a solo partner's referral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referring_business: Option<RetrieveBusinessesResponseReferringBusiness>,
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
    blueprint_partner: Option<RetrieveBusinessesResponseBlueprintPartner>,
    can_edit_fees: Option<bool>,
    commission_advance: Option<PartnerAdvance>,
    commission_advance_cap: Option<Money>,
    commission_advance_projections_allowed: Option<bool>,
    commission_advance_rate: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    deal_closed_reason: Option<String>,
    earnings_usd: Option<RetrieveBusinessesResponseEarningsUsd>,
    estimated_commission_rate: Option<String>,
    first_tier_partner: Option<RetrieveBusinessesResponseFirstTierPartner>,
    id: Option<String>,
    my_partner_tier: Option<RetrieveBusinessesResponseMyPartnerTier>,
    my_payout_percentages: Option<RetrieveBusinessesResponseMyPayoutPercentages>,
    object: Option<RetrieveBusinessesResponseObject>,
    owner: Option<RetrieveBusinessesResponseOwner>,
    payout_percentages: Option<RetrieveBusinessesResponsePayoutPercentages>,
    pending_owner_invites: Option<Vec<RetrieveBusinessesResponsePendingOwnerInvitesItem>>,
    referral_expires_at: Option<DateTime<FixedOffset>>,
    referral_started_at: Option<DateTime<FixedOffset>>,
    referring_business: Option<RetrieveBusinessesResponseReferringBusiness>,
    second_tier_partner: Option<RetrieveBusinessesResponseSecondTierPartner>,
    status: Option<RetrieveBusinessesResponseStatus>,
    volume_usd: Option<RetrieveBusinessesResponseVolumeUsd>,
}

impl RetrieveBusinessesResponseBuilder {
    pub fn account(mut self, value: RetrieveBusinessesResponseAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn blueprint_partner(mut self, value: RetrieveBusinessesResponseBlueprintPartner) -> Self {
        self.blueprint_partner = Some(value);
        self
    }

    pub fn can_edit_fees(mut self, value: bool) -> Self {
        self.can_edit_fees = Some(value);
        self
    }

    pub fn commission_advance(mut self, value: PartnerAdvance) -> Self {
        self.commission_advance = Some(value);
        self
    }

    pub fn commission_advance_cap(mut self, value: Money) -> Self {
        self.commission_advance_cap = Some(value);
        self
    }

    pub fn commission_advance_projections_allowed(mut self, value: bool) -> Self {
        self.commission_advance_projections_allowed = Some(value);
        self
    }

    pub fn commission_advance_rate(mut self, value: impl Into<String>) -> Self {
        self.commission_advance_rate = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn deal_closed_reason(mut self, value: impl Into<String>) -> Self {
        self.deal_closed_reason = Some(value.into());
        self
    }

    pub fn earnings_usd(mut self, value: RetrieveBusinessesResponseEarningsUsd) -> Self {
        self.earnings_usd = Some(value);
        self
    }

    pub fn estimated_commission_rate(mut self, value: impl Into<String>) -> Self {
        self.estimated_commission_rate = Some(value.into());
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

    pub fn my_payout_percentages(
        mut self,
        value: RetrieveBusinessesResponseMyPayoutPercentages,
    ) -> Self {
        self.my_payout_percentages = Some(value);
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

    pub fn pending_owner_invites(
        mut self,
        value: Vec<RetrieveBusinessesResponsePendingOwnerInvitesItem>,
    ) -> Self {
        self.pending_owner_invites = Some(value);
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

    pub fn referring_business(
        mut self,
        value: RetrieveBusinessesResponseReferringBusiness,
    ) -> Self {
        self.referring_business = Some(value);
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
    /// - [`can_edit_fees`](RetrieveBusinessesResponseBuilder::can_edit_fees)
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
            blueprint_partner: self.blueprint_partner,
            can_edit_fees: self
                .can_edit_fees
                .ok_or_else(|| BuildError::missing_field("can_edit_fees"))?,
            commission_advance: self.commission_advance,
            commission_advance_cap: self.commission_advance_cap,
            commission_advance_projections_allowed: self.commission_advance_projections_allowed,
            commission_advance_rate: self.commission_advance_rate,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            deal_closed_reason: self.deal_closed_reason,
            earnings_usd: self
                .earnings_usd
                .ok_or_else(|| BuildError::missing_field("earnings_usd"))?,
            estimated_commission_rate: self.estimated_commission_rate,
            first_tier_partner: self.first_tier_partner,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            my_partner_tier: self
                .my_partner_tier
                .ok_or_else(|| BuildError::missing_field("my_partner_tier"))?,
            my_payout_percentages: self.my_payout_percentages,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            owner: self.owner,
            payout_percentages: self
                .payout_percentages
                .ok_or_else(|| BuildError::missing_field("payout_percentages"))?,
            pending_owner_invites: self.pending_owner_invites,
            referral_expires_at: self.referral_expires_at,
            referral_started_at: self.referral_started_at,
            referring_business: self.referring_business,
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
