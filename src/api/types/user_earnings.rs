pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserEarnings {
    /// The first time the user earned gross income, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_earned_at: Option<String>,
    /// Gross income from accounts the user owns or is owner-authorized on.
    #[serde(default)]
    pub owned_accounts: UserEarningsAmount,
    /// Gross income from the user's personal wallet.
    #[serde(default)]
    pub personal: UserEarningsAmount,
    /// Gross income from the user's personal wallet plus accounts they own or are owner-authorized on.
    #[serde(default)]
    pub total: UserEarningsAmount,
}

impl UserEarnings {
    pub fn builder() -> UserEarningsBuilder {
        <UserEarningsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserEarningsBuilder {
    first_earned_at: Option<String>,
    owned_accounts: Option<UserEarningsAmount>,
    personal: Option<UserEarningsAmount>,
    total: Option<UserEarningsAmount>,
}

impl UserEarningsBuilder {
    pub fn first_earned_at(mut self, value: impl Into<String>) -> Self {
        self.first_earned_at = Some(value.into());
        self
    }

    pub fn owned_accounts(mut self, value: UserEarningsAmount) -> Self {
        self.owned_accounts = Some(value);
        self
    }

    pub fn personal(mut self, value: UserEarningsAmount) -> Self {
        self.personal = Some(value);
        self
    }

    pub fn total(mut self, value: UserEarningsAmount) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserEarnings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`owned_accounts`](UserEarningsBuilder::owned_accounts)
    /// - [`personal`](UserEarningsBuilder::personal)
    /// - [`total`](UserEarningsBuilder::total)
    pub fn build(self) -> Result<UserEarnings, BuildError> {
        Ok(UserEarnings {
            first_earned_at: self.first_earned_at,
            owned_accounts: self
                .owned_accounts
                .ok_or_else(|| BuildError::missing_field("owned_accounts"))?,
            personal: self
                .personal
                .ok_or_else(|| BuildError::missing_field("personal"))?,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
