pub use crate::prelude::*;

/// A discussion forum where members can create posts, comment, and react, belonging to an experience.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Forum {
    /// A list of words that are automatically filtered from posts in this forum.
    #[serde(default)]
    pub banned_words: Vec<String>,
    /// The banner image displayed at the top of the forum page. Null if no banner has been uploaded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner_image: Option<ForumBannerImage>,
    /// The email notification setting that controls which posts trigger email alerts. One of: all_admin_posts, only_weekly_summary, none.
    pub email_notification_preference: ForumEmailNotificationPreferences,
    /// The parent experience that this forum belongs to.
    #[serde(default)]
    pub experience: ForumExperience,
    /// The unique identifier for the entity
    #[serde(default)]
    pub id: String,
    /// The permission level controlling who can comment on posts. One of: everyone, admins.
    pub who_can_comment: ForumWhoCanCommentTypes,
    /// The permission level controlling who can create new posts. One of: everyone, admins.
    pub who_can_post: ForumWhoCanPostTypes,
}

impl Forum {
    pub fn builder() -> ForumBuilder {
        <ForumBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForumBuilder {
    banned_words: Option<Vec<String>>,
    banner_image: Option<ForumBannerImage>,
    email_notification_preference: Option<ForumEmailNotificationPreferences>,
    experience: Option<ForumExperience>,
    id: Option<String>,
    who_can_comment: Option<ForumWhoCanCommentTypes>,
    who_can_post: Option<ForumWhoCanPostTypes>,
}

impl ForumBuilder {
    pub fn banned_words(mut self, value: Vec<String>) -> Self {
        self.banned_words = Some(value);
        self
    }

    pub fn banner_image(mut self, value: ForumBannerImage) -> Self {
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

    pub fn experience(mut self, value: ForumExperience) -> Self {
        self.experience = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    /// Consumes the builder and constructs a [`Forum`].
    /// This method will fail if any of the following fields are not set:
    /// - [`banned_words`](ForumBuilder::banned_words)
    /// - [`email_notification_preference`](ForumBuilder::email_notification_preference)
    /// - [`experience`](ForumBuilder::experience)
    /// - [`id`](ForumBuilder::id)
    /// - [`who_can_comment`](ForumBuilder::who_can_comment)
    /// - [`who_can_post`](ForumBuilder::who_can_post)
    pub fn build(self) -> Result<Forum, BuildError> {
        Ok(Forum {
            banned_words: self
                .banned_words
                .ok_or_else(|| BuildError::missing_field("banned_words"))?,
            banner_image: self.banner_image,
            email_notification_preference: self
                .email_notification_preference
                .ok_or_else(|| BuildError::missing_field("email_notification_preference"))?,
            experience: self
                .experience
                .ok_or_else(|| BuildError::missing_field("experience"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            who_can_comment: self
                .who_can_comment
                .ok_or_else(|| BuildError::missing_field("who_can_comment"))?,
            who_can_post: self
                .who_can_post
                .ok_or_else(|| BuildError::missing_field("who_can_post"))?,
        })
    }
}
