pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ProductGalleryImage {
    /// Uploaded file MIME type, such as image/jpeg.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// Gallery image ID.
    #[serde(default)]
    pub id: String,
    /// Pre-optimized URL for rendering this image on the client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl ProductGalleryImage {
    pub fn builder() -> ProductGalleryImageBuilder {
        <ProductGalleryImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProductGalleryImageBuilder {
    content_type: Option<String>,
    id: Option<String>,
    url: Option<String>,
}

impl ProductGalleryImageBuilder {
    pub fn content_type(mut self, value: impl Into<String>) -> Self {
        self.content_type = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ProductGalleryImage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ProductGalleryImageBuilder::id)
    pub fn build(self) -> Result<ProductGalleryImage, BuildError> {
        Ok(ProductGalleryImage {
            content_type: self.content_type,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            url: self.url,
        })
    }
}
