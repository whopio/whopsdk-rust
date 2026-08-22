pub use crate::prelude::*;

/// The permissions policy for the API key: explicit permission statements, or a system role to inherit from. Statements without a `resources` array default to the owning account (Account API keys) or every key-addressable resource (App API keys).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateApiKeysRequestPermissions {
    /// Explicit permission statements. Required unless `system_role` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<UpdateApiKeysRequestPermissionsStatementsItem>>,
    /// A system role to inherit permissions from. Only Account API keys can use a system role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_role: Option<UpdateApiKeysRequestPermissionsSystemRole>,
}

impl UpdateApiKeysRequestPermissions {
    pub fn builder() -> UpdateApiKeysRequestPermissionsBuilder {
        <UpdateApiKeysRequestPermissionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateApiKeysRequestPermissionsBuilder {
    statements: Option<Vec<UpdateApiKeysRequestPermissionsStatementsItem>>,
    system_role: Option<UpdateApiKeysRequestPermissionsSystemRole>,
}

impl UpdateApiKeysRequestPermissionsBuilder {
    pub fn statements(mut self, value: Vec<UpdateApiKeysRequestPermissionsStatementsItem>) -> Self {
        self.statements = Some(value);
        self
    }

    pub fn system_role(mut self, value: UpdateApiKeysRequestPermissionsSystemRole) -> Self {
        self.system_role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateApiKeysRequestPermissions`].
    pub fn build(self) -> Result<UpdateApiKeysRequestPermissions, BuildError> {
        Ok(UpdateApiKeysRequestPermissions {
            statements: self.statements,
            system_role: self.system_role,
        })
    }
}
