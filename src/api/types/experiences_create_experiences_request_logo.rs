pub use crate::prelude::*;

/// A logo image displayed alongside the experience name.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateExperiencesRequestLogo {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl CreateExperiencesRequestLogo {
    pub fn builder() -> CreateExperiencesRequestLogoBuilder {
        <CreateExperiencesRequestLogoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateExperiencesRequestLogoBuilder {
    id: Option<String>,
}

impl CreateExperiencesRequestLogoBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateExperiencesRequestLogo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateExperiencesRequestLogoBuilder::id)
    pub fn build(self) -> Result<CreateExperiencesRequestLogo, BuildError> {
        Ok(CreateExperiencesRequestLogo {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
