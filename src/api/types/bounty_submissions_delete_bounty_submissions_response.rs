pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteBountySubmissionsResponse {
    /// Always true.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the cancelled submission.
    #[serde(default)]
    pub id: String,
}

impl DeleteBountySubmissionsResponse {
    pub fn builder() -> DeleteBountySubmissionsResponseBuilder {
        <DeleteBountySubmissionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteBountySubmissionsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteBountySubmissionsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteBountySubmissionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteBountySubmissionsResponseBuilder::deleted)
    /// - [`id`](DeleteBountySubmissionsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteBountySubmissionsResponse, BuildError> {
        Ok(DeleteBountySubmissionsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
