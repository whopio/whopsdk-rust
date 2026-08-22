pub use crate::prelude::*;

/// A user who belongs to a company's team with access determined by their assigned role.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AuthorizedUser {
    /// The company this authorized user has access to.
    #[serde(default)]
    pub company: AuthorizedUserCompany,
    /// The unique identifier for the authorized user.
    #[serde(default)]
    pub id: String,
    /// The permission role assigned to this authorized user within the company.
    pub role: AuthorizedUserRoles,
    /// The user account linked to this authorized user record.
    #[serde(default)]
    pub user: AuthorizedUserUser,
}

impl AuthorizedUser {
    pub fn builder() -> AuthorizedUserBuilder {
        <AuthorizedUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthorizedUserBuilder {
    company: Option<AuthorizedUserCompany>,
    id: Option<String>,
    role: Option<AuthorizedUserRoles>,
    user: Option<AuthorizedUserUser>,
}

impl AuthorizedUserBuilder {
    pub fn company(mut self, value: AuthorizedUserCompany) -> Self {
        self.company = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn role(mut self, value: AuthorizedUserRoles) -> Self {
        self.role = Some(value);
        self
    }

    pub fn user(mut self, value: AuthorizedUserUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthorizedUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company`](AuthorizedUserBuilder::company)
    /// - [`id`](AuthorizedUserBuilder::id)
    /// - [`role`](AuthorizedUserBuilder::role)
    /// - [`user`](AuthorizedUserBuilder::user)
    pub fn build(self) -> Result<AuthorizedUser, BuildError> {
        Ok(AuthorizedUser {
            company: self
                .company
                .ok_or_else(|| BuildError::missing_field("company"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
