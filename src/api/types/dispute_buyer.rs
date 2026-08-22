pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeBuyer {
    /// The customer's email address. Requires the `member:email:read` scope; `null` without it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The customer's member row on the account, prefixed `mem_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// The customer's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The customer's user ID, prefixed `user_`. `null` for a guest checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The customer's Whop username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl DisputeBuyer {
    pub fn builder() -> DisputeBuyerBuilder {
        <DisputeBuyerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeBuyerBuilder {
    email: Option<String>,
    member_id: Option<String>,
    name: Option<String>,
    user_id: Option<String>,
    username: Option<String>,
}

impl DisputeBuyerBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeBuyer`].
    pub fn build(self) -> Result<DisputeBuyer, BuildError> {
        Ok(DisputeBuyer {
            email: self.email,
            member_id: self.member_id,
            name: self.name,
            user_id: self.user_id,
            username: self.username,
        })
    }
}
