pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppsListQueryRequest {
    /// Only return apps created by this account (`biz_` tag). With developer access to the account this includes its unlisted and hidden apps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Filter apps by the type of end-user they are built for. Apps of type `website` are left out unless you ask for them by name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<ListAppsRequestAppType>,
    /// Only return apps supporting this view type, such as `dashboard` or `hub`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<ListAppsRequestViewType>,
    /// Only return apps whose Whop verification status matches this value. Omit this filter to include every verification status the caller can see.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified: Option<bool>,
    /// Legacy compatibility filter. Use `verified` for field equality. `true` returns verified apps; clients pinned before `2026-08-25-2` retain the earlier public website discovery behavior.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_apps_only: Option<bool>,
    /// Only return apps Whop recommends (or, with `false`, only those it does not), independently of verification status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended: Option<bool>,
    /// A search string matched against app names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    /// Only return apps whose `product_category` is this product category, such as `pressure_washing`. An unknown product category is rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_category: Option<String>,
    /// Only return apps whose `product_category` sits under this business type in Whop's business taxonomy, such as `services`. An unknown business type is rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<String>,
    /// Only return apps whose `product_category` sits under this industry type of `business_type`, such as `exterior_cleaning`. Requires `business_type`. An industry type outside it is rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub industry_type: Option<String>,
    /// Only return apps in this app store category. Use a `route` from `GET /apps/categories`, such as `business-productivity`. An unknown route is rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_category_route: Option<String>,
    /// The field to sort apps by. Defaults to discoverable_at, showing the most recently published apps first. `template_usage` ranks Whop-verified apps first, then by how many businesses created apps from each app as a template. `template_recent_sales` ranks them by how many of those businesses processed a payment in the last 24 hours, recounted hourly, breaking ties by `template_usage`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListAppsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListAppsRequestDirection>,
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
    /// Set to `true` to return the number of matching results in `page_info.total_count`. Send `first=0` as well to return the count without any results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_total_count: Option<bool>,
}

impl AppsListQueryRequest {
    pub fn builder() -> AppsListQueryRequestBuilder {
        <AppsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppsListQueryRequestBuilder {
    account_id: Option<String>,
    app_type: Option<ListAppsRequestAppType>,
    view_type: Option<ListAppsRequestViewType>,
    verified: Option<bool>,
    verified_apps_only: Option<bool>,
    recommended: Option<bool>,
    query: Option<String>,
    product_category: Option<String>,
    business_type: Option<String>,
    industry_type: Option<String>,
    marketplace_category_route: Option<String>,
    order: Option<ListAppsRequestOrder>,
    direction: Option<ListAppsRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    include_total_count: Option<bool>,
}

impl AppsListQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn app_type(mut self, value: ListAppsRequestAppType) -> Self {
        self.app_type = Some(value);
        self
    }

    pub fn view_type(mut self, value: ListAppsRequestViewType) -> Self {
        self.view_type = Some(value);
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    pub fn verified_apps_only(mut self, value: bool) -> Self {
        self.verified_apps_only = Some(value);
        self
    }

    pub fn recommended(mut self, value: bool) -> Self {
        self.recommended = Some(value);
        self
    }

    pub fn query(mut self, value: impl Into<String>) -> Self {
        self.query = Some(value.into());
        self
    }

    pub fn product_category(mut self, value: impl Into<String>) -> Self {
        self.product_category = Some(value.into());
        self
    }

    pub fn business_type(mut self, value: impl Into<String>) -> Self {
        self.business_type = Some(value.into());
        self
    }

    pub fn industry_type(mut self, value: impl Into<String>) -> Self {
        self.industry_type = Some(value.into());
        self
    }

    pub fn marketplace_category_route(mut self, value: impl Into<String>) -> Self {
        self.marketplace_category_route = Some(value.into());
        self
    }

    pub fn order(mut self, value: ListAppsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListAppsRequestDirection) -> Self {
        self.direction = Some(value);
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

    pub fn include_total_count(mut self, value: bool) -> Self {
        self.include_total_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppsListQueryRequest`].
    pub fn build(self) -> Result<AppsListQueryRequest, BuildError> {
        Ok(AppsListQueryRequest {
            account_id: self.account_id,
            app_type: self.app_type,
            view_type: self.view_type,
            verified: self.verified,
            verified_apps_only: self.verified_apps_only,
            recommended: self.recommended,
            query: self.query,
            product_category: self.product_category,
            business_type: self.business_type,
            industry_type: self.industry_type,
            marketplace_category_route: self.marketplace_category_route,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            include_total_count: self.include_total_count,
        })
    }
}
