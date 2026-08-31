pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApiLogsListQueryRequest {
    /// The account (biz_*) whose API logs to list. Defaults to the authenticated account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only return requests served at or after this ISO 8601 timestamp. Defaults to 7 days before created_before, or 7 days ago.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Only return requests served before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only return requests for this operation, matched exactly against the operation_name shown on each log row (for example api/v1/products#create).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    /// Only return requests made with this HTTP method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<ListApiLogsRequestHttpMethod>,
    /// Only return requests that finished with this status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListApiLogsRequestStatus>,
    /// Only return requests made with this API key (apik_…).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_id: Option<String>,
    /// Only return requests that took at least this many milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_duration_ms: Option<i64>,
    /// Only return requests that took at most this many milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_ms: Option<i64>,
    /// Number of logs to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Cursor to fetch the page after (from page_info.end_cursor).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl ApiLogsListQueryRequest {
    pub fn builder() -> ApiLogsListQueryRequestBuilder {
        <ApiLogsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiLogsListQueryRequestBuilder {
    account_id: Option<String>,
    created_after: Option<String>,
    created_before: Option<String>,
    operation_name: Option<String>,
    http_method: Option<ListApiLogsRequestHttpMethod>,
    status: Option<ListApiLogsRequestStatus>,
    api_key_id: Option<String>,
    min_duration_ms: Option<i64>,
    max_duration_ms: Option<i64>,
    first: Option<i64>,
    after: Option<String>,
}

impl ApiLogsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
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

    pub fn operation_name(mut self, value: impl Into<String>) -> Self {
        self.operation_name = Some(value.into());
        self
    }

    pub fn http_method(mut self, value: ListApiLogsRequestHttpMethod) -> Self {
        self.http_method = Some(value);
        self
    }

    pub fn status(mut self, value: ListApiLogsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn api_key_id(mut self, value: impl Into<String>) -> Self {
        self.api_key_id = Some(value.into());
        self
    }

    pub fn min_duration_ms(mut self, value: i64) -> Self {
        self.min_duration_ms = Some(value);
        self
    }

    pub fn max_duration_ms(mut self, value: i64) -> Self {
        self.max_duration_ms = Some(value);
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApiLogsListQueryRequest`].
    pub fn build(self) -> Result<ApiLogsListQueryRequest, BuildError> {
        Ok(ApiLogsListQueryRequest {
            account_id: self.account_id,
            created_after: self.created_after,
            created_before: self.created_before,
            operation_name: self.operation_name,
            http_method: self.http_method,
            status: self.status,
            api_key_id: self.api_key_id,
            min_duration_ms: self.min_duration_ms,
            max_duration_ms: self.max_duration_ms,
            first: self.first,
            after: self.after,
        })
    }
}
