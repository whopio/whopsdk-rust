pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Partner {
    /// When the user joined the partner program, as an ISO 8601 timestamp. Null when they have not joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<String>,
    #[serde(default)]
    pub payout_rates: Vec<PartnerPayoutTier>,
    /// Number of active first-tier business referrals attributed to the partner, excluding deleted businesses.
    #[serde(default)]
    pub referred_businesses_count: i64,
    /// The authenticated partner's public profile.
    #[serde(default)]
    pub user: UserSummary,
    /// Whether the user has a pending or approved personal entry on the Verified Partner waitlist.
    #[serde(default)]
    pub verification_waitlist_joined: bool,
    /// When the user became a verified Whop Partner, as an ISO 8601 timestamp. `null` if not verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whop_partner_verified_at: Option<String>,
}

impl Partner {
    pub fn builder() -> PartnerBuilder {
        <PartnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerBuilder {
    joined_at: Option<String>,
    payout_rates: Option<Vec<PartnerPayoutTier>>,
    referred_businesses_count: Option<i64>,
    user: Option<UserSummary>,
    verification_waitlist_joined: Option<bool>,
    whop_partner_verified_at: Option<String>,
}

impl PartnerBuilder {
    pub fn joined_at(mut self, value: impl Into<String>) -> Self {
        self.joined_at = Some(value.into());
        self
    }

    pub fn payout_rates(mut self, value: Vec<PartnerPayoutTier>) -> Self {
        self.payout_rates = Some(value);
        self
    }

    pub fn referred_businesses_count(mut self, value: i64) -> Self {
        self.referred_businesses_count = Some(value);
        self
    }

    pub fn user(mut self, value: UserSummary) -> Self {
        self.user = Some(value);
        self
    }

    pub fn verification_waitlist_joined(mut self, value: bool) -> Self {
        self.verification_waitlist_joined = Some(value);
        self
    }

    pub fn whop_partner_verified_at(mut self, value: impl Into<String>) -> Self {
        self.whop_partner_verified_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Partner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payout_rates`](PartnerBuilder::payout_rates)
    /// - [`referred_businesses_count`](PartnerBuilder::referred_businesses_count)
    /// - [`user`](PartnerBuilder::user)
    /// - [`verification_waitlist_joined`](PartnerBuilder::verification_waitlist_joined)
    pub fn build(self) -> Result<Partner, BuildError> {
        Ok(Partner {
            joined_at: self.joined_at,
            payout_rates: self
                .payout_rates
                .ok_or_else(|| BuildError::missing_field("payout_rates"))?,
            referred_businesses_count: self
                .referred_businesses_count
                .ok_or_else(|| BuildError::missing_field("referred_businesses_count"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
            verification_waitlist_joined: self
                .verification_waitlist_joined
                .ok_or_else(|| BuildError::missing_field("verification_waitlist_joined"))?,
            whop_partner_verified_at: self.whop_partner_verified_at,
        })
    }
}
