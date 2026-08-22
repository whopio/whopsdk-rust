pub use crate::prelude::*;

/// The company associated with this payout destination. Null if not linked to a specific company.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutMethodListItemCompany {
    /// The unique identifier for the company.
    #[serde(default)]
    pub id: String,
}

impl PayoutMethodListItemCompany {
    pub fn builder() -> PayoutMethodListItemCompanyBuilder {
        <PayoutMethodListItemCompanyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutMethodListItemCompanyBuilder {
    id: Option<String>,
}

impl PayoutMethodListItemCompanyBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayoutMethodListItemCompany`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PayoutMethodListItemCompanyBuilder::id)
    pub fn build(self) -> Result<PayoutMethodListItemCompany, BuildError> {
        Ok(PayoutMethodListItemCompany {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
