pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct App {
    /// The account that owns the app.
    #[serde(default)]
    pub account: AccountParent,
    /// Legacy app API key used to authenticate requests on the app's behalf. `null` when no key exists or the caller lacks the `developer:manage_api_key` permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key: Option<AppApiKey>,
    /// Detailed description shown on the app store's in-depth app page, or `null` when none has been set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_description: Option<String>,
    /// The type of end-user the app is built for.
    pub app_type: AppAppType,
    /// Banner image from the app's product listing, or `null` when none is uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<AppBannerImage>,
    /// The production base URL where the app is hosted. `null` if no base URL is configured, if the caller lacks the `developer:basic:read` permission on the app's account, or on list responses, which never expose it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// Website businesses created from this app as a template.
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
    /// The app's default API key. `null` when the app has no default key or the caller lacks the `developer:manage_api_key` permission; `secret_key` is additionally `null` unless the caller could have created the key themselves.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_api_key: Option<AppDefaultApiKey>,
    /// What the app has left to publish, and how a publish in flight is going — `status` is only ever `unpublished`, `publishing`, or `failed` here. `null` means there is nothing to report: the app is fully published, there is no working copy to publish from, or the caller cannot deploy this app. Tell those apart from the app's own `production_web_build`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deployment: Option<AppDeployment>,
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
    /// Approval status of the app's product listing on the Whop app store, or `null` when the app has no associated product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_status: Option<AppMarketplaceStatus>,
    /// Display name shown on the app store and in experience navigation.
    #[serde(default)]
    pub name: String,
    /// How the app authenticates at the OAuth token endpoint.
    pub oauth_client_type: AppOauthClientType,
    /// URL path to the app's OpenAPI spec file, or `null` when not configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openapi_path: Option<String>,
    /// Full origin URL of the app's proxied domain, for example https://ab1c2d3e4f.apps.whop.com.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    /// ID of the app's product listing on the Whop app store, or `null` when the app has no associated product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The approved build currently served on Android, or `null` when none is deployed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_android_build: Option<AppProductionBuild>,
    /// The approved build currently served on iOS, or `null` when none is deployed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_ios_build: Option<AppProductionBuild>,
    /// The approved build currently served on web, or `null` when none is deployed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_web_build: Option<AppProductionBuild>,
    #[serde(default)]
    pub redirect_uris: Vec<String>,
    #[serde(default)]
    pub requested_permissions: Vec<AppRequestedPermission>,
    #[serde(default)]
    pub required_scopes: Vec<AppRequiredScopesItem>,
    /// Claimed subdomain route where hosted web builds are served (`myapp` for myapp.whop.app), or `null` if no route is claimed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// The app's production secrets as an object of string values, injected into the hosted server runtime. `null` when the caller lacks the `developer:update_app` permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<HashMap<String, serde_json::Value>>,
    /// URL path to the app's skills directory, or `null` when not configured.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills_path: Option<String>,
    /// Visibility on the Whop app store: `live` is publicly discoverable, `unlisted` is accessible only via direct link, `hidden` is not visible anywhere.
    pub status: AppStatus,
    /// Whether the app has been verified by Whop and is eligible for the featured apps section.
    #[serde(default)]
    pub verified: bool,
}

impl App {
    pub fn builder() -> AppBuilder {
        <AppBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppBuilder {
    account: Option<AccountParent>,
    api_key: Option<AppApiKey>,
    app_store_description: Option<String>,
    app_type: Option<AppAppType>,
    banner_image: Option<AppBannerImage>,
    base_url: Option<String>,
    businesses_created_count: Option<i64>,
    businesses_created_logo_urls: Option<Vec<String>>,
    creator: Option<AppCreator>,
    dashboard_path: Option<String>,
    default_api_key: Option<AppDefaultApiKey>,
    deployment: Option<AppDeployment>,
    description: Option<String>,
    discover_path: Option<String>,
    domain_id: Option<String>,
    experience_path: Option<String>,
    hosted_url: Option<String>,
    icon: Option<AppIcon>,
    id: Option<String>,
    marketplace_status: Option<AppMarketplaceStatus>,
    name: Option<String>,
    oauth_client_type: Option<AppOauthClientType>,
    openapi_path: Option<String>,
    origin: Option<String>,
    product_id: Option<String>,
    production_android_build: Option<AppProductionBuild>,
    production_ios_build: Option<AppProductionBuild>,
    production_web_build: Option<AppProductionBuild>,
    redirect_uris: Option<Vec<String>>,
    requested_permissions: Option<Vec<AppRequestedPermission>>,
    required_scopes: Option<Vec<AppRequiredScopesItem>>,
    route: Option<String>,
    secrets: Option<HashMap<String, serde_json::Value>>,
    skills_path: Option<String>,
    status: Option<AppStatus>,
    verified: Option<bool>,
}

impl AppBuilder {
    pub fn account(mut self, value: AccountParent) -> Self {
        self.account = Some(value);
        self
    }

    pub fn api_key(mut self, value: AppApiKey) -> Self {
        self.api_key = Some(value);
        self
    }

    pub fn app_store_description(mut self, value: impl Into<String>) -> Self {
        self.app_store_description = Some(value.into());
        self
    }

    pub fn app_type(mut self, value: AppAppType) -> Self {
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

    pub fn default_api_key(mut self, value: AppDefaultApiKey) -> Self {
        self.default_api_key = Some(value);
        self
    }

    pub fn deployment(mut self, value: AppDeployment) -> Self {
        self.deployment = Some(value);
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

    pub fn marketplace_status(mut self, value: AppMarketplaceStatus) -> Self {
        self.marketplace_status = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn oauth_client_type(mut self, value: AppOauthClientType) -> Self {
        self.oauth_client_type = Some(value);
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

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn production_android_build(mut self, value: AppProductionBuild) -> Self {
        self.production_android_build = Some(value);
        self
    }

    pub fn production_ios_build(mut self, value: AppProductionBuild) -> Self {
        self.production_ios_build = Some(value);
        self
    }

    pub fn production_web_build(mut self, value: AppProductionBuild) -> Self {
        self.production_web_build = Some(value);
        self
    }

    pub fn redirect_uris(mut self, value: Vec<String>) -> Self {
        self.redirect_uris = Some(value);
        self
    }

    pub fn requested_permissions(mut self, value: Vec<AppRequestedPermission>) -> Self {
        self.requested_permissions = Some(value);
        self
    }

    pub fn required_scopes(mut self, value: Vec<AppRequiredScopesItem>) -> Self {
        self.required_scopes = Some(value);
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn secrets(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.secrets = Some(value);
        self
    }

    pub fn skills_path(mut self, value: impl Into<String>) -> Self {
        self.skills_path = Some(value.into());
        self
    }

    pub fn status(mut self, value: AppStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn verified(mut self, value: bool) -> Self {
        self.verified = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`App`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account`](AppBuilder::account)
    /// - [`app_type`](AppBuilder::app_type)
    /// - [`businesses_created_count`](AppBuilder::businesses_created_count)
    /// - [`businesses_created_logo_urls`](AppBuilder::businesses_created_logo_urls)
    /// - [`creator`](AppBuilder::creator)
    /// - [`domain_id`](AppBuilder::domain_id)
    /// - [`icon`](AppBuilder::icon)
    /// - [`id`](AppBuilder::id)
    /// - [`name`](AppBuilder::name)
    /// - [`oauth_client_type`](AppBuilder::oauth_client_type)
    /// - [`redirect_uris`](AppBuilder::redirect_uris)
    /// - [`requested_permissions`](AppBuilder::requested_permissions)
    /// - [`required_scopes`](AppBuilder::required_scopes)
    /// - [`status`](AppBuilder::status)
    /// - [`verified`](AppBuilder::verified)
    pub fn build(self) -> Result<App, BuildError> {
        Ok(App {
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
            api_key: self.api_key,
            app_store_description: self.app_store_description,
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
            default_api_key: self.default_api_key,
            deployment: self.deployment,
            description: self.description,
            discover_path: self.discover_path,
            domain_id: self
                .domain_id
                .ok_or_else(|| BuildError::missing_field("domain_id"))?,
            experience_path: self.experience_path,
            hosted_url: self.hosted_url,
            icon: self.icon.ok_or_else(|| BuildError::missing_field("icon"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            marketplace_status: self.marketplace_status,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            oauth_client_type: self
                .oauth_client_type
                .ok_or_else(|| BuildError::missing_field("oauth_client_type"))?,
            openapi_path: self.openapi_path,
            origin: self.origin,
            product_id: self.product_id,
            production_android_build: self.production_android_build,
            production_ios_build: self.production_ios_build,
            production_web_build: self.production_web_build,
            redirect_uris: self
                .redirect_uris
                .ok_or_else(|| BuildError::missing_field("redirect_uris"))?,
            requested_permissions: self
                .requested_permissions
                .ok_or_else(|| BuildError::missing_field("requested_permissions"))?,
            required_scopes: self
                .required_scopes
                .ok_or_else(|| BuildError::missing_field("required_scopes"))?,
            route: self.route,
            secrets: self.secrets,
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
