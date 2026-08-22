pub use crate::prelude::*;

/// How many of the matching cases were opened for each reason. Every reason is present, including those with a count of zero.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SummaryResolutionCenterCasesResponseGroupsReason {
    #[serde(default)]
    pub fraudulent: i64,
    #[serde(default)]
    pub not_as_described: i64,
    #[serde(default)]
    pub product_not_received: i64,
    #[serde(default)]
    pub product_unacceptable: i64,
    #[serde(default)]
    pub subscription_canceled: i64,
}

impl SummaryResolutionCenterCasesResponseGroupsReason {
    pub fn builder() -> SummaryResolutionCenterCasesResponseGroupsReasonBuilder {
        <SummaryResolutionCenterCasesResponseGroupsReasonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryResolutionCenterCasesResponseGroupsReasonBuilder {
    fraudulent: Option<i64>,
    not_as_described: Option<i64>,
    product_not_received: Option<i64>,
    product_unacceptable: Option<i64>,
    subscription_canceled: Option<i64>,
}

impl SummaryResolutionCenterCasesResponseGroupsReasonBuilder {
    pub fn fraudulent(mut self, value: i64) -> Self {
        self.fraudulent = Some(value);
        self
    }

    pub fn not_as_described(mut self, value: i64) -> Self {
        self.not_as_described = Some(value);
        self
    }

    pub fn product_not_received(mut self, value: i64) -> Self {
        self.product_not_received = Some(value);
        self
    }

    pub fn product_unacceptable(mut self, value: i64) -> Self {
        self.product_unacceptable = Some(value);
        self
    }

    pub fn subscription_canceled(mut self, value: i64) -> Self {
        self.subscription_canceled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SummaryResolutionCenterCasesResponseGroupsReason`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fraudulent`](SummaryResolutionCenterCasesResponseGroupsReasonBuilder::fraudulent)
    /// - [`not_as_described`](SummaryResolutionCenterCasesResponseGroupsReasonBuilder::not_as_described)
    /// - [`product_not_received`](SummaryResolutionCenterCasesResponseGroupsReasonBuilder::product_not_received)
    /// - [`product_unacceptable`](SummaryResolutionCenterCasesResponseGroupsReasonBuilder::product_unacceptable)
    /// - [`subscription_canceled`](SummaryResolutionCenterCasesResponseGroupsReasonBuilder::subscription_canceled)
    pub fn build(self) -> Result<SummaryResolutionCenterCasesResponseGroupsReason, BuildError> {
        Ok(SummaryResolutionCenterCasesResponseGroupsReason {
            fraudulent: self
                .fraudulent
                .ok_or_else(|| BuildError::missing_field("fraudulent"))?,
            not_as_described: self
                .not_as_described
                .ok_or_else(|| BuildError::missing_field("not_as_described"))?,
            product_not_received: self
                .product_not_received
                .ok_or_else(|| BuildError::missing_field("product_not_received"))?,
            product_unacceptable: self
                .product_unacceptable
                .ok_or_else(|| BuildError::missing_field("product_unacceptable"))?,
            subscription_canceled: self
                .subscription_canceled
                .ok_or_else(|| BuildError::missing_field("subscription_canceled"))?,
        })
    }
}
