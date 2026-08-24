pub use crate::prelude::*;

/// The company's logo image. Accepts PNG, JPEG, or GIF format.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCompaniesRequestLogo {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateCompaniesRequestLogo {
    pub fn builder() -> UpdateCompaniesRequestLogoBuilder {
        <UpdateCompaniesRequestLogoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCompaniesRequestLogoBuilder {
    id: Option<String>,
}

impl UpdateCompaniesRequestLogoBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateCompaniesRequestLogo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateCompaniesRequestLogoBuilder::id)
    pub fn build(self) -> Result<UpdateCompaniesRequestLogo, BuildError> {
        Ok(UpdateCompaniesRequestLogo {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
