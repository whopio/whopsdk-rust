pub use crate::prelude::*;

/// A token transaction records a credit or debit to a member's token balance within a company, including transfers between members.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompanyTokenTransaction {
    /// The token amount for this transaction. Always a positive value regardless of transaction type.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub amount: f64,
    /// The company whose token balance this transaction affects.
    #[serde(default)]
    pub company: CompanyTokenTransactionCompany,
    /// The datetime the company token transaction was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// Free-text description explaining the reason for this token transaction. Null if no description was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The unique identifier for the company token transaction.
    #[serde(default)]
    pub id: String,
    /// A unique key used to prevent duplicate transactions when retrying API requests. Null if no idempotency key was provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    /// The ID of the corresponding transaction on the other side of a transfer. Null if this is not a transfer transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_transaction_id: Option<String>,
    /// The member whose token balance was affected by this transaction.
    #[serde(default)]
    pub member: CompanyTokenTransactionMember,
    /// The direction of this token transaction (add, subtract, or transfer).
    pub transaction_type: CompanyTokenTransactionTypes,
    /// The user whose token balance was affected by this transaction.
    #[serde(default)]
    pub user: CompanyTokenTransactionUser,
}

impl CompanyTokenTransaction {
    pub fn builder() -> CompanyTokenTransactionBuilder {
        <CompanyTokenTransactionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyTokenTransactionBuilder {
    amount: Option<f64>,
    company: Option<CompanyTokenTransactionCompany>,
    created_at: Option<DateTime<FixedOffset>>,
    description: Option<String>,
    id: Option<String>,
    idempotency_key: Option<String>,
    linked_transaction_id: Option<String>,
    member: Option<CompanyTokenTransactionMember>,
    transaction_type: Option<CompanyTokenTransactionTypes>,
    user: Option<CompanyTokenTransactionUser>,
}

impl CompanyTokenTransactionBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn company(mut self, value: CompanyTokenTransactionCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn linked_transaction_id(mut self, value: impl Into<String>) -> Self {
        self.linked_transaction_id = Some(value.into());
        self
    }

    pub fn member(mut self, value: CompanyTokenTransactionMember) -> Self {
        self.member = Some(value);
        self
    }

    pub fn transaction_type(mut self, value: CompanyTokenTransactionTypes) -> Self {
        self.transaction_type = Some(value);
        self
    }

    pub fn user(mut self, value: CompanyTokenTransactionUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompanyTokenTransaction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](CompanyTokenTransactionBuilder::amount)
    /// - [`company`](CompanyTokenTransactionBuilder::company)
    /// - [`created_at`](CompanyTokenTransactionBuilder::created_at)
    /// - [`id`](CompanyTokenTransactionBuilder::id)
    /// - [`member`](CompanyTokenTransactionBuilder::member)
    /// - [`transaction_type`](CompanyTokenTransactionBuilder::transaction_type)
    /// - [`user`](CompanyTokenTransactionBuilder::user)
    pub fn build(self) -> Result<CompanyTokenTransaction, BuildError> {
        Ok(CompanyTokenTransaction {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            company: self
                .company
                .ok_or_else(|| BuildError::missing_field("company"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            description: self.description,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            idempotency_key: self.idempotency_key,
            linked_transaction_id: self.linked_transaction_id,
            member: self
                .member
                .ok_or_else(|| BuildError::missing_field("member"))?,
            transaction_type: self
                .transaction_type
                .ok_or_else(|| BuildError::missing_field("transaction_type"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
