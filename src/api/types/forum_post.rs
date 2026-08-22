pub use crate::prelude::*;

/// A post or comment in a forum feed, supporting rich text, attachments, polls, and reactions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ForumPost {
    /// All file attachments on this post, such as images, documents, and videos.
    #[serde(default)]
    pub attachments: Vec<ForumPostAttachmentsItem>,
    /// The total number of direct comments on this post.
    #[serde(default)]
    pub comment_count: i64,
    /// The body of the forum post in Markdown format. Null if the post is paywalled and the current user does not have access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// The time this post was created, as a Unix timestamp.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Represents a unique identifier that is Base64 obfuscated. It is often used to refetch an object or as key for a cache. The ID type appears in a JSON response as a String; however, it is not intended to be human-readable. When expected as an input type, any string (such as `"VXNlci0xMA=="`) or integer (such as `4`) input value will be accepted as an ID.
    #[serde(default)]
    pub id: String,
    /// Whether this post has been edited after its initial creation.
    #[serde(default)]
    pub is_edited: bool,
    /// Whether this post is pinned to the top of the forum feed.
    #[serde(default)]
    pub is_pinned: bool,
    /// Whether the author of this post is an admin of the company that owns the forum.
    #[serde(default)]
    pub is_poster_admin: bool,
    /// The total number of like reactions this post has received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub like_count: Option<i64>,
    /// The unique identifier of the parent post. Null if this is a top-level post.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// The headline of the forum post. Null if the post has no title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The time this post was last updated, as a Unix timestamp.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The user who authored this forum post.
    #[serde(default)]
    pub user: ForumPostUser,
    /// The total number of times this post has been viewed by users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<i64>,
}

impl ForumPost {
    pub fn builder() -> ForumPostBuilder {
        <ForumPostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ForumPostBuilder {
    attachments: Option<Vec<ForumPostAttachmentsItem>>,
    comment_count: Option<i64>,
    content: Option<String>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    is_edited: Option<bool>,
    is_pinned: Option<bool>,
    is_poster_admin: Option<bool>,
    like_count: Option<i64>,
    parent_id: Option<String>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    user: Option<ForumPostUser>,
    view_count: Option<i64>,
}

impl ForumPostBuilder {
    pub fn attachments(mut self, value: Vec<ForumPostAttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn comment_count(mut self, value: i64) -> Self {
        self.comment_count = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_edited(mut self, value: bool) -> Self {
        self.is_edited = Some(value);
        self
    }

    pub fn is_pinned(mut self, value: bool) -> Self {
        self.is_pinned = Some(value);
        self
    }

    pub fn is_poster_admin(mut self, value: bool) -> Self {
        self.is_poster_admin = Some(value);
        self
    }

    pub fn like_count(mut self, value: i64) -> Self {
        self.like_count = Some(value);
        self
    }

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn user(mut self, value: ForumPostUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn view_count(mut self, value: i64) -> Self {
        self.view_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ForumPost`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachments`](ForumPostBuilder::attachments)
    /// - [`comment_count`](ForumPostBuilder::comment_count)
    /// - [`created_at`](ForumPostBuilder::created_at)
    /// - [`id`](ForumPostBuilder::id)
    /// - [`is_edited`](ForumPostBuilder::is_edited)
    /// - [`is_pinned`](ForumPostBuilder::is_pinned)
    /// - [`is_poster_admin`](ForumPostBuilder::is_poster_admin)
    /// - [`updated_at`](ForumPostBuilder::updated_at)
    /// - [`user`](ForumPostBuilder::user)
    pub fn build(self) -> Result<ForumPost, BuildError> {
        Ok(ForumPost {
            attachments: self
                .attachments
                .ok_or_else(|| BuildError::missing_field("attachments"))?,
            comment_count: self
                .comment_count
                .ok_or_else(|| BuildError::missing_field("comment_count"))?,
            content: self.content,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_edited: self
                .is_edited
                .ok_or_else(|| BuildError::missing_field("is_edited"))?,
            is_pinned: self
                .is_pinned
                .ok_or_else(|| BuildError::missing_field("is_pinned"))?,
            is_poster_admin: self
                .is_poster_admin
                .ok_or_else(|| BuildError::missing_field("is_poster_admin"))?,
            like_count: self.like_count,
            parent_id: self.parent_id,
            title: self.title,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
            view_count: self.view_count,
        })
    }
}
