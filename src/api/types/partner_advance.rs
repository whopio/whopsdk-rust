pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PartnerAdvance {
    /// Qualifying card and BNPL payment volume in the preceding 30 days, in USD. Available on business detail responses before acceptance.
    #[serde(rename = "activity_30_day")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity30day: Option<Money>,
    /// Actual payment volume (cards and BNPL) in the preceding 60 days, in USD. Null when unavailable.
    #[serde(rename = "activity_60_day")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity60day: Option<Money>,
    /// The offered or accepted upfront CPA payment in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Money>,
    /// Annual projected processing in USD.
    #[serde(default)]
    pub annual_processing: Money,
    /// When low activity triggered reversal and normal partner terms resumed. No prior commissions are paid retroactively.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clawed_back_at: Option<String>,
    /// End of the nine-month CPA term. Normal commissions resume prospectively without backfill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deadline_at: Option<String>,
    /// Estimated advance using the current CPA formula and observed payout rates, or current reference card pricing when qualifying payment history is unavailable. Available for unqualified projections on business detail responses; not a final offer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_amount: Option<Money>,
    /// When the CPA payment was funded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funded_at: Option<String>,
    /// Signed offer valid for 15 minutes. Submit unchanged when accepting. Null unless the caller may accept this offer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_token: Option<String>,
    /// Processing required in a rolling 30-day period to qualify, in USD.
    #[serde(default)]
    pub processing_target: Money,
    /// Projected payment volume (cards and BNPL) over 60 days, in USD.
    #[serde(rename = "projected_60_day_activity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected60day_activity: Option<Money>,
    /// Projected commissions over the remaining CPA term.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_commissions: Option<Money>,
    /// Processing commissions projected for the remaining CPA term, in USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub projected_processing_commissions: Option<Money>,
    /// First qualification timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qualified_at: Option<String>,
    /// 20% of projected 60-day payment volume (cards and BNPL). Performance checks begin 60 days after funding.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovery_threshold: Option<Money>,
    /// The CPA payment lifecycle state.
    pub status: PartnerAdvanceStatus,
    /// Terms version required for acceptance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_version: Option<String>,
}

impl PartnerAdvance {
    pub fn builder() -> PartnerAdvanceBuilder {
        <PartnerAdvanceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnerAdvanceBuilder {
    activity30day: Option<Money>,
    activity60day: Option<Money>,
    amount: Option<Money>,
    annual_processing: Option<Money>,
    clawed_back_at: Option<String>,
    deadline_at: Option<String>,
    estimated_amount: Option<Money>,
    funded_at: Option<String>,
    offer_token: Option<String>,
    processing_target: Option<Money>,
    projected60day_activity: Option<Money>,
    projected_commissions: Option<Money>,
    projected_processing_commissions: Option<Money>,
    qualified_at: Option<String>,
    recovery_threshold: Option<Money>,
    status: Option<PartnerAdvanceStatus>,
    terms_version: Option<String>,
}

impl PartnerAdvanceBuilder {
    pub fn activity30day(mut self, value: Money) -> Self {
        self.activity30day = Some(value);
        self
    }

    pub fn activity60day(mut self, value: Money) -> Self {
        self.activity60day = Some(value);
        self
    }

    pub fn amount(mut self, value: Money) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn annual_processing(mut self, value: Money) -> Self {
        self.annual_processing = Some(value);
        self
    }

    pub fn clawed_back_at(mut self, value: impl Into<String>) -> Self {
        self.clawed_back_at = Some(value.into());
        self
    }

    pub fn deadline_at(mut self, value: impl Into<String>) -> Self {
        self.deadline_at = Some(value.into());
        self
    }

    pub fn estimated_amount(mut self, value: Money) -> Self {
        self.estimated_amount = Some(value);
        self
    }

    pub fn funded_at(mut self, value: impl Into<String>) -> Self {
        self.funded_at = Some(value.into());
        self
    }

    pub fn offer_token(mut self, value: impl Into<String>) -> Self {
        self.offer_token = Some(value.into());
        self
    }

    pub fn processing_target(mut self, value: Money) -> Self {
        self.processing_target = Some(value);
        self
    }

    pub fn projected60day_activity(mut self, value: Money) -> Self {
        self.projected60day_activity = Some(value);
        self
    }

    pub fn projected_commissions(mut self, value: Money) -> Self {
        self.projected_commissions = Some(value);
        self
    }

    pub fn projected_processing_commissions(mut self, value: Money) -> Self {
        self.projected_processing_commissions = Some(value);
        self
    }

    pub fn qualified_at(mut self, value: impl Into<String>) -> Self {
        self.qualified_at = Some(value.into());
        self
    }

    pub fn recovery_threshold(mut self, value: Money) -> Self {
        self.recovery_threshold = Some(value);
        self
    }

    pub fn status(mut self, value: PartnerAdvanceStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn terms_version(mut self, value: impl Into<String>) -> Self {
        self.terms_version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PartnerAdvance`].
    /// This method will fail if any of the following fields are not set:
    /// - [`annual_processing`](PartnerAdvanceBuilder::annual_processing)
    /// - [`processing_target`](PartnerAdvanceBuilder::processing_target)
    /// - [`status`](PartnerAdvanceBuilder::status)
    pub fn build(self) -> Result<PartnerAdvance, BuildError> {
        Ok(PartnerAdvance {
            activity30day: self.activity30day,
            activity60day: self.activity60day,
            amount: self.amount,
            annual_processing: self
                .annual_processing
                .ok_or_else(|| BuildError::missing_field("annual_processing"))?,
            clawed_back_at: self.clawed_back_at,
            deadline_at: self.deadline_at,
            estimated_amount: self.estimated_amount,
            funded_at: self.funded_at,
            offer_token: self.offer_token,
            processing_target: self
                .processing_target
                .ok_or_else(|| BuildError::missing_field("processing_target"))?,
            projected60day_activity: self.projected60day_activity,
            projected_commissions: self.projected_commissions,
            projected_processing_commissions: self.projected_processing_commissions,
            qualified_at: self.qualified_at,
            recovery_threshold: self.recovery_threshold,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            terms_version: self.terms_version,
        })
    }
}
