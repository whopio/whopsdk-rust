pub use crate::prelude::*;

/// The account's cancellation policy document. Attached to new disputes as the cancellation policy evidence, with the terms of service as the fallback. PDF only. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file), or `null` to remove it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAccountsRequestCancellationPolicy {
    /// ID of a file from [Create File](/api-reference/files/create-file), prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UpdateAccountsRequestCancellationPolicy {
    pub fn builder() -> UpdateAccountsRequestCancellationPolicyBuilder {
        <UpdateAccountsRequestCancellationPolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAccountsRequestCancellationPolicyBuilder {
    id: Option<String>,
}

impl UpdateAccountsRequestCancellationPolicyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAccountsRequestCancellationPolicy`].
    pub fn build(self) -> Result<UpdateAccountsRequestCancellationPolicy, BuildError> {
        Ok(UpdateAccountsRequestCancellationPolicy {
            id: self.id,
            extra: Default::default(),
        })
    }
}
