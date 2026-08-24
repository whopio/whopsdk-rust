pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AppListItem {
    /// The account that owns the app.
    #[serde(default)]
    pub account: AccountParent,
    /// The type of end-user the app is built for.
    pub app_type: AppListItemAppType,
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
    /// URL path for the member-facing hub view, or `null` when not configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_path: Option<String>,
    /// Full URL where the app's hosted web build is served, or `null` if no route is claimed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_url: Option<String>,
    /// The app's icon. Falls back to the default app icon when none is uploaded.
    #[serde(default)]
    pub icon: AppIcon,
    /// App ID, prefixed `app_`.
    #[serde(default)]
    pub id: String,
    /// Display name shown on the app store and in experience navigation.
    #[serde(default)]
    pub name: String,
    /// URL path to the app's OpenAPI spec file, or `null` when not configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openapi_path: Option<String>,
    /// Full origin URL of the app's proxied domain, for example https://ab1c2d3e4f.apps.whop.com.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// Claimed subdomain route where hosted web builds are served (`myapp` for myapp.whop.app), or `null` if no route is claimed.
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
    banner_image: Option<AppBannerImage>,
    base_url: Option<String>,
    businesses_created_count: Option<i64>,
    businesses_created_logo_urls: Option<Vec<String>>,
    creator: Option<AppCreator>,
    dashboard_path: Option<String>,
    description: Option<String>,
    discover_path: Option<String>,
    domain_id: Option<String>,
    experience_path: Option<String>,
    hosted_url: Option<String>,
    icon: Option<AppIcon>,
    id: Option<String>,
    name: Option<String>,
    openapi_path: Option<String>,
    origin: Option<String>,
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
    /// - [`businesses_created_count`](AppListItemBuilder::businesses_created_count)
    /// - [`businesses_created_logo_urls`](AppListItemBuilder::businesses_created_logo_urls)
    /// - [`creator`](AppListItemBuilder::creator)
    /// - [`domain_id`](AppListItemBuilder::domain_id)
    /// - [`icon`](AppListItemBuilder::icon)
    /// - [`id`](AppListItemBuilder::id)
    /// - [`name`](AppListItemBuilder::name)
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
            banner_image: self.banner_image,
            base_url: self.base_url,
            businesses_created_count: self
                .businesses_created_count
                .ok_or_else(|| BuildError::missing_field("businesses_created_count"))?,
            businesses_created_logo_urls: self
                .businesses_created_logo_urls
                .ok_or_else(|| BuildError::missing_field("businesses_created_logo_urls"))?,
            creator: self
                .creator
                .ok_or_else(|| BuildError::missing_field("creator"))?,
            dashboard_path: self.dashboard_path,
            description: self.description,
            discover_path: self.discover_path,
            domain_id: self
                .domain_id
                .ok_or_else(|| BuildError::missing_field("domain_id"))?,
            experience_path: self.experience_path,
            hosted_url: self.hosted_url,
            icon: self.icon.ok_or_else(|| BuildError::missing_field("icon"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            openapi_path: self.openapi_path,
            origin: self.origin,
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
