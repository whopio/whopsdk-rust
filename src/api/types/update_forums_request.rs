pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateForumsRequest {
    /// A list of words that are automatically blocked from posts in this forum. For example, ['spam', 'scam'].
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banned_words: Option<Vec<String>>,
    /// The banner image displayed at the top of the forum page. Pass null to remove the existing banner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<UpdateForumsRequestBannerImage>,
    /// Controls how email notifications are sent to members when new posts are created in this forum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_notification_preference: Option<ForumEmailNotificationPreferences>,
    /// Controls which roles are allowed to comment on posts in this forum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_can_comment: Option<ForumWhoCanCommentTypes>,
    /// Controls which roles are allowed to create new posts in this forum.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_can_post: Option<ForumWhoCanPostTypes>,
}

impl UpdateForumsRequest {
    pub fn builder() -> UpdateForumsRequestBuilder {
        <UpdateForumsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateForumsRequestBuilder {
    banned_words: Option<Vec<String>>,
    banner_image: Option<UpdateForumsRequestBannerImage>,
    email_notification_preference: Option<ForumEmailNotificationPreferences>,
    who_can_comment: Option<ForumWhoCanCommentTypes>,
    who_can_post: Option<ForumWhoCanPostTypes>,
}

impl UpdateForumsRequestBuilder {
    pub fn banned_words(mut self, value: Vec<String>) -> Self {
        self.banned_words = Some(value);
        self
    }

    pub fn banner_image(mut self, value: UpdateForumsRequestBannerImage) -> Self {
        self.banner_image = Some(value);
        self
    }

    pub fn email_notification_preference(
        mut self,
        value: ForumEmailNotificationPreferences,
    ) -> Self {
        self.email_notification_preference = Some(value);
        self
    }

    pub fn who_can_comment(mut self, value: ForumWhoCanCommentTypes) -> Self {
        self.who_can_comment = Some(value);
        self
    }

    pub fn who_can_post(mut self, value: ForumWhoCanPostTypes) -> Self {
        self.who_can_post = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateForumsRequest`].
    pub fn build(self) -> Result<UpdateForumsRequest, BuildError> {
        Ok(UpdateForumsRequest {
            banned_words: self.banned_words,
            banner_image: self.banner_image,
            email_notification_preference: self.email_notification_preference,
            who_can_comment: self.who_can_comment,
            who_can_post: self.who_can_post,
        })
    }
}
