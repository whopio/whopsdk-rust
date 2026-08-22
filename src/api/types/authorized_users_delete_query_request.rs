pub use crate::prelude::*;

/// Query parameters for delete
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthorizedUsersDeleteQueryRequest {
    /// The ID of the company the authorized user belongs to. Optional if the authorized user ID is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
}

impl AuthorizedUsersDeleteQueryRequest {
    pub fn builder() -> AuthorizedUsersDeleteQueryRequestBuilder {
        <AuthorizedUsersDeleteQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthorizedUsersDeleteQueryRequestBuilder {
    company_id: Option<String>,
}

impl AuthorizedUsersDeleteQueryRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AuthorizedUsersDeleteQueryRequest`].
    pub fn build(self) -> Result<AuthorizedUsersDeleteQueryRequest, BuildError> {
        Ok(AuthorizedUsersDeleteQueryRequest {
            company_id: self.company_id,
        })
    }
}
