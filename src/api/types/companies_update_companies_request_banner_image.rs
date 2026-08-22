pub use crate::prelude::*;

/// The company's banner image. Accepts PNG or JPEG format.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCompaniesRequestBannerImage {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateCompaniesRequestBannerImage {
    pub fn builder() -> UpdateCompaniesRequestBannerImageBuilder {
        <UpdateCompaniesRequestBannerImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCompaniesRequestBannerImageBuilder {
    id: Option<String>,
}

impl UpdateCompaniesRequestBannerImageBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCompaniesRequestBannerImage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateCompaniesRequestBannerImageBuilder::id)
    pub fn build(self) -> Result<UpdateCompaniesRequestBannerImage, BuildError> {
        Ok(UpdateCompaniesRequestBannerImage {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
