pub use crate::prelude::*;

/// The company that initiated this setup intent. Null if the company has been deleted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSetupIntentsResponseCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
}

impl CreateSetupIntentsResponseCompany {
    pub fn builder() -> CreateSetupIntentsResponseCompanyBuilder {
        <CreateSetupIntentsResponseCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSetupIntentsResponseCompanyBuilder {
    id: Option<String>,
}

impl CreateSetupIntentsResponseCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSetupIntentsResponseCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateSetupIntentsResponseCompanyBuilder::id)
    pub fn build(self) -> Result<CreateSetupIntentsResponseCompany, BuildError> {
        Ok(CreateSetupIntentsResponseCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
