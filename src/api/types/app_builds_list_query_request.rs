pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppBuildsListQueryRequest {
    /// The app to list builds for, prefixed `app_`.
    #[serde(default)]
    pub app_id: String,
    /// Filter builds by target platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<ListAppBuildsRequestPlatform>,
    /// Filter builds by review status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListAppBuildsRequestStatus>,
    /// Only return builds created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<ListAppBuildsRequestCreatedBefore>,
    /// Only return builds created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<ListAppBuildsRequestCreatedAfter>,
    /// The number of builds to return (default 20, max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns builds after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of builds to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns builds before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl AppBuildsListQueryRequest {
    pub fn builder() -> AppBuildsListQueryRequestBuilder {
        <AppBuildsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppBuildsListQueryRequestBuilder {
    app_id: Option<String>,
    platform: Option<ListAppBuildsRequestPlatform>,
    status: Option<ListAppBuildsRequestStatus>,
    created_before: Option<ListAppBuildsRequestCreatedBefore>,
    created_after: Option<ListAppBuildsRequestCreatedAfter>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl AppBuildsListQueryRequestBuilder {
    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn platform(mut self, value: ListAppBuildsRequestPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn status(mut self, value: ListAppBuildsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_before(mut self, value: ListAppBuildsRequestCreatedBefore) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_after(mut self, value: ListAppBuildsRequestCreatedAfter) -> Self {
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

    /// Consumes the builder and constructs a [`AppBuildsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`app_id`](AppBuildsListQueryRequestBuilder::app_id)
    pub fn build(self) -> Result<AppBuildsListQueryRequest, BuildError> {
        Ok(AppBuildsListQueryRequest {
            app_id: self
                .app_id
                .ok_or_else(|| BuildError::missing_field("app_id"))?,
            platform: self.platform,
            status: self.status,
            created_before: self.created_before,
            created_after: self.created_after,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
