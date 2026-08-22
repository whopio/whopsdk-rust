pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAppsRequest {
    /// The detailed description shown on the app store's in-depth app view page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_store_description: Option<String>,
    /// The type of end-user the app is built for. Cannot be changed on an app whose type is already `website`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<UpdateAppsRequestAppType>,
    /// The base production URL where the app is hosted. Set to `null` to take the app proxy offline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    /// The URL path for the account dashboard view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dashboard_path: Option<String>,
    /// A short description of the app shown in listings and search results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The URL path for the discover view.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover_path: Option<String>,
    /// The URL path for the member-facing hub view, such as `/experiences/[experienceId]`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_path: Option<String>,
    /// The icon image for the app in PNG, JPEG, or GIF format, referencing an uploaded file: `{ id }` for an existing attachment or `{ direct_upload_id }` for a new direct upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<UpdateAppsRequestIcon>,
    /// The display name for the app, shown to users on the app store and product pages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// How the app authenticates at the OAuth token endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oauth_client_type: Option<UpdateAppsRequestOauthClientType>,
    /// The URL path to the app's OpenAPI spec file (requires the ai_chat capability).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub openapi_path: Option<String>,
    /// The app build (`abld_` tag) to serve as the Android production build, or `null` to unassign it. Same rules as `production_web_build_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_android_build_id: Option<String>,
    /// The app build (`abld_` tag) to serve as the iOS production build, or `null` to unassign it. Same rules as `production_web_build_id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_ios_build_id: Option<String>,
    /// The app build (`abld_` tag) to serve as the web production build, or `null` to unassign it. The build must belong to this app, target web, and be in the draft or approved status; a draft build is queued for approval and takes over once approved. Requires the `developer:manage_builds` scope.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_web_build_id: Option<String>,
    /// The whitelisted OAuth callback URLs users are redirected to after authorizing the app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
    /// The OAuth scopes the app requests from users when they install it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_scopes: Option<Vec<String>>,
    /// The subdomain route where the app's hosted web builds are served.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Secrets to add or overwrite on the app, as an object of string values. Keys not included are left untouched; pass null or an empty string as the value to delete a secret. Encrypted at rest and injected into the app's hosted server runtime.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secrets: Option<HashMap<String, serde_json::Value>>,
    /// The URL path to the app's skills directory (requires the ai_chat capability).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skills_path: Option<String>,
    /// Controls whether the app is published on Whop discovery or accessible only through its direct link. Publishing requires a name, icon, and description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UpdateAppsRequestStatus>,
}

impl UpdateAppsRequest {
    pub fn builder() -> UpdateAppsRequestBuilder {
        <UpdateAppsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAppsRequestBuilder {
    app_store_description: Option<String>,
    app_type: Option<UpdateAppsRequestAppType>,
    base_url: Option<String>,
    dashboard_path: Option<String>,
    description: Option<String>,
    discover_path: Option<String>,
    experience_path: Option<String>,
    icon: Option<UpdateAppsRequestIcon>,
    name: Option<String>,
    oauth_client_type: Option<UpdateAppsRequestOauthClientType>,
    openapi_path: Option<String>,
    production_android_build_id: Option<String>,
    production_ios_build_id: Option<String>,
    production_web_build_id: Option<String>,
    redirect_uris: Option<Vec<String>>,
    required_scopes: Option<Vec<String>>,
    route: Option<String>,
    secrets: Option<HashMap<String, serde_json::Value>>,
    skills_path: Option<String>,
    status: Option<UpdateAppsRequestStatus>,
}

impl UpdateAppsRequestBuilder {
    pub fn app_store_description(mut self, value: impl Into<String>) -> Self {
        self.app_store_description = Some(value.into());
        self
    }

    pub fn app_type(mut self, value: UpdateAppsRequestAppType) -> Self {
        self.app_type = Some(value);
        self
    }

    pub fn base_url(mut self, value: impl Into<String>) -> Self {
        self.base_url = Some(value.into());
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

    pub fn experience_path(mut self, value: impl Into<String>) -> Self {
        self.experience_path = Some(value.into());
        self
    }

    pub fn icon(mut self, value: UpdateAppsRequestIcon) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn oauth_client_type(mut self, value: UpdateAppsRequestOauthClientType) -> Self {
        self.oauth_client_type = Some(value);
        self
    }

    pub fn openapi_path(mut self, value: impl Into<String>) -> Self {
        self.openapi_path = Some(value.into());
        self
    }

    pub fn production_android_build_id(mut self, value: impl Into<String>) -> Self {
        self.production_android_build_id = Some(value.into());
        self
    }

    pub fn production_ios_build_id(mut self, value: impl Into<String>) -> Self {
        self.production_ios_build_id = Some(value.into());
        self
    }

    pub fn production_web_build_id(mut self, value: impl Into<String>) -> Self {
        self.production_web_build_id = Some(value.into());
        self
    }

    pub fn redirect_uris(mut self, value: Vec<String>) -> Self {
        self.redirect_uris = Some(value);
        self
    }

    pub fn required_scopes(mut self, value: Vec<String>) -> Self {
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

    pub fn status(mut self, value: UpdateAppsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAppsRequest`].
    pub fn build(self) -> Result<UpdateAppsRequest, BuildError> {
        Ok(UpdateAppsRequest {
            app_store_description: self.app_store_description,
            app_type: self.app_type,
            base_url: self.base_url,
            dashboard_path: self.dashboard_path,
            description: self.description,
            discover_path: self.discover_path,
            experience_path: self.experience_path,
            icon: self.icon,
            name: self.name,
            oauth_client_type: self.oauth_client_type,
            openapi_path: self.openapi_path,
            production_android_build_id: self.production_android_build_id,
            production_ios_build_id: self.production_ios_build_id,
            production_web_build_id: self.production_web_build_id,
            redirect_uris: self.redirect_uris,
            required_scopes: self.required_scopes,
            route: self.route,
            secrets: self.secrets,
            skills_path: self.skills_path,
            status: self.status,
        })
    }
}
