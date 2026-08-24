pub use crate::prelude::*;

/// Open Graph preview media used when the account is shared. Image and video files up to 5 MB. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAccountsRequestOpengraphImage {
    /// ID of a file from [Create File](/api-reference/files/create-file), prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UpdateAccountsRequestOpengraphImage {
    pub fn builder() -> UpdateAccountsRequestOpengraphImageBuilder {
        <UpdateAccountsRequestOpengraphImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAccountsRequestOpengraphImageBuilder {
    id: Option<String>,
}

impl UpdateAccountsRequestOpengraphImageBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAccountsRequestOpengraphImage`].
    pub fn build(self) -> Result<UpdateAccountsRequestOpengraphImage, BuildError> {
        Ok(UpdateAccountsRequestOpengraphImage {
            id: self.id,
            extra: Default::default(),
        })
    }
}
