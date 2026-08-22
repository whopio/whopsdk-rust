pub use crate::prelude::*;

/// A logo image displayed alongside the experience name.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateExperiencesRequestLogo {
    /// The ID of an existing file object.
    #[serde(default)]
    pub id: String,
}

impl UpdateExperiencesRequestLogo {
    pub fn builder() -> UpdateExperiencesRequestLogoBuilder {
        <UpdateExperiencesRequestLogoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateExperiencesRequestLogoBuilder {
    id: Option<String>,
}

impl UpdateExperiencesRequestLogoBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateExperiencesRequestLogo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateExperiencesRequestLogoBuilder::id)
    pub fn build(self) -> Result<UpdateExperiencesRequestLogo, BuildError> {
        Ok(UpdateExperiencesRequestLogo {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
