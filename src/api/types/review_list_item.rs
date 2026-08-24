pub use crate::prelude::*;

/// A user-submitted review of a company, including a star rating and optional text feedback.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ReviewListItem {
    /// A list of files and media attached to the review.
    #[serde(default)]
    pub attachments: Vec<ReviewListItemAttachmentsItem>,
    /// The datetime the review was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The body text of the review containing the user's detailed feedback. Returns an empty string if no description was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The unique identifier for the review.
    #[serde(default)]
    pub id: String,
    /// The timestamp of when the reviewer first joined the product. Null if unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub joined_at: Option<DateTime<FixedOffset>>,
    /// Whether the reviewer paid for the product. Null if the payment status is unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_for_product: Option<bool>,
    /// The timestamp of when the review was published. Null if the review has not been published yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub published_at: Option<DateTime<FixedOffset>>,
    /// The star rating given by the reviewer, from 1 to 5.
    #[serde(default)]
    pub stars: i64,
    /// The current moderation status of the review.
    pub status: ReviewStatus,
    /// A short summary title for the review. Null if the reviewer did not provide one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The datetime the review was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// The user account of the person who wrote this review.
    #[serde(default)]
    pub user: ReviewListItemUser,
}

impl ReviewListItem {
    pub fn builder() -> ReviewListItemBuilder {
        <ReviewListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewListItemBuilder {
    attachments: Option<Vec<ReviewListItemAttachmentsItem>>,
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    id: Option<String>,
    joined_at: Option<DateTime<FixedOffset>>,
    paid_for_product: Option<bool>,
    published_at: Option<DateTime<FixedOffset>>,
    stars: Option<i64>,
    status: Option<ReviewStatus>,
    title: Option<String>,
    updated_at: Option<DateTime<FixedOffset>>,
    user: Option<ReviewListItemUser>,
}

impl ReviewListItemBuilder {
    pub fn attachments(mut self, value: Vec<ReviewListItemAttachmentsItem>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn joined_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.joined_at = Some(value);
        self
    }

    pub fn paid_for_product(mut self, value: bool) -> Self {
        self.paid_for_product = Some(value);
        self
    }

    pub fn published_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.published_at = Some(value);
        self
    }

    pub fn stars(mut self, value: i64) -> Self {
        self.stars = Some(value);
        self
    }

    pub fn status(mut self, value: ReviewStatus) -> Self {
        self.status = Some(value);
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

    pub fn user(mut self, value: ReviewListItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReviewListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`attachments`](ReviewListItemBuilder::attachments)
    /// - [`created_at`](ReviewListItemBuilder::created_at)
    /// - [`id`](ReviewListItemBuilder::id)
    /// - [`stars`](ReviewListItemBuilder::stars)
    /// - [`status`](ReviewListItemBuilder::status)
    /// - [`updated_at`](ReviewListItemBuilder::updated_at)
    /// - [`user`](ReviewListItemBuilder::user)
    pub fn build(self) -> Result<ReviewListItem, BuildError> {
        Ok(ReviewListItem {
            attachments: self
                .attachments
                .ok_or_else(|| BuildError::missing_field("attachments"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            joined_at: self.joined_at,
            paid_for_product: self.paid_for_product,
            published_at: self.published_at,
            stars: self
                .stars
                .ok_or_else(|| BuildError::missing_field("stars"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            title: self.title,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
