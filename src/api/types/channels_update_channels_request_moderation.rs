pub use crate::prelude::*;

/// Moderation settings, accepted only for community channels. Requires chat moderation permission. Omitted settings are preserved.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateChannelsRequestModeration {
    /// Prevent members from posting media.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_media: Option<bool>,
    /// Prevent members from posting links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_urls: Option<bool>,
    /// Words blocked in the channel. An empty list clears the blocklist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_words: Option<Vec<String>>,
    /// Seconds a member must wait between messages. `0` or `null` turns the cooldown off.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_posts_cooldown_seconds: Option<i64>,
    /// Who can post messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_can_post: Option<UpdateChannelsRequestModerationWhoCanPost>,
    /// Who can react to messages.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_can_react: Option<UpdateChannelsRequestModerationWhoCanReact>,
}

impl UpdateChannelsRequestModeration {
    pub fn builder() -> UpdateChannelsRequestModerationBuilder {
        <UpdateChannelsRequestModerationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateChannelsRequestModerationBuilder {
    ban_media: Option<bool>,
    ban_urls: Option<bool>,
    banned_words: Option<Vec<String>>,
    user_posts_cooldown_seconds: Option<i64>,
    who_can_post: Option<UpdateChannelsRequestModerationWhoCanPost>,
    who_can_react: Option<UpdateChannelsRequestModerationWhoCanReact>,
}

impl UpdateChannelsRequestModerationBuilder {
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

    pub fn who_can_post(mut self, value: UpdateChannelsRequestModerationWhoCanPost) -> Self {
        self.who_can_post = Some(value);
        self
    }

    pub fn who_can_react(mut self, value: UpdateChannelsRequestModerationWhoCanReact) -> Self {
        self.who_can_react = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateChannelsRequestModeration`].
    pub fn build(self) -> Result<UpdateChannelsRequestModeration, BuildError> {
        Ok(UpdateChannelsRequestModeration {
            ban_media: self.ban_media,
            ban_urls: self.ban_urls,
            banned_words: self.banned_words,
            user_posts_cooldown_seconds: self.user_posts_cooldown_seconds,
            who_can_post: self.who_can_post,
            who_can_react: self.who_can_react,
        })
    }
}
