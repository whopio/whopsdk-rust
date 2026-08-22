pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAffiliatesRequest {
    /// The ID of the company to create the affiliate for.
    #[serde(default)]
    pub company_id: String,
    /// The user identifier (username, email, user ID, or Discord ID).
    #[serde(default)]
    pub user_identifier: String,
}

impl CreateAffiliatesRequest {
    pub fn builder() -> CreateAffiliatesRequestBuilder {
        <CreateAffiliatesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAffiliatesRequestBuilder {
    company_id: Option<String>,
    user_identifier: Option<String>,
}

impl CreateAffiliatesRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn user_identifier(mut self, value: impl Into<String>) -> Self {
        self.user_identifier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAffiliatesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](CreateAffiliatesRequestBuilder::company_id)
    /// - [`user_identifier`](CreateAffiliatesRequestBuilder::user_identifier)
    pub fn build(self) -> Result<CreateAffiliatesRequest, BuildError> {
        Ok(CreateAffiliatesRequest {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            user_identifier: self
                .user_identifier
                .ok_or_else(|| BuildError::missing_field("user_identifier"))?,
        })
    }
}
