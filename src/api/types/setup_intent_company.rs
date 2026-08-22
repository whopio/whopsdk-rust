pub use crate::prelude::*;

/// The company that initiated this setup intent. Null if the company has been deleted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetupIntentCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
}

impl SetupIntentCompany {
    pub fn builder() -> SetupIntentCompanyBuilder {
        <SetupIntentCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupIntentCompanyBuilder {
    id: Option<String>,
}

impl SetupIntentCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetupIntentCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SetupIntentCompanyBuilder::id)
    pub fn build(self) -> Result<SetupIntentCompany, BuildError> {
        Ok(SetupIntentCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
