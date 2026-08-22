pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteSocialAccountsResponse {
    /// Always true.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the disconnected social account.
    #[serde(default)]
    pub id: String,
}

impl DeleteSocialAccountsResponse {
    pub fn builder() -> DeleteSocialAccountsResponseBuilder {
        <DeleteSocialAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteSocialAccountsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteSocialAccountsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteSocialAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteSocialAccountsResponseBuilder::deleted)
    /// - [`id`](DeleteSocialAccountsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteSocialAccountsResponse, BuildError> {
        Ok(DeleteSocialAccountsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
