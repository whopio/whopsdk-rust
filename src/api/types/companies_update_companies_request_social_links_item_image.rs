pub use crate::prelude::*;

/// The custom image for the social link
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCompaniesRequestSocialLinksItemImage {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateCompaniesRequestSocialLinksItemImage {
    pub fn builder() -> UpdateCompaniesRequestSocialLinksItemImageBuilder {
        <UpdateCompaniesRequestSocialLinksItemImageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCompaniesRequestSocialLinksItemImageBuilder {
    id: Option<String>,
}

impl UpdateCompaniesRequestSocialLinksItemImageBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCompaniesRequestSocialLinksItemImage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateCompaniesRequestSocialLinksItemImageBuilder::id)
    pub fn build(self) -> Result<UpdateCompaniesRequestSocialLinksItemImage, BuildError> {
        Ok(UpdateCompaniesRequestSocialLinksItemImage {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
