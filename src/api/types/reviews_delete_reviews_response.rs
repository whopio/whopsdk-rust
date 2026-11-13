pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteReviewsResponse {
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub id: String,
}

impl DeleteReviewsResponse {
    pub fn builder() -> DeleteReviewsResponseBuilder {
        <DeleteReviewsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteReviewsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteReviewsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteReviewsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteReviewsResponseBuilder::deleted)
    /// - [`id`](DeleteReviewsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteReviewsResponse, BuildError> {
        Ok(DeleteReviewsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
