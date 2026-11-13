pub use crate::prelude::*;

/// Referred account.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResendInviteBusinessesResponseAccount {
    pub capabilities: AccountCapabilities,
    /// Referred account ID.
    #[serde(default)]
    pub id: String,
    /// Referred account logo URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Optional actions that unlock capabilities or grow the referred account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_actions: Option<Vec<AccountRecommendedAction>>,
    /// Actions the referred account owner must take to unblock capabilities.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_actions: Option<Vec<AccountRequiredAction>>,
    /// Referred account route.
    #[serde(default)]
    pub route: String,
    /// Referred account display name.
    #[serde(default)]
    pub title: String,
}

impl ResendInviteBusinessesResponseAccount {
    pub fn builder() -> ResendInviteBusinessesResponseAccountBuilder {
        <ResendInviteBusinessesResponseAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendInviteBusinessesResponseAccountBuilder {
    capabilities: Option<AccountCapabilities>,
    id: Option<String>,
    logo_url: Option<String>,
    recommended_actions: Option<Vec<AccountRecommendedAction>>,
    required_actions: Option<Vec<AccountRequiredAction>>,
    route: Option<String>,
    title: Option<String>,
}

impl ResendInviteBusinessesResponseAccountBuilder {
    pub fn capabilities(mut self, value: AccountCapabilities) -> Self {
        self.capabilities = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn recommended_actions(mut self, value: Vec<AccountRecommendedAction>) -> Self {
        self.recommended_actions = Some(value);
        self
    }

    pub fn required_actions(mut self, value: Vec<AccountRequiredAction>) -> Self {
        self.required_actions = Some(value);
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResendInviteBusinessesResponseAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capabilities`](ResendInviteBusinessesResponseAccountBuilder::capabilities)
    /// - [`id`](ResendInviteBusinessesResponseAccountBuilder::id)
    /// - [`route`](ResendInviteBusinessesResponseAccountBuilder::route)
    /// - [`title`](ResendInviteBusinessesResponseAccountBuilder::title)
    pub fn build(self) -> Result<ResendInviteBusinessesResponseAccount, BuildError> {
        Ok(ResendInviteBusinessesResponseAccount {
            capabilities: self
                .capabilities
                .ok_or_else(|| BuildError::missing_field("capabilities"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo_url: self.logo_url,
            recommended_actions: self.recommended_actions,
            required_actions: self.required_actions,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
