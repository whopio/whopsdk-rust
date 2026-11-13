pub use crate::prelude::*;

/// The company that initiated this setup intent. Null if the company has been deleted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetupIntentLegacyCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
}

impl SetupIntentLegacyCompany {
    pub fn builder() -> SetupIntentLegacyCompanyBuilder {
        <SetupIntentLegacyCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupIntentLegacyCompanyBuilder {
    id: Option<String>,
}

impl SetupIntentLegacyCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetupIntentLegacyCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SetupIntentLegacyCompanyBuilder::id)
    pub fn build(self) -> Result<SetupIntentLegacyCompany, BuildError> {
        Ok(SetupIntentLegacyCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
