pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProductsRequestGalleryImagesItem {
    /// The signed ID of a completed direct upload, as an alternative to id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    /// The tag of an already-uploaded attachment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreateProductsRequestGalleryImagesItem {
    pub fn builder() -> CreateProductsRequestGalleryImagesItemBuilder {
        <CreateProductsRequestGalleryImagesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProductsRequestGalleryImagesItemBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl CreateProductsRequestGalleryImagesItemBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateProductsRequestGalleryImagesItem`].
    pub fn build(self) -> Result<CreateProductsRequestGalleryImagesItem, BuildError> {
        Ok(CreateProductsRequestGalleryImagesItem {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
