pub use crate::prelude::*;

/// The company that initiated this setup intent. Null if the company has been deleted.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetupIntentListItemCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
}

impl SetupIntentListItemCompany {
    pub fn builder() -> SetupIntentListItemCompanyBuilder {
        <SetupIntentListItemCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupIntentListItemCompanyBuilder {
    id: Option<String>,
}

impl SetupIntentListItemCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetupIntentListItemCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SetupIntentListItemCompanyBuilder::id)
    pub fn build(self) -> Result<SetupIntentListItemCompany, BuildError> {
        Ok(SetupIntentListItemCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
