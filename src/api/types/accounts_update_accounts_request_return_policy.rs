pub use crate::prelude::*;

/// The account's return and refund policy document. Attached to new disputes as the refund policy evidence, with the terms of service as the fallback. PDF only. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file), or `null` to remove it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAccountsRequestReturnPolicy {
    /// ID of a file from [Create File](/api-reference/files/create-file), prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UpdateAccountsRequestReturnPolicy {
    pub fn builder() -> UpdateAccountsRequestReturnPolicyBuilder {
        <UpdateAccountsRequestReturnPolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAccountsRequestReturnPolicyBuilder {
    id: Option<String>,
}

impl UpdateAccountsRequestReturnPolicyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAccountsRequestReturnPolicy`].
    pub fn build(self) -> Result<UpdateAccountsRequestReturnPolicy, BuildError> {
        Ok(UpdateAccountsRequestReturnPolicy {
            id: self.id,
            extra: Default::default(),
        })
    }
}
