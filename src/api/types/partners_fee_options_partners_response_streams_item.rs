pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FeeOptionsPartnersResponseStreamsItem {
    /// Whether a partner may lower the fee on this stream in exchange for a cut to their commission on it.
    #[serde(default)]
    pub adjustable: bool,
    /// The partner's commission on this stream, in percent units (30 means 30%).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub commission_percentage: Option<f64>,
    /// The merchant fee behind this stream, or null when the business is charged nothing on it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<FeeOptionsPartnersResponseStreamsItemFee>,
    /// The income stream these terms describe.
    pub income_stream: FeeOptionsPartnersResponseStreamsItemIncomeStream,
    /// Why the stream cannot be adjusted: `no_merchant_fee` (the business is charged nothing on it), `commission_fixed` (the commission is a flat share a fee discount may not move), `no_headroom` (already at the floor), `fee_unavailable` (no fee or rate could be resolved). Null when adjustable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unadjustable_reason: Option<FeeOptionsPartnersResponseStreamsItemUnadjustableReason>,
}

impl FeeOptionsPartnersResponseStreamsItem {
    pub fn builder() -> FeeOptionsPartnersResponseStreamsItemBuilder {
        <FeeOptionsPartnersResponseStreamsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeeOptionsPartnersResponseStreamsItemBuilder {
    adjustable: Option<bool>,
    commission_percentage: Option<f64>,
    fee: Option<FeeOptionsPartnersResponseStreamsItemFee>,
    income_stream: Option<FeeOptionsPartnersResponseStreamsItemIncomeStream>,
    unadjustable_reason: Option<FeeOptionsPartnersResponseStreamsItemUnadjustableReason>,
}

impl FeeOptionsPartnersResponseStreamsItemBuilder {
    pub fn adjustable(mut self, value: bool) -> Self {
        self.adjustable = Some(value);
        self
    }

    pub fn commission_percentage(mut self, value: f64) -> Self {
        self.commission_percentage = Some(value);
        self
    }

    pub fn fee(mut self, value: FeeOptionsPartnersResponseStreamsItemFee) -> Self {
        self.fee = Some(value);
        self
    }

    pub fn income_stream(
        mut self,
        value: FeeOptionsPartnersResponseStreamsItemIncomeStream,
    ) -> Self {
        self.income_stream = Some(value);
        self
    }

    pub fn unadjustable_reason(
        mut self,
        value: FeeOptionsPartnersResponseStreamsItemUnadjustableReason,
    ) -> Self {
        self.unadjustable_reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeeOptionsPartnersResponseStreamsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`adjustable`](FeeOptionsPartnersResponseStreamsItemBuilder::adjustable)
    /// - [`income_stream`](FeeOptionsPartnersResponseStreamsItemBuilder::income_stream)
    pub fn build(self) -> Result<FeeOptionsPartnersResponseStreamsItem, BuildError> {
        Ok(FeeOptionsPartnersResponseStreamsItem {
            adjustable: self
                .adjustable
                .ok_or_else(|| BuildError::missing_field("adjustable"))?,
            commission_percentage: self.commission_percentage,
            fee: self.fee,
            income_stream: self
                .income_stream
                .ok_or_else(|| BuildError::missing_field("income_stream"))?,
            unadjustable_reason: self.unadjustable_reason,
        })
    }
}
