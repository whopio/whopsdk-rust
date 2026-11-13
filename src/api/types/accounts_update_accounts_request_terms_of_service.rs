pub use crate::prelude::*;

/// The account's terms of service document. Attached to new disputes as the cancellation policy evidence when no cancellation policy is set. PDF only. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file), or `null` to remove it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAccountsRequestTermsOfService {
    /// ID of a file from [Create File](/api-reference/files/create-file), prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UpdateAccountsRequestTermsOfService {
    pub fn builder() -> UpdateAccountsRequestTermsOfServiceBuilder {
        <UpdateAccountsRequestTermsOfServiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAccountsRequestTermsOfServiceBuilder {
    id: Option<String>,
}

impl UpdateAccountsRequestTermsOfServiceBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAccountsRequestTermsOfService`].
    pub fn build(self) -> Result<UpdateAccountsRequestTermsOfService, BuildError> {
        Ok(UpdateAccountsRequestTermsOfService {
            id: self.id,
            extra: Default::default(),
        })
    }
}
