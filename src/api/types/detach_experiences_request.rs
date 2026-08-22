pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DetachExperiencesRequest {
    /// The unique identifier of the product to detach the experience from.
    #[serde(default)]
    pub product_id: String,
}

impl DetachExperiencesRequest {
    pub fn builder() -> DetachExperiencesRequestBuilder {
        <DetachExperiencesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DetachExperiencesRequestBuilder {
    product_id: Option<String>,
}

impl DetachExperiencesRequestBuilder {
    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DetachExperiencesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`product_id`](DetachExperiencesRequestBuilder::product_id)
    pub fn build(self) -> Result<DetachExperiencesRequest, BuildError> {
        Ok(DetachExperiencesRequest {
            product_id: self
                .product_id
                .ok_or_else(|| BuildError::missing_field("product_id"))?,
        })
    }
}
