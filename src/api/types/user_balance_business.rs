pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserBalanceBusiness {
    /// The account's total balance in USD.
    #[serde(default)]
    pub balance_usd: String,
    /// The account ID, which looks like biz_*************.
    #[serde(default)]
    pub id: String,
    /// The account's logo URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// The account's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UserBalanceBusiness {
    pub fn builder() -> UserBalanceBusinessBuilder {
        <UserBalanceBusinessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBalanceBusinessBuilder {
    balance_usd: Option<String>,
    id: Option<String>,
    logo_url: Option<String>,
    name: Option<String>,
}

impl UserBalanceBusinessBuilder {
    pub fn balance_usd(mut self, value: impl Into<String>) -> Self {
        self.balance_usd = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserBalanceBusiness`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balance_usd`](UserBalanceBusinessBuilder::balance_usd)
    /// - [`id`](UserBalanceBusinessBuilder::id)
    pub fn build(self) -> Result<UserBalanceBusiness, BuildError> {
        Ok(UserBalanceBusiness {
            balance_usd: self
                .balance_usd
                .ok_or_else(|| BuildError::missing_field("balance_usd"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            logo_url: self.logo_url,
            name: self.name,
        })
    }
}
