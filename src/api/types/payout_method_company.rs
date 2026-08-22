pub use crate::prelude::*;

/// The company associated with this payout destination. Null if not linked to a specific company.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutMethodCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
}

impl PayoutMethodCompany {
    pub fn builder() -> PayoutMethodCompanyBuilder {
        <PayoutMethodCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutMethodCompanyBuilder {
    id: Option<String>,
}

impl PayoutMethodCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayoutMethodCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PayoutMethodCompanyBuilder::id)
    pub fn build(self) -> Result<PayoutMethodCompany, BuildError> {
        Ok(PayoutMethodCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
