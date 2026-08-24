pub use crate::prelude::*;

/// The member whose token balance was affected by this transaction.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompanyTokenTransactionMember {
    /// The unique identifier for the company member.
    #[serde(default)]
    pub id: String,
}

impl CompanyTokenTransactionMember {
    pub fn builder() -> CompanyTokenTransactionMemberBuilder {
        <CompanyTokenTransactionMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyTokenTransactionMemberBuilder {
    id: Option<String>,
}

impl CompanyTokenTransactionMemberBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompanyTokenTransactionMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompanyTokenTransactionMemberBuilder::id)
    pub fn build(self) -> Result<CompanyTokenTransactionMember, BuildError> {
        Ok(CompanyTokenTransactionMember {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
