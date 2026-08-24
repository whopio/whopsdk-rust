pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApiKeysListQueryRequest {
    /// The account (`biz_`) or app (`app_`) tag to list API keys for.
    #[serde(default)]
    pub resource_id: String,
    /// The type of resource that owns the API keys.
    pub resource_type: ListApiKeysRequestResourceType,
    /// Only return API keys created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<ListApiKeysRequestCreatedBefore>,
    /// Only return API keys created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<ListApiKeysRequestCreatedAfter>,
    /// The number of API keys to return (default 20, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns API keys after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of API keys to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns API keys before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The field to sort API keys by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListApiKeysRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListApiKeysRequestDirection>,
}

impl ApiKeysListQueryRequest {
    pub fn builder() -> ApiKeysListQueryRequestBuilder {
        <ApiKeysListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApiKeysListQueryRequestBuilder {
    resource_id: Option<String>,
    resource_type: Option<ListApiKeysRequestResourceType>,
    created_before: Option<ListApiKeysRequestCreatedBefore>,
    created_after: Option<ListApiKeysRequestCreatedAfter>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListApiKeysRequestOrder>,
    direction: Option<ListApiKeysRequestDirection>,
}

impl ApiKeysListQueryRequestBuilder {
    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn resource_type(mut self, value: ListApiKeysRequestResourceType) -> Self {
        self.resource_type = Some(value);
        self
    }

    pub fn created_before(mut self, value: ListApiKeysRequestCreatedBefore) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_after(mut self, value: ListApiKeysRequestCreatedAfter) -> Self {
        self.created_after = Some(value);
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

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListApiKeysRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListApiKeysRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApiKeysListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource_id`](ApiKeysListQueryRequestBuilder::resource_id)
    /// - [`resource_type`](ApiKeysListQueryRequestBuilder::resource_type)
    pub fn build(self) -> Result<ApiKeysListQueryRequest, BuildError> {
        Ok(ApiKeysListQueryRequest {
            resource_id: self
                .resource_id
                .ok_or_else(|| BuildError::missing_field("resource_id"))?,
            resource_type: self
                .resource_type
                .ok_or_else(|| BuildError::missing_field("resource_type"))?,
            created_before: self.created_before,
            created_after: self.created_after,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
        })
    }
}
