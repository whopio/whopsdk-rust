pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationAccount {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub id: String,
    /// Account logo image URL. `null` when the account has not uploaded one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// URL slug of the account's store page on whop.com.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    /// Account display name.
    #[serde(default)]
    pub title: String,
}

impl NotificationAccount {
    pub fn builder() -> NotificationAccountBuilder {
        <NotificationAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationAccountBuilder {
    id: Option<String>,
    logo_url: Option<String>,
    route: Option<String>,
    title: Option<String>,
}

impl NotificationAccountBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
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

    /// Consumes the builder and constructs a [`NotificationAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](NotificationAccountBuilder::id)
    /// - [`title`](NotificationAccountBuilder::title)
    pub fn build(self) -> Result<NotificationAccount, BuildError> {
        Ok(NotificationAccount {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo_url: self.logo_url,
            route: self.route,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
