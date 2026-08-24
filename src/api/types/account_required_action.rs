pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountRequiredAction {
    /// What the holder must do; new values may be added, so handle unknown actions gracefully
    pub action: AccountRequiredActionAction,
    #[serde(default)]
    pub blocked_capabilities: Vec<String>,
    /// The URL the call-to-action links to, or null when there is no button
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cta: Option<String>,
    /// Button label, or empty when there is no button
    #[serde(default)]
    pub cta_label: String,
    /// Supporting copy, or empty
    #[serde(default)]
    pub description: String,
    /// The URL of the action's illustration icon, or null if it has none
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_url: Option<String>,
    /// required (act now) or pending (under review)
    pub status: AccountRequiredActionStatus,
    /// Headline for the action
    #[serde(default)]
    pub title: String,
}

impl AccountRequiredAction {
    pub fn builder() -> AccountRequiredActionBuilder {
        <AccountRequiredActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountRequiredActionBuilder {
    action: Option<AccountRequiredActionAction>,
    blocked_capabilities: Option<Vec<String>>,
    cta: Option<String>,
    cta_label: Option<String>,
    description: Option<String>,
    icon_url: Option<String>,
    status: Option<AccountRequiredActionStatus>,
    title: Option<String>,
}

impl AccountRequiredActionBuilder {
    pub fn action(mut self, value: AccountRequiredActionAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn blocked_capabilities(mut self, value: Vec<String>) -> Self {
        self.blocked_capabilities = Some(value);
        self
    }

    pub fn cta(mut self, value: impl Into<String>) -> Self {
        self.cta = Some(value.into());
        self
    }

    pub fn cta_label(mut self, value: impl Into<String>) -> Self {
        self.cta_label = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn icon_url(mut self, value: impl Into<String>) -> Self {
        self.icon_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: AccountRequiredActionStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountRequiredAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](AccountRequiredActionBuilder::action)
    /// - [`blocked_capabilities`](AccountRequiredActionBuilder::blocked_capabilities)
    /// - [`cta_label`](AccountRequiredActionBuilder::cta_label)
    /// - [`description`](AccountRequiredActionBuilder::description)
    /// - [`status`](AccountRequiredActionBuilder::status)
    /// - [`title`](AccountRequiredActionBuilder::title)
    pub fn build(self) -> Result<AccountRequiredAction, BuildError> {
        Ok(AccountRequiredAction {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            blocked_capabilities: self
                .blocked_capabilities
                .ok_or_else(|| BuildError::missing_field("blocked_capabilities"))?,
            cta: self.cta,
            cta_label: self
                .cta_label
                .ok_or_else(|| BuildError::missing_field("cta_label"))?,
            description: self
                .description
                .ok_or_else(|| BuildError::missing_field("description"))?,
            icon_url: self.icon_url,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
