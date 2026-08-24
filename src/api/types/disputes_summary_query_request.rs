pub use crate::prelude::*;

/// Query parameters for summary
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputesSummaryQueryRequest {
    /// Which breakdowns to return, keyed by these names under `groups`. Repeat the parameter to ask for several; omit it for all of them.
    #[serde(default)]
    pub groups: Vec<Option<SummaryDisputesRequestGroupsItem>>,
    /// Only disputes filed against this account (`biz_` tag). Omit it to cover every account you can read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only disputes in these statuses. Repeat the parameter to pass several. A `needs_response` dispute whose evidence deadline has passed reports and filters as `under_review` instead.
    #[serde(default)]
    pub status: Vec<Option<SummaryDisputesRequestStatusItem>>,
    /// Only disputes in this three-letter ISO currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Only disputes opened before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only disputes opened after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
}

impl DisputesSummaryQueryRequest {
    pub fn builder() -> DisputesSummaryQueryRequestBuilder {
        <DisputesSummaryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputesSummaryQueryRequestBuilder {
    groups: Option<Vec<Option<SummaryDisputesRequestGroupsItem>>>,
    account_id: Option<String>,
    status: Option<Vec<Option<SummaryDisputesRequestStatusItem>>>,
    currency: Option<String>,
    created_before: Option<String>,
    created_after: Option<String>,
}

impl DisputesSummaryQueryRequestBuilder {
    pub fn groups(mut self, value: Vec<Option<SummaryDisputesRequestGroupsItem>>) -> Self {
        self.groups = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: Vec<Option<SummaryDisputesRequestStatusItem>>) -> Self {
        self.status = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputesSummaryQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`groups`](DisputesSummaryQueryRequestBuilder::groups)
    /// - [`status`](DisputesSummaryQueryRequestBuilder::status)
    pub fn build(self) -> Result<DisputesSummaryQueryRequest, BuildError> {
        Ok(DisputesSummaryQueryRequest {
            groups: self
                .groups
                .ok_or_else(|| BuildError::missing_field("groups"))?,
            account_id: self.account_id,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            currency: self.currency,
            created_before: self.created_before,
            created_after: self.created_after,
        })
    }
}
