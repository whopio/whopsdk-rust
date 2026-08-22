pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateNotificationsRequest {
    /// Account whose team members receive the notification (`biz_` tag). Exactly one of `experience_id` or `account_id` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Main body text of the notification.
    #[serde(default)]
    pub content: String,
    /// Experience whose users receive the notification (`exp_` tag). Exactly one of `experience_id` or `account_id` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience_id: Option<String>,
    /// User whose profile picture is used as the notification icon. Defaults to the experience or account avatar.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_user_id: Option<String>,
    /// Path segment appended to the generated deep link that opens your app, for example `/settings/billing`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_path: Option<String>,
    /// Optional secondary line displayed below the title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// Headline text of the notification.
    #[serde(default)]
    pub title: String,
    /// Optional `user_` tags narrowing the audience. When provided, only these users are notified (as a mention), provided they are in the targeted experience or account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
}

impl CreateNotificationsRequest {
    pub fn builder() -> CreateNotificationsRequestBuilder {
        <CreateNotificationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateNotificationsRequestBuilder {
    account_id: Option<String>,
    content: Option<String>,
    experience_id: Option<String>,
    icon_user_id: Option<String>,
    rest_path: Option<String>,
    subtitle: Option<String>,
    title: Option<String>,
    user_ids: Option<Vec<String>>,
}

impl CreateNotificationsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn icon_user_id(mut self, value: impl Into<String>) -> Self {
        self.icon_user_id = Some(value.into());
        self
    }

    pub fn rest_path(mut self, value: impl Into<String>) -> Self {
        self.rest_path = Some(value.into());
        self
    }

    pub fn subtitle(mut self, value: impl Into<String>) -> Self {
        self.subtitle = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn user_ids(mut self, value: Vec<String>) -> Self {
        self.user_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateNotificationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](CreateNotificationsRequestBuilder::content)
    /// - [`title`](CreateNotificationsRequestBuilder::title)
    pub fn build(self) -> Result<CreateNotificationsRequest, BuildError> {
        Ok(CreateNotificationsRequest {
            account_id: self.account_id,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            experience_id: self.experience_id,
            icon_user_id: self.icon_user_id,
            rest_path: self.rest_path,
            subtitle: self.subtitle,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            user_ids: self.user_ids,
        })
    }
}
