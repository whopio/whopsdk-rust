pub use crate::prelude::*;

/// The user that made this payment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentListItemUser {
    /// The user's email address. Requires the member:email:read permission to access. Null if not authorized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The unique identifier for the user.
    #[serde(default)]
    pub id: String,
    /// The user's display name shown on their public profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's unique username shown on their public profile.
    #[serde(default)]
    pub username: String,
}

impl PaymentListItemUser {
    pub fn builder() -> PaymentListItemUserBuilder {
        <PaymentListItemUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentListItemUserBuilder {
    email: Option<String>,
    id: Option<String>,
    name: Option<String>,
    username: Option<String>,
}

impl PaymentListItemUserBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentListItemUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PaymentListItemUserBuilder::id)
    /// - [`username`](PaymentListItemUserBuilder::username)
    pub fn build(self) -> Result<PaymentListItemUser, BuildError> {
        Ok(PaymentListItemUser {
            email: self.email,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
        })
    }
}
