pub use crate::prelude::*;

/// The company's logo image. Accepts PNG, JPEG, or GIF format.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateCompaniesRequestLogo {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl CreateCompaniesRequestLogo {
    pub fn builder() -> CreateCompaniesRequestLogoBuilder {
        <CreateCompaniesRequestLogoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCompaniesRequestLogoBuilder {
    id: Option<String>,
}

impl CreateCompaniesRequestLogoBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCompaniesRequestLogo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateCompaniesRequestLogoBuilder::id)
    pub fn build(self) -> Result<CreateCompaniesRequestLogo, BuildError> {
        Ok(CreateCompaniesRequestLogo {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
