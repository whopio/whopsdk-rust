pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateProductsRequestGalleryImagesItem {
    /// The signed ID of a completed direct upload, as an alternative to id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    /// The tag of an already-uploaded attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl UpdateProductsRequestGalleryImagesItem {
    pub fn builder() -> UpdateProductsRequestGalleryImagesItemBuilder {
        <UpdateProductsRequestGalleryImagesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateProductsRequestGalleryImagesItemBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl UpdateProductsRequestGalleryImagesItemBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateProductsRequestGalleryImagesItem`].
    pub fn build(self) -> Result<UpdateProductsRequestGalleryImagesItem, BuildError> {
        Ok(UpdateProductsRequestGalleryImagesItem {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
