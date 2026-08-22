pub use crate::prelude::*;

/// Query parameters for describeStats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DescribeStatsQueryRequest {
    /// Resource path using : as separator (e.g., 'receipts', 'payments:membership', 'receipts:gross_revenue').
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<String>,
    /// Scope query to a specific company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// Scope query to a specific user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl DescribeStatsQueryRequest {
    pub fn builder() -> DescribeStatsQueryRequestBuilder {
        <DescribeStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DescribeStatsQueryRequestBuilder {
    resource: Option<String>,
    company_id: Option<String>,
    user_id: Option<String>,
}

impl DescribeStatsQueryRequestBuilder {
    pub fn resource(mut self, value: impl Into<String>) -> Self {
        self.resource = Some(value.into());
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DescribeStatsQueryRequest`].
    pub fn build(self) -> Result<DescribeStatsQueryRequest, BuildError> {
        Ok(DescribeStatsQueryRequest {
            resource: self.resource,
            company_id: self.company_id,
            user_id: self.user_id,
        })
    }
}
