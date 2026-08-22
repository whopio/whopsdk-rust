pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Notification {
    /// Account the notification belongs to. `null` when the notification is not associated with an account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<NotificationAccount>,
    /// Image displayed alongside the notification — the sender's avatar, or the default notification image.
    #[serde(default)]
    pub attachment_url: String,
    /// The body text of the notification displayed to the user.
    #[serde(default)]
    pub content: String,
    /// When the notification was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Experience the notification is related to. `null` when not tied to a specific experience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experience: Option<NotificationExperience>,
    /// Notification ID. Feed rows carry a composite id that doubles as the list cursor.
    #[serde(default)]
    pub id: String,
    /// The same destination on the app's own domain, which the Whop web and mobile clients embed instead of navigating to it. Only relevant if you render Whop apps yourself. `null` when the notification carries its own `link`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iframe_link: Option<String>,
    /// The whop.com page the notification opens, as a normal top-level navigation. This is the link to use unless you host Whop apps yourself. `null` when the notification has no click-through destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<String>,
    /// Whether the authenticated user was directly mentioned in this notification.
    #[serde(default)]
    pub mentions_me: bool,
    /// Additional path information appended to the notification's deep link, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_path: Option<String>,
    /// User who triggered the notification. `null` when it was system-generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<NotificationSender>,
    /// The title line of the notification displayed to the user.
    #[serde(default)]
    pub subject: String,
    /// Topic category the notification belongs to, used for grouping and preference management. `null` when uncategorized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<NotificationTopicSummary>,
}

impl Notification {
    pub fn builder() -> NotificationBuilder {
        <NotificationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationBuilder {
    account: Option<NotificationAccount>,
    attachment_url: Option<String>,
    content: Option<String>,
    created_at: Option<String>,
    experience: Option<NotificationExperience>,
    id: Option<String>,
    iframe_link: Option<String>,
    link: Option<String>,
    mentions_me: Option<bool>,
    rest_path: Option<String>,
    sender: Option<NotificationSender>,
    subject: Option<String>,
    topic: Option<NotificationTopicSummary>,
}

impl NotificationBuilder {
    pub fn account(mut self, value: NotificationAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn attachment_url(mut self, value: impl Into<String>) -> Self {
        self.attachment_url = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn experience(mut self, value: NotificationExperience) -> Self {
        self.experience = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn iframe_link(mut self, value: impl Into<String>) -> Self {
        self.iframe_link = Some(value.into());
        self
    }

    pub fn link(mut self, value: impl Into<String>) -> Self {
        self.link = Some(value.into());
        self
    }

    pub fn mentions_me(mut self, value: bool) -> Self {
        self.mentions_me = Some(value);
        self
    }

    pub fn rest_path(mut self, value: impl Into<String>) -> Self {
        self.rest_path = Some(value.into());
        self
    }

    pub fn sender(mut self, value: NotificationSender) -> Self {
        self.sender = Some(value);
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn topic(mut self, value: NotificationTopicSummary) -> Self {
        self.topic = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Notification`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachment_url`](NotificationBuilder::attachment_url)
    /// - [`content`](NotificationBuilder::content)
    /// - [`created_at`](NotificationBuilder::created_at)
    /// - [`id`](NotificationBuilder::id)
    /// - [`mentions_me`](NotificationBuilder::mentions_me)
    /// - [`subject`](NotificationBuilder::subject)
    pub fn build(self) -> Result<Notification, BuildError> {
        Ok(Notification {
            account: self.account,
            attachment_url: self
                .attachment_url
                .ok_or_else(|| BuildError::missing_field("attachment_url"))?,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            experience: self.experience,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            iframe_link: self.iframe_link,
            link: self.link,
            mentions_me: self
                .mentions_me
                .ok_or_else(|| BuildError::missing_field("mentions_me"))?,
            rest_path: self.rest_path,
            sender: self.sender,
            subject: self
                .subject
                .ok_or_else(|| BuildError::missing_field("subject"))?,
            topic: self.topic,
        })
    }
}
