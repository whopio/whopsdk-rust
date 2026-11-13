pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChannelModeration {
    /// Prevent members from posting media.
    #[serde(default)]
    pub ban_media: bool,
    /// Prevent members from posting links.
    #[serde(default)]
    pub ban_urls: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_words: Option<Vec<String>>,
    /// Seconds a member must wait between messages. `0` or `null` turns the cooldown off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_posts_cooldown_seconds: Option<i64>,
    /// Who can post messages.
    pub who_can_post: ChannelModerationWhoCanPost,
    /// Who can react to messages.
    pub who_can_react: ChannelModerationWhoCanReact,
}

impl ChannelModeration {
    pub fn builder() -> ChannelModerationBuilder {
        <ChannelModerationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChannelModerationBuilder {
    ban_media: Option<bool>,
    ban_urls: Option<bool>,
    banned_words: Option<Vec<String>>,
    user_posts_cooldown_seconds: Option<i64>,
    who_can_post: Option<ChannelModerationWhoCanPost>,
    who_can_react: Option<ChannelModerationWhoCanReact>,
}

impl ChannelModerationBuilder {
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

    pub fn user_posts_cooldown_seconds(mut self, value: i64) -> Self {
        self.user_posts_cooldown_seconds = Some(value);
        self
    }

    pub fn who_can_post(mut self, value: ChannelModerationWhoCanPost) -> Self {
        self.who_can_post = Some(value);
        self
    }

    pub fn who_can_react(mut self, value: ChannelModerationWhoCanReact) -> Self {
        self.who_can_react = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChannelModeration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ban_media`](ChannelModerationBuilder::ban_media)
    /// - [`ban_urls`](ChannelModerationBuilder::ban_urls)
    /// - [`who_can_post`](ChannelModerationBuilder::who_can_post)
    /// - [`who_can_react`](ChannelModerationBuilder::who_can_react)
    pub fn build(self) -> Result<ChannelModeration, BuildError> {
        Ok(ChannelModeration {
            ban_media: self
                .ban_media
                .ok_or_else(|| BuildError::missing_field("ban_media"))?,
            ban_urls: self
                .ban_urls
                .ok_or_else(|| BuildError::missing_field("ban_urls"))?,
            banned_words: self.banned_words,
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
