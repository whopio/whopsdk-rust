pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AppSummary {
    /// Account that develops the app.
    #[serde(default)]
    pub developer: AccountSummary,
    /// App icon. Falls back to the default app icon when none is uploaded.
    #[serde(default)]
    pub icon: AppIcon,
    /// App ID, prefixed `app_`.
    #[serde(default)]
    pub id: String,
    /// App display name.
    #[serde(default)]
    pub name: String,
}

impl AppSummary {
    pub fn builder() -> AppSummaryBuilder {
        <AppSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppSummaryBuilder {
    developer: Option<AccountSummary>,
    icon: Option<AppIcon>,
    id: Option<String>,
    name: Option<String>,
}

impl AppSummaryBuilder {
    pub fn developer(mut self, value: AccountSummary) -> Self {
        self.developer = Some(value);
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

    /// Consumes the builder and constructs a [`AppSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`developer`](AppSummaryBuilder::developer)
    /// - [`icon`](AppSummaryBuilder::icon)
    /// - [`id`](AppSummaryBuilder::id)
    /// - [`name`](AppSummaryBuilder::name)
    pub fn build(self) -> Result<AppSummary, BuildError> {
        Ok(AppSummary {
            developer: self
                .developer
                .ok_or_else(|| BuildError::missing_field("developer"))?,
            icon: self.icon.ok_or_else(|| BuildError::missing_field("icon"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
