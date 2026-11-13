pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReportIssuesAdsRequestIssuesItem {
    /// The policy a `content_rejected` ad broke, which the account reads in its message. Send it only with `content_rejected`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<ReportIssuesAdsRequestIssuesItemPolicy>,
    /// Why the object can't deliver yet. `content_rejected`, `landing_page_rejected`, and `creative_unsupported` go on ads, `targeting_rejected` on ad groups, and `in_review` on either.
    pub reason: ReportIssuesAdsRequestIssuesItemReason,
}

impl ReportIssuesAdsRequestIssuesItem {
    pub fn builder() -> ReportIssuesAdsRequestIssuesItemBuilder {
        <ReportIssuesAdsRequestIssuesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReportIssuesAdsRequestIssuesItemBuilder {
    policy: Option<ReportIssuesAdsRequestIssuesItemPolicy>,
    reason: Option<ReportIssuesAdsRequestIssuesItemReason>,
}

impl ReportIssuesAdsRequestIssuesItemBuilder {
    pub fn policy(mut self, value: ReportIssuesAdsRequestIssuesItemPolicy) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn reason(mut self, value: ReportIssuesAdsRequestIssuesItemReason) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReportIssuesAdsRequestIssuesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reason`](ReportIssuesAdsRequestIssuesItemBuilder::reason)
    pub fn build(self) -> Result<ReportIssuesAdsRequestIssuesItem, BuildError> {
        Ok(ReportIssuesAdsRequestIssuesItem {
            policy: self.policy,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
