pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PartnerPayoutTier {
    /// Default period during which a new referred business can generate commissions, measured from its attribution start. This is not a payout delay. Individual business terms can differ.
    pub duration: PartnerPayoutDuration,
    #[serde(default)]
    pub rates: Vec<PartnerPayoutRate>,
    /// Referral tier: first for a directly referred business, or second for a business brought by a referred partner.
    pub tier: PartnerPayoutTierTier,
}

impl PartnerPayoutTier {
    pub fn builder() -> PartnerPayoutTierBuilder {
        <PartnerPayoutTierBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerPayoutTierBuilder {
    duration: Option<PartnerPayoutDuration>,
    rates: Option<Vec<PartnerPayoutRate>>,
    tier: Option<PartnerPayoutTierTier>,
}

impl PartnerPayoutTierBuilder {
    pub fn duration(mut self, value: PartnerPayoutDuration) -> Self {
        self.duration = Some(value);
        self
    }

    pub fn rates(mut self, value: Vec<PartnerPayoutRate>) -> Self {
        self.rates = Some(value);
        self
    }

    pub fn tier(mut self, value: PartnerPayoutTierTier) -> Self {
        self.tier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PartnerPayoutTier`].
    /// This method will fail if any of the following fields are not set:
    /// - [`duration`](PartnerPayoutTierBuilder::duration)
    /// - [`rates`](PartnerPayoutTierBuilder::rates)
    /// - [`tier`](PartnerPayoutTierBuilder::tier)
    pub fn build(self) -> Result<PartnerPayoutTier, BuildError> {
        Ok(PartnerPayoutTier {
            duration: self
                .duration
                .ok_or_else(|| BuildError::missing_field("duration"))?,
            rates: self
                .rates
                .ok_or_else(|| BuildError::missing_field("rates"))?,
            tier: self.tier.ok_or_else(|| BuildError::missing_field("tier"))?,
        })
    }
}
