pub use crate::prelude::*;

/// A real-time chat feed attached to an experience, with configurable moderation and posting permissions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChatChannelListItem {
    /// Whether media uploads such as images and videos are blocked in this chat.
    #[serde(default)]
    pub ban_media: bool,
    /// Whether URL links are blocked from being posted in this chat.
    #[serde(default)]
    pub ban_urls: bool,
    /// A list of words that are automatically filtered from messages in this chat.
    #[serde(default)]
    pub banned_words: Vec<String>,
    /// The experience this chat feed is attached to.
    #[serde(default)]
    pub experience: ChatChannelListItemExperience,
    /// The unique identifier for the entity
    #[serde(default)]
    pub id: String,
    /// The minimum number of seconds a user must wait between consecutive messages. Null if no cooldown is enforced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_posts_cooldown_seconds: Option<i64>,
    /// The permission level controlling which users can send messages in this chat.
    pub who_can_post: WhoCanPostTypes,
    /// The permission level controlling which users can add reactions in this chat.
    pub who_can_react: WhoCanReactTypes,
}

impl ChatChannelListItem {
    pub fn builder() -> ChatChannelListItemBuilder {
        <ChatChannelListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatChannelListItemBuilder {
    ban_media: Option<bool>,
    ban_urls: Option<bool>,
    banned_words: Option<Vec<String>>,
    experience: Option<ChatChannelListItemExperience>,
    id: Option<String>,
    user_posts_cooldown_seconds: Option<i64>,
    who_can_post: Option<WhoCanPostTypes>,
    who_can_react: Option<WhoCanReactTypes>,
}

impl ChatChannelListItemBuilder {
    pub fn ban_media(mut self, value: bool) -> Self {
        self.ban_media = Some(value);
        self
    }

    pub fn ban_urls(mut self, value: bool) -> Self {
        self.ban_urls = Some(value);
        self
    }

    pub fn banned_words(mut self, value: Vec<String>) -> Self {
        self.banned_words = Some(value);
        self
    }

    pub fn experience(mut self, value: ChatChannelListItemExperience) -> Self {
        self.experience = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn user_posts_cooldown_seconds(mut self, value: i64) -> Self {
        self.user_posts_cooldown_seconds = Some(value);
        self
    }

    pub fn who_can_post(mut self, value: WhoCanPostTypes) -> Self {
        self.who_can_post = Some(value);
        self
    }

    pub fn who_can_react(mut self, value: WhoCanReactTypes) -> Self {
        self.who_can_react = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatChannelListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ban_media`](ChatChannelListItemBuilder::ban_media)
    /// - [`ban_urls`](ChatChannelListItemBuilder::ban_urls)
    /// - [`banned_words`](ChatChannelListItemBuilder::banned_words)
    /// - [`experience`](ChatChannelListItemBuilder::experience)
    /// - [`id`](ChatChannelListItemBuilder::id)
    /// - [`who_can_post`](ChatChannelListItemBuilder::who_can_post)
    /// - [`who_can_react`](ChatChannelListItemBuilder::who_can_react)
    pub fn build(self) -> Result<ChatChannelListItem, BuildError> {
        Ok(ChatChannelListItem {
            ban_media: self
                .ban_media
                .ok_or_else(|| BuildError::missing_field("ban_media"))?,
            ban_urls: self
                .ban_urls
                .ok_or_else(|| BuildError::missing_field("ban_urls"))?,
            banned_words: self
                .banned_words
                .ok_or_else(|| BuildError::missing_field("banned_words"))?,
            experience: self
                .experience
                .ok_or_else(|| BuildError::missing_field("experience"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            user_posts_cooldown_seconds: self.user_posts_cooldown_seconds,
            who_can_post: self
                .who_can_post
                .ok_or_else(|| BuildError::missing_field("who_can_post"))?,
            who_can_react: self
                .who_can_react
                .ok_or_else(|| BuildError::missing_field("who_can_react"))?,
        })
    }
}
