pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AttachExperiencesRequest {
    /// The unique identifier of the product to attach the experience to.
    #[serde(default)]
    pub product_id: String,
}

impl AttachExperiencesRequest {
    pub fn builder() -> AttachExperiencesRequestBuilder {
        <AttachExperiencesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachExperiencesRequestBuilder {
    product_id: Option<String>,
}

impl AttachExperiencesRequestBuilder {
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AttachExperiencesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`product_id`](AttachExperiencesRequestBuilder::product_id)
    pub fn build(self) -> Result<AttachExperiencesRequest, BuildError> {
        Ok(AttachExperiencesRequest {
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
        })
    }
}
