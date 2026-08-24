pub use crate::prelude::*;

/// The permissions policy for the API key: explicit permission statements, or a system role to inherit from. Statements without a `resources` array default to the owning account (Account API keys) or every key-addressable resource (App API keys).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateApiKeysRequestPermissions {
    /// Explicit permission statements. Required unless `system_role` is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statements: Option<Vec<CreateApiKeysRequestPermissionsStatementsItem>>,
    /// A system role to inherit permissions from. Only Account API keys can use a system role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_role: Option<CreateApiKeysRequestPermissionsSystemRole>,
}

impl CreateApiKeysRequestPermissions {
    pub fn builder() -> CreateApiKeysRequestPermissionsBuilder {
        <CreateApiKeysRequestPermissionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApiKeysRequestPermissionsBuilder {
    statements: Option<Vec<CreateApiKeysRequestPermissionsStatementsItem>>,
    system_role: Option<CreateApiKeysRequestPermissionsSystemRole>,
}

impl CreateApiKeysRequestPermissionsBuilder {
    pub fn statements(mut self, value: Vec<CreateApiKeysRequestPermissionsStatementsItem>) -> Self {
        self.statements = Some(value);
        self
    }

    pub fn system_role(mut self, value: CreateApiKeysRequestPermissionsSystemRole) -> Self {
        self.system_role = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateApiKeysRequestPermissions`].
    pub fn build(self) -> Result<CreateApiKeysRequestPermissions, BuildError> {
        Ok(CreateApiKeysRequestPermissions {
            statements: self.statements,
            system_role: self.system_role,
        })
    }
}
