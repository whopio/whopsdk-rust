pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateChatChannelsRequest {
    /// Whether media uploads such as images and videos are banned in this chat channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_media: Option<bool>,
    /// Whether URLs and links are banned from being posted in this chat channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ban_urls: Option<bool>,
    /// A list of words that are automatically blocked from messages in this chat channel. For example, ['spam', 'scam'].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_words: Option<Vec<String>>,
    /// The minimum number of seconds a user must wait between sending messages in this chat channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_posts_cooldown_seconds: Option<i64>,
    /// Controls which roles are allowed to send messages in this chat channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_can_post: Option<WhoCanPostTypes>,
    /// Controls which roles are allowed to add reactions to messages in this chat channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_can_react: Option<WhoCanReactTypes>,
}

impl UpdateChatChannelsRequest {
    pub fn builder() -> UpdateChatChannelsRequestBuilder {
        <UpdateChatChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateChatChannelsRequestBuilder {
    ban_media: Option<bool>,
    ban_urls: Option<bool>,
    banned_words: Option<Vec<String>>,
    user_posts_cooldown_seconds: Option<i64>,
    who_can_post: Option<WhoCanPostTypes>,
    who_can_react: Option<WhoCanReactTypes>,
}

impl UpdateChatChannelsRequestBuilder {
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

    pub fn who_can_post(mut self, value: WhoCanPostTypes) -> Self {
        self.who_can_post = Some(value);
        self
    }

    pub fn who_can_react(mut self, value: WhoCanReactTypes) -> Self {
        self.who_can_react = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateChatChannelsRequest`].
    pub fn build(self) -> Result<UpdateChatChannelsRequest, BuildError> {
        Ok(UpdateChatChannelsRequest {
            ban_media: self.ban_media,
            ban_urls: self.ban_urls,
            banned_words: self.banned_words,
            user_posts_cooldown_seconds: self.user_posts_cooldown_seconds,
            who_can_post: self.who_can_post,
            who_can_react: self.who_can_react,
        })
    }
}
