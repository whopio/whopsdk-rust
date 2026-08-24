pub use crate::prelude::*;

/// Account logo, used as the profile picture when creating a Whop-managed Facebook page. Image files up to 5 MB. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAccountsRequestLogo {
    /// ID of a file from [Create File](/api-reference/files/create-file), prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UpdateAccountsRequestLogo {
    pub fn builder() -> UpdateAccountsRequestLogoBuilder {
        <UpdateAccountsRequestLogoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAccountsRequestLogoBuilder {
    id: Option<String>,
}

impl UpdateAccountsRequestLogoBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAccountsRequestLogo`].
    pub fn build(self) -> Result<UpdateAccountsRequestLogo, BuildError> {
        Ok(UpdateAccountsRequestLogo {
            id: self.id,
            extra: Default::default(),
        })
    }
}
