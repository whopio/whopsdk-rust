pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppInstallationsListQueryRequest {
    /// Only return installations on this account, prefixed `biz_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Only return installations of this app, prefixed `app_`. Pass your own app's ID to list the accounts that installed it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Only return installations of apps that have this view, such as `dashboard` for apps shown in a business's dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<ListAppInstallationsRequestViewType>,
    /// Only return the installation with this `external_id`, the installed app's own identifier for it. Only the app and its developer can filter by it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Only return installations created before this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only return installations created after this ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Field to sort installations by. `name` sorts by app name, and only when listing the apps installed on one account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListAppInstallationsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListAppInstallationsRequestDirection>,
}

impl AppInstallationsListQueryRequest {
    pub fn builder() -> AppInstallationsListQueryRequestBuilder {
        <AppInstallationsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppInstallationsListQueryRequestBuilder {
    account_id: Option<String>,
    app_id: Option<String>,
    view_type: Option<ListAppInstallationsRequestViewType>,
    external_id: Option<String>,
    created_before: Option<String>,
    created_after: Option<String>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListAppInstallationsRequestOrder>,
    direction: Option<ListAppInstallationsRequestDirection>,
}

impl AppInstallationsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn view_type(mut self, value: ListAppInstallationsRequestViewType) -> Self {
        self.view_type = Some(value);
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
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

    pub fn order(mut self, value: ListAppInstallationsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListAppInstallationsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppInstallationsListQueryRequest`].
    pub fn build(self) -> Result<AppInstallationsListQueryRequest, BuildError> {
        Ok(AppInstallationsListQueryRequest {
            account_id: self.account_id,
            app_id: self.app_id,
            view_type: self.view_type,
            external_id: self.external_id,
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
