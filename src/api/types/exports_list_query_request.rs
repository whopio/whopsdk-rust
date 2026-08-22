pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportsListQueryRequest {
    /// The account to list exports for, prefixed `biz_`. Defaults to the credential's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only return exports of this resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource: Option<ListExportsRequestResource>,
    /// Only return exports in this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListExportsRequestStatus>,
    /// Only return exports created at or after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Only return exports created at or before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// The field to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListExportsRequestOrder>,
    /// The sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListExportsRequestDirection>,
}

impl ExportsListQueryRequest {
    pub fn builder() -> ExportsListQueryRequestBuilder {
        <ExportsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportsListQueryRequestBuilder {
    account_id: Option<String>,
    resource: Option<ListExportsRequestResource>,
    status: Option<ListExportsRequestStatus>,
    created_after: Option<String>,
    created_before: Option<String>,
    order: Option<ListExportsRequestOrder>,
    direction: Option<ListExportsRequestDirection>,
}

impl ExportsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn resource(mut self, value: ListExportsRequestResource) -> Self {
        self.resource = Some(value);
        self
    }

    pub fn status(mut self, value: ListExportsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListExportsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListExportsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportsListQueryRequest`].
    pub fn build(self) -> Result<ExportsListQueryRequest, BuildError> {
        Ok(ExportsListQueryRequest {
            account_id: self.account_id,
            resource: self.resource,
            status: self.status,
            created_after: self.created_after,
            created_before: self.created_before,
            order: self.order,
            direction: self.direction,
        })
    }
}
