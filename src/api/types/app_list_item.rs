pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppListItem {
    /// The account that owns the app.
    #[serde(default)]
    pub account: AccountParent,
    /// The type of end-user the app is built for.
    pub app_type: AppListItemAppType,
    /// Average star rating across published reviews for this app, from `1.0` to `5.0`. Returns `0.0` when no published-review rating is available.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub average_review_rating: f64,
    /// Banner image from the app's product listing, or `null` when none is uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<AppBannerImage>,
    /// The production base URL where the app is hosted. `null` if no base URL is configured, if the caller lacks the `developer:basic:read` permission on the app's account, or on list responses, which never expose it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// Number of businesses created from this app as a template.
    #[serde(default)]
    pub businesses_created_count: i64,
    #[serde(default)]
    pub businesses_created_logo_urls: Vec<String>,
    /// Number of businesses created from this app as a template that processed a payment in the last 24 hours. Recounted hourly, so it can be up to an hour behind.
    #[serde(default)]
    pub businesses_with_recent_sales_count: i64,
    /// The app's App Store category, or `null` when none has been set. An app can show a category that `GET /apps/categories` does not list, if it was categorized before the current store categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<AppCategory>,
    /// The user who owns the publishing account.
    #[serde(default)]
    pub creator: AppCreator,
    /// URL path for the account dashboard view, or `null` when not configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_path: Option<String>,
    /// Short description shown in listings and search results, or `null` if none has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// URL path for the discover view, or `null` when not configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover_path: Option<String>,
    /// Subdomain identifier for the app's proxied URL, forming https://{domain_id}.apps.whop.com.
    #[serde(default)]
    pub domain_id: String,
    #[serde(default)]
    pub domains: Vec<AppDomain>,
    /// URL path for the member-facing hub view, or `null` when not configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_path: Option<String>,
    /// Full URL where the app's hosted web build is served, or `null` if no route is claimed.
    /// DEPRECATED: Read `domains` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_url: Option<String>,
    /// The app's icon. Falls back to the default app icon when none is uploaded.
    #[serde(default)]
    pub icon: AppIcon,
    /// App ID, prefixed `app_`.
    #[serde(default)]
    pub id: String,
    /// Whether Whop publishes this app.
    #[serde(default)]
    pub is_whop_app: bool,
    /// Display name shown on the app store and in experience navigation.
    #[serde(default)]
    pub name: String,
    /// URL path to the app's OpenAPI spec file, or `null` when not configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openapi_path: Option<String>,
    /// Full origin URL of the app's proxied domain, for example https://ab1c2d3e4f.apps.whop.com.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Product category a business built from this app as a template sells, such as `pressure_washing`, or `null` when none has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_category: Option<String>,
    /// Published customer reviews for this app.
    #[serde(default)]
    pub published_reviews_count: i64,
    /// Claimed subdomain route where hosted web builds are served (`myapp` for myapp.whop.site), or `null` if no route is claimed.
    /// DEPRECATED: Read `domains` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// URL path to the app's skills directory, or `null` when not configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills_path: Option<String>,
    /// Visibility on the Whop app store: `live` is publicly discoverable, `unlisted` is accessible only via direct link, `hidden` is not visible anywhere.
    pub status: AppListItemStatus,
    /// Whether the app has been verified by Whop and is eligible for the featured apps section.
    #[serde(default)]
    pub verified: bool,
}

impl AppListItem {
    pub fn builder() -> AppListItemBuilder {
        <AppListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppListItemBuilder {
    account: Option<AccountParent>,
    app_type: Option<AppListItemAppType>,
    average_review_rating: Option<f64>,
    banner_image: Option<AppBannerImage>,
    base_url: Option<String>,
    businesses_created_count: Option<i64>,
    businesses_created_logo_urls: Option<Vec<String>>,
    businesses_with_recent_sales_count: Option<i64>,
    category: Option<AppCategory>,
    creator: Option<AppCreator>,
    dashboard_path: Option<String>,
    description: Option<String>,
    discover_path: Option<String>,
    domain_id: Option<String>,
    domains: Option<Vec<AppDomain>>,
    experience_path: Option<String>,
    hosted_url: Option<String>,
    icon: Option<AppIcon>,
    id: Option<String>,
    is_whop_app: Option<bool>,
    name: Option<String>,
    openapi_path: Option<String>,
    origin: Option<String>,
    product_category: Option<String>,
    published_reviews_count: Option<i64>,
    route: Option<String>,
    skills_path: Option<String>,
    status: Option<AppListItemStatus>,
    verified: Option<bool>,
}

impl AppListItemBuilder {
    pub fn account(mut self, value: AccountParent) -> Self {
        self.account = Some(value);
        self
    }

    pub fn app_type(mut self, value: AppListItemAppType) -> Self {
        self.app_type = Some(value);
        self
    }

    pub fn average_review_rating(mut self, value: f64) -> Self {
        self.average_review_rating = Some(value);
        self
    }

    pub fn banner_image(mut self, value: AppBannerImage) -> Self {
        self.banner_image = Some(value);
        self
    }

    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = Some(value.into());
        self
    }

    pub fn businesses_created_count(mut self, value: i64) -> Self {
        self.businesses_created_count = Some(value);
        self
    }

    pub fn businesses_created_logo_urls(mut self, value: Vec<String>) -> Self {
        self.businesses_created_logo_urls = Some(value);
        self
    }

    pub fn businesses_with_recent_sales_count(mut self, value: i64) -> Self {
        self.businesses_with_recent_sales_count = Some(value);
        self
    }

    pub fn category(mut self, value: AppCategory) -> Self {
        self.category = Some(value);
        self
    }

    pub fn creator(mut self, value: AppCreator) -> Self {
        self.creator = Some(value);
        self
    }

    pub fn dashboard_path(mut self, value: impl Into<String>) -> Self {
        self.dashboard_path = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn discover_path(mut self, value: impl Into<String>) -> Self {
        self.discover_path = Some(value.into());
        self
    }

    pub fn domain_id(mut self, value: impl Into<String>) -> Self {
        self.domain_id = Some(value.into());
        self
    }

    pub fn domains(mut self, value: Vec<AppDomain>) -> Self {
        self.domains = Some(value);
        self
    }

    pub fn experience_path(mut self, value: impl Into<String>) -> Self {
        self.experience_path = Some(value.into());
        self
    }

    pub fn hosted_url(mut self, value: impl Into<String>) -> Self {
        self.hosted_url = Some(value.into());
        self
    }

    pub fn icon(mut self, value: AppIcon) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_whop_app(mut self, value: bool) -> Self {
        self.is_whop_app = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn openapi_path(mut self, value: impl Into<String>) -> Self {
        self.openapi_path = Some(value.into());
        self
    }

    pub fn origin(mut self, value: impl Into<String>) -> Self {
        self.origin = Some(value.into());
        self
    }

    pub fn product_category(mut self, value: impl Into<String>) -> Self {
        self.product_category = Some(value.into());
        self
    }

    pub fn published_reviews_count(mut self, value: i64) -> Self {
        self.published_reviews_count = Some(value);
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn skills_path(mut self, value: impl Into<String>) -> Self {
        self.skills_path = Some(value.into());
        self
    }

    pub fn status(mut self, value: AppListItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account`](AppListItemBuilder::account)
    /// - [`app_type`](AppListItemBuilder::app_type)
    /// - [`average_review_rating`](AppListItemBuilder::average_review_rating)
    /// - [`businesses_created_count`](AppListItemBuilder::businesses_created_count)
    /// - [`businesses_created_logo_urls`](AppListItemBuilder::businesses_created_logo_urls)
    /// - [`businesses_with_recent_sales_count`](AppListItemBuilder::businesses_with_recent_sales_count)
    /// - [`creator`](AppListItemBuilder::creator)
    /// - [`domain_id`](AppListItemBuilder::domain_id)
    /// - [`domains`](AppListItemBuilder::domains)
    /// - [`icon`](AppListItemBuilder::icon)
    /// - [`id`](AppListItemBuilder::id)
    /// - [`is_whop_app`](AppListItemBuilder::is_whop_app)
    /// - [`name`](AppListItemBuilder::name)
    /// - [`published_reviews_count`](AppListItemBuilder::published_reviews_count)
    /// - [`status`](AppListItemBuilder::status)
    /// - [`verified`](AppListItemBuilder::verified)
    pub fn build(self) -> Result<AppListItem, BuildError> {
        Ok(AppListItem {
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
            app_type: self
                .app_type
                .ok_or_else(|| BuildError::missing_field("app_type"))?,
            average_review_rating: self
                .average_review_rating
                .ok_or_else(|| BuildError::missing_field("average_review_rating"))?,
            banner_image: self.banner_image,
            base_url: self.base_url,
            businesses_created_count: self
                .businesses_created_count
                .ok_or_else(|| BuildError::missing_field("businesses_created_count"))?,
            businesses_created_logo_urls: self
                .businesses_created_logo_urls
                .ok_or_else(|| BuildError::missing_field("businesses_created_logo_urls"))?,
            businesses_with_recent_sales_count: self
                .businesses_with_recent_sales_count
                .ok_or_else(|| BuildError::missing_field("businesses_with_recent_sales_count"))?,
            category: self.category,
            creator: self
                .creator
                .ok_or_else(|| BuildError::missing_field("creator"))?,
            dashboard_path: self.dashboard_path,
            description: self.description,
            discover_path: self.discover_path,
            domain_id: self
                .domain_id
                .ok_or_else(|| BuildError::missing_field("domain_id"))?,
            domains: self
                .domains
                .ok_or_else(|| BuildError::missing_field("domains"))?,
            experience_path: self.experience_path,
            hosted_url: self.hosted_url,
            icon: self.icon.ok_or_else(|| BuildError::missing_field("icon"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_whop_app: self
                .is_whop_app
                .ok_or_else(|| BuildError::missing_field("is_whop_app"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            openapi_path: self.openapi_path,
            origin: self.origin,
            product_category: self.product_category,
            published_reviews_count: self
                .published_reviews_count
                .ok_or_else(|| BuildError::missing_field("published_reviews_count"))?,
            route: self.route,
            skills_path: self.skills_path,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            verified: self
                .verified
                .ok_or_else(|| BuildError::missing_field("verified"))?,
        })
    }
}
