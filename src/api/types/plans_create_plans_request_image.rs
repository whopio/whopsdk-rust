pub use crate::prelude::*;

/// An image displayed on the product page to represent this plan.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePlansRequestImage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direct_upload_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl CreatePlansRequestImage {
    pub fn builder() -> CreatePlansRequestImageBuilder {
        <CreatePlansRequestImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlansRequestImageBuilder {
    direct_upload_id: Option<String>,
    id: Option<String>,
}

impl CreatePlansRequestImageBuilder {
    pub fn direct_upload_id(mut self, value: impl Into<String>) -> Self {
        self.direct_upload_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePlansRequestImage`].
    pub fn build(self) -> Result<CreatePlansRequestImage, BuildError> {
        Ok(CreatePlansRequestImage {
            direct_upload_id: self.direct_upload_id,
            id: self.id,
        })
    }
}
