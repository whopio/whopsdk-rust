pub use crate::prelude::*;

/// A wide image for the product, shown on the product page and on listing cards. Pass `{ id }` for an existing attachment or `{ direct_upload_id }` for a completed direct upload; `null` removes it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateProductsRequestBannerImage {
    /// The signed id of a completed direct upload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    /// The tag of an already-uploaded attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UpdateProductsRequestBannerImage {
    pub fn builder() -> UpdateProductsRequestBannerImageBuilder {
        <UpdateProductsRequestBannerImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateProductsRequestBannerImageBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl UpdateProductsRequestBannerImageBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateProductsRequestBannerImage`].
    pub fn build(self) -> Result<UpdateProductsRequestBannerImage, BuildError> {
        Ok(UpdateProductsRequestBannerImage {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
