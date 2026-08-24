pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReviewsListQueryRequest {
    /// Returns the elements in the list that come after the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Returns the elements in the list that come before the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Returns the first _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Returns the last _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// The unique identifier of the product to list reviews for.
    #[serde(default)]
    pub product_id: String,
    /// The minimum star rating to include in results, from 1 to 5 inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_stars: Option<i64>,
    /// The maximum star rating to include in results, from 1 to 5 inclusive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_stars: Option<i64>,
    /// Only return reviews created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only return reviews created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
}

impl ReviewsListQueryRequest {
    pub fn builder() -> ReviewsListQueryRequestBuilder {
        <ReviewsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewsListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    product_id: Option<String>,
    min_stars: Option<i64>,
    max_stars: Option<i64>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
}

impl ReviewsListQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn min_stars(mut self, value: i64) -> Self {
        self.min_stars = Some(value);
        self
    }

    pub fn max_stars(mut self, value: i64) -> Self {
        self.max_stars = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReviewsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`product_id`](ReviewsListQueryRequestBuilder::product_id)
    pub fn build(self) -> Result<ReviewsListQueryRequest, BuildError> {
        Ok(ReviewsListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
            min_stars: self.min_stars,
            max_stars: self.max_stars,
            created_before: self.created_before,
            created_after: self.created_after,
        })
    }
}
