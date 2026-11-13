pub use crate::prelude::*;

/// The account's shipping policy document. Sent with physical-goods dispute evidence. PDF only. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file), or `null` to remove it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAccountsRequestShippingPolicy {
    /// ID of a file from [Create File](/api-reference/files/create-file), prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UpdateAccountsRequestShippingPolicy {
    pub fn builder() -> UpdateAccountsRequestShippingPolicyBuilder {
        <UpdateAccountsRequestShippingPolicyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAccountsRequestShippingPolicyBuilder {
    id: Option<String>,
}

impl UpdateAccountsRequestShippingPolicyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAccountsRequestShippingPolicy`].
    pub fn build(self) -> Result<UpdateAccountsRequestShippingPolicy, BuildError> {
        Ok(UpdateAccountsRequestShippingPolicy {
            id: self.id,
            extra: Default::default(),
        })
    }
}
