pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Review {
    /// The account that this review was written for.
    #[serde(default)]
    pub account: ReviewAccount,
    #[serde(default)]
    pub attachments: Vec<ReviewAttachment>,
    /// When the review was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The body text of the review containing the user's detailed feedback. Returns an empty string if no description was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The unique identifier for the review.
    #[serde(default)]
    pub id: String,
    /// When the reviewer first joined the product, as an ISO 8601 timestamp. Null if unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<String>,
    /// Whether the reviewer paid for the product. Null if the payment status is unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_for_product: Option<bool>,
    /// The product that this review was written for.
    #[serde(default)]
    pub product: ReviewProduct,
    /// When the review was published, as an ISO 8601 timestamp. Null if unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published_at: Option<String>,
    /// The star rating given by the reviewer, from 1 to 5.
    #[serde(default)]
    pub stars: i64,
    /// The moderation status of the review. Only published reviews are returned.
    pub status: ReviewStatus,
    /// A short summary title for the review. Null if the reviewer did not provide one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// When the review was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// The user account of the person who wrote this review.
    #[serde(default)]
    pub user: ReviewUser,
}

impl Review {
    pub fn builder() -> ReviewBuilder {
        <ReviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewBuilder {
    account: Option<ReviewAccount>,
    attachments: Option<Vec<ReviewAttachment>>,
    created_at: Option<String>,
    description: Option<String>,
    id: Option<String>,
    joined_at: Option<String>,
    paid_for_product: Option<bool>,
    product: Option<ReviewProduct>,
    published_at: Option<String>,
    stars: Option<i64>,
    status: Option<ReviewStatus>,
    title: Option<String>,
    updated_at: Option<String>,
    user: Option<ReviewUser>,
}

impl ReviewBuilder {
    pub fn account(mut self, value: ReviewAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn attachments(mut self, value: Vec<ReviewAttachment>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
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

    pub fn joined_at(mut self, value: impl Into<String>) -> Self {
        self.joined_at = Some(value.into());
        self
    }

    pub fn paid_for_product(mut self, value: bool) -> Self {
        self.paid_for_product = Some(value);
        self
    }

    pub fn product(mut self, value: ReviewProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn published_at(mut self, value: impl Into<String>) -> Self {
        self.published_at = Some(value.into());
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

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn user(mut self, value: ReviewUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Review`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account`](ReviewBuilder::account)
    /// - [`attachments`](ReviewBuilder::attachments)
    /// - [`created_at`](ReviewBuilder::created_at)
    /// - [`id`](ReviewBuilder::id)
    /// - [`product`](ReviewBuilder::product)
    /// - [`stars`](ReviewBuilder::stars)
    /// - [`status`](ReviewBuilder::status)
    /// - [`updated_at`](ReviewBuilder::updated_at)
    /// - [`user`](ReviewBuilder::user)
    pub fn build(self) -> Result<Review, BuildError> {
        Ok(Review {
            account: self
                .account
                .ok_or_else(|| BuildError::missing_field("account"))?,
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
            product: self
                .product
                .ok_or_else(|| BuildError::missing_field("product"))?,
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
