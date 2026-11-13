pub use crate::prelude::*;

/// The account's privacy policy document. PDF only. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file), or `null` to remove it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAccountsRequestPrivacyPolicy {
    /// ID of a file from [Create File](/api-reference/files/create-file), prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UpdateAccountsRequestPrivacyPolicy {
    pub fn builder() -> UpdateAccountsRequestPrivacyPolicyBuilder {
        <UpdateAccountsRequestPrivacyPolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAccountsRequestPrivacyPolicyBuilder {
    id: Option<String>,
}

impl UpdateAccountsRequestPrivacyPolicyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAccountsRequestPrivacyPolicy`].
    pub fn build(self) -> Result<UpdateAccountsRequestPrivacyPolicy, BuildError> {
        Ok(UpdateAccountsRequestPrivacyPolicy {
            id: self.id,
            extra: Default::default(),
        })
    }
}
