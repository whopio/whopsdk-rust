pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAffiliatesRequest {
    /// The ID of the company to create the affiliate for.
    #[serde(default)]
    pub account_id: String,
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
    account_id: Option<String>,
    user_identifier: Option<String>,
}

impl CreateAffiliatesRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn user_identifier(mut self, value: impl Into<String>) -> Self {
        self.user_identifier = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAffiliatesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateAffiliatesRequestBuilder::account_id)
    /// - [`user_identifier`](CreateAffiliatesRequestBuilder::user_identifier)
    pub fn build(self) -> Result<CreateAffiliatesRequest, BuildError> {
        Ok(CreateAffiliatesRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            user_identifier: self
                .user_identifier
                .ok_or_else(|| BuildError::missing_field("user_identifier"))?,
        })
    }
}
