pub use crate::prelude::*;

/// The ledger account from which the withdrawal funds are sourced.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WithdrawalLedgerAccount {
    /// Represents a unique identifier that is Base64 obfuscated. It is often used to refetch an object or as key for a cache. The ID type appears in a JSON response as a String; however, it is not intended to be human-readable. When expected as an input type, any string (such as `"VXNlci0xMA=="`) or integer (such as `4`) input value will be accepted as an ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// The unique identifier for the ledger account.
    #[serde(default)]
    pub id: String,
}

impl WithdrawalLedgerAccount {
    pub fn builder() -> WithdrawalLedgerAccountBuilder {
        <WithdrawalLedgerAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WithdrawalLedgerAccountBuilder {
    company_id: Option<String>,
    id: Option<String>,
}

impl WithdrawalLedgerAccountBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WithdrawalLedgerAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](WithdrawalLedgerAccountBuilder::id)
    pub fn build(self) -> Result<WithdrawalLedgerAccount, BuildError> {
        Ok(WithdrawalLedgerAccount {
            company_id: self.company_id,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
