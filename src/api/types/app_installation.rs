pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AppInstallation {
    /// Account the app is installed on.
    #[serde(default)]
    pub account: AccountSummary,
    /// App that is installed.
    #[serde(default)]
    pub app: AppSummary,
    /// When the app was first installed on the account, as an ISO 8601 timestamp. Installing the app again later keeps the original time.
    #[serde(default)]
    pub created_at: String,
    /// Your app's own identifier for this installation, such as the ID of the linked account in another system, or `null` when none is set. At most 255 characters and unique across your app's installations, so you can find an installation with `GET /app_installations?external_id=`; setting one another installation already has returns a `409`. Only the app and its developer see it; for the account the app is installed on it is always `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// App installation ID, prefixed `aag_`.
    #[serde(default)]
    pub id: String,
    /// String keys and values your app stores about this installation, such as a sync cursor or the ID of the linked account in another system. Empty until your app sets a key, and kept if the account removes the app and installs it again. At most 50 keys; keys are at most 100 characters and values at most 500. Only the app can change it. Only the app and its developer see it; for the account the app is installed on it is always `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, Option<String>>>,
    /// Whether the app now asks for permissions the account hasn't approved yet.
    #[serde(default)]
    pub requesting_reauthorization: bool,
    /// When the installation last changed, such as a `metadata` update or the account changing the app's permissions, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl AppInstallation {
    pub fn builder() -> AppInstallationBuilder {
        <AppInstallationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppInstallationBuilder {
    account: Option<AccountSummary>,
    app: Option<AppSummary>,
    created_at: Option<String>,
    external_id: Option<String>,
    id: Option<String>,
    metadata: Option<HashMap<String, Option<String>>>,
    requesting_reauthorization: Option<bool>,
    updated_at: Option<String>,
}

impl AppInstallationBuilder {
    pub fn account(mut self, value: AccountSummary) -> Self {
        self.account = Some(value);
        self
    }

    pub fn app(mut self, value: AppSummary) -> Self {
        self.app = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn requesting_reauthorization(mut self, value: bool) -> Self {
        self.requesting_reauthorization = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AppInstallation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account`](AppInstallationBuilder::account)
    /// - [`app`](AppInstallationBuilder::app)
    /// - [`created_at`](AppInstallationBuilder::created_at)
    /// - [`id`](AppInstallationBuilder::id)
    /// - [`requesting_reauthorization`](AppInstallationBuilder::requesting_reauthorization)
    /// - [`updated_at`](AppInstallationBuilder::updated_at)
    pub fn build(self) -> Result<AppInstallation, BuildError> {
        Ok(AppInstallation {
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
            app: self.app.ok_or_else(|| BuildError::missing_field("app"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            external_id: self.external_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            metadata: self.metadata,
            requesting_reauthorization: self
                .requesting_reauthorization
                .ok_or_else(|| BuildError::missing_field("requesting_reauthorization"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
