pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolutionBuyer {
    /// The customer's email address. Requires the `member:email:read` scope; `null` without it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The customer's member row on the account, prefixed `mem_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// The customer's display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The customer's user ID, prefixed `user_`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The customer's Whop username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl ResolutionBuyer {
    pub fn builder() -> ResolutionBuyerBuilder {
        <ResolutionBuyerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionBuyerBuilder {
    email: Option<String>,
    member_id: Option<String>,
    name: Option<String>,
    user_id: Option<String>,
    username: Option<String>,
}

impl ResolutionBuyerBuilder {
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

    /// Consumes the builder and constructs a [`ResolutionBuyer`].
    pub fn build(self) -> Result<ResolutionBuyer, BuildError> {
        Ok(ResolutionBuyer {
            email: self.email,
            member_id: self.member_id,
            name: self.name,
            user_id: self.user_id,
            username: self.username,
        })
    }
}
