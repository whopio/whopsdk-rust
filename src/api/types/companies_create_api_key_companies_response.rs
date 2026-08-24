pub use crate::prelude::*;

/// An API key created for a child company, including the one-time secret key.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateApiKeyCompaniesResponse {
    /// The unique identifier for the authorized api key.
    #[serde(default)]
    pub id: String,
    /// A user set name to identify an API key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The secret key used to authenticate requests. Only returned at creation time.
    #[serde(default)]
    pub secret_key: String,
}

impl CreateApiKeyCompaniesResponse {
    pub fn builder() -> CreateApiKeyCompaniesResponseBuilder {
        <CreateApiKeyCompaniesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApiKeyCompaniesResponseBuilder {
    id: Option<String>,
    name: Option<String>,
    secret_key: Option<String>,
}

impl CreateApiKeyCompaniesResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn secret_key(mut self, value: impl Into<String>) -> Self {
        self.secret_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateApiKeyCompaniesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateApiKeyCompaniesResponseBuilder::id)
    /// - [`secret_key`](CreateApiKeyCompaniesResponseBuilder::secret_key)
    pub fn build(self) -> Result<CreateApiKeyCompaniesResponse, BuildError> {
        Ok(CreateApiKeyCompaniesResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            secret_key: self
                .secret_key
                .ok_or_else(|| BuildError::missing_field("secret_key"))?,
        })
    }
}
