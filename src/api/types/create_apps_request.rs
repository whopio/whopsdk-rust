pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAppsRequest {
    /// The account to create the app for (`biz_` tag). Defaults to the account behind the presented credential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The type of app to create. Defaults to `b2c_app`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<CreateAppsRequestAppType>,
    /// The base production URL where the app is hosted, such as `https://myapp.example.com`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// The icon image for the app in PNG, JPEG, or GIF format, referencing an uploaded file: `{ id }` for an existing attachment or `{ direct_upload_id }` for a new direct upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<CreateAppsRequestIcon>,
    /// The display name for the app, shown to users on the app store and product pages.
    #[serde(default)]
    pub name: String,
    /// The whitelisted OAuth callback URLs that users are redirected to after authorizing the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
    /// The subdomain route where the app's hosted web builds are served, such as `myapp` for myapp.whop.app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
}

impl CreateAppsRequest {
    pub fn builder() -> CreateAppsRequestBuilder {
        <CreateAppsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAppsRequestBuilder {
    account_id: Option<String>,
    app_type: Option<CreateAppsRequestAppType>,
    base_url: Option<String>,
    icon: Option<CreateAppsRequestIcon>,
    name: Option<String>,
    redirect_uris: Option<Vec<String>>,
    route: Option<String>,
}

impl CreateAppsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn app_type(mut self, value: CreateAppsRequestAppType) -> Self {
        self.app_type = Some(value);
        self
    }

    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = Some(value.into());
        self
    }

    pub fn icon(mut self, value: CreateAppsRequestIcon) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn redirect_uris(mut self, value: Vec<String>) -> Self {
        self.redirect_uris = Some(value);
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAppsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateAppsRequestBuilder::name)
    pub fn build(self) -> Result<CreateAppsRequest, BuildError> {
        Ok(CreateAppsRequest {
            account_id: self.account_id,
            app_type: self.app_type,
            base_url: self.base_url,
            icon: self.icon,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            redirect_uris: self.redirect_uris,
            route: self.route,
        })
    }
}
