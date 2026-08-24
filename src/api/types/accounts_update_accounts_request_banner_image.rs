pub use crate::prelude::*;

/// Account banner image, used as the cover photo when creating a Whop-managed Facebook page. Image files up to 10 MB, except `image/gif`. Pass a JSON object containing an `id` from [Create File](/api-reference/files/create-file).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateAccountsRequestBannerImage {
    /// ID of a file from [Create File](/api-reference/files/create-file), prefixed `file_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl UpdateAccountsRequestBannerImage {
    pub fn builder() -> UpdateAccountsRequestBannerImageBuilder {
        <UpdateAccountsRequestBannerImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAccountsRequestBannerImageBuilder {
    id: Option<String>,
}

impl UpdateAccountsRequestBannerImageBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateAccountsRequestBannerImage`].
    pub fn build(self) -> Result<UpdateAccountsRequestBannerImage, BuildError> {
        Ok(UpdateAccountsRequestBannerImage {
            id: self.id,
            extra: Default::default(),
        })
    }
}
