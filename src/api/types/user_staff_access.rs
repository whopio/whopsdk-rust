pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserStaffAccess {
    /// Whether the user holds the admin staff role with a valid second factor.
    #[serde(default)]
    pub admin: bool,
    /// Whether the user can open Whop-internal investigation tooling right now: a qualifying staff role plus their investigation toggle switched on.
    #[serde(default)]
    pub investigation_access: bool,
    /// Whether the user holds the manager staff role with a valid second factor.
    #[serde(default)]
    pub manager: bool,
    /// Whether the user holds the support staff role with a valid second factor.
    #[serde(default)]
    pub support: bool,
}

impl UserStaffAccess {
    pub fn builder() -> UserStaffAccessBuilder {
        <UserStaffAccessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserStaffAccessBuilder {
    admin: Option<bool>,
    investigation_access: Option<bool>,
    manager: Option<bool>,
    support: Option<bool>,
}

impl UserStaffAccessBuilder {
    pub fn admin(mut self, value: bool) -> Self {
        self.admin = Some(value);
        self
    }

    pub fn investigation_access(mut self, value: bool) -> Self {
        self.investigation_access = Some(value);
        self
    }

    pub fn manager(mut self, value: bool) -> Self {
        self.manager = Some(value);
        self
    }

    pub fn support(mut self, value: bool) -> Self {
        self.support = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserStaffAccess`].
    /// This method will fail if any of the following fields are not set:
    /// - [`admin`](UserStaffAccessBuilder::admin)
    /// - [`investigation_access`](UserStaffAccessBuilder::investigation_access)
    /// - [`manager`](UserStaffAccessBuilder::manager)
    /// - [`support`](UserStaffAccessBuilder::support)
    pub fn build(self) -> Result<UserStaffAccess, BuildError> {
        Ok(UserStaffAccess {
            admin: self
                .admin
                .ok_or_else(|| BuildError::missing_field("admin"))?,
            investigation_access: self
                .investigation_access
                .ok_or_else(|| BuildError::missing_field("investigation_access"))?,
            manager: self
                .manager
                .ok_or_else(|| BuildError::missing_field("manager"))?,
            support: self
                .support
                .ok_or_else(|| BuildError::missing_field("support"))?,
        })
    }
}
