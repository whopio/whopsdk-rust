pub use crate::prelude::*;

/// The member whose token balance was affected by this transaction.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompanyTokenTransactionListItemMember {
    /// The unique identifier for the company member.
    #[serde(default)]
    pub id: String,
}

impl CompanyTokenTransactionListItemMember {
    pub fn builder() -> CompanyTokenTransactionListItemMemberBuilder {
        <CompanyTokenTransactionListItemMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyTokenTransactionListItemMemberBuilder {
    id: Option<String>,
}

impl CompanyTokenTransactionListItemMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompanyTokenTransactionListItemMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompanyTokenTransactionListItemMemberBuilder::id)
    pub fn build(self) -> Result<CompanyTokenTransactionListItemMember, BuildError> {
        Ok(CompanyTokenTransactionListItemMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
