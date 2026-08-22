pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAdsResponse {
    /// Always true.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the deleted ad.
    #[serde(default)]
    pub id: String,
}

impl DeleteAdsResponse {
    pub fn builder() -> DeleteAdsResponseBuilder {
        <DeleteAdsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAdsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteAdsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAdsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteAdsResponseBuilder::deleted)
    /// - [`id`](DeleteAdsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteAdsResponse, BuildError> {
        Ok(DeleteAdsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
