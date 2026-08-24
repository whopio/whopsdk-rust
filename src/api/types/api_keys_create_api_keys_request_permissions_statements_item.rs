pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateApiKeysRequestPermissionsStatementsItem {
    /// Permission actions covered by this statement, for example `company:basic:read`.
    #[serde(default)]
    pub actions: Vec<String>,
    /// Whether the actions are granted (`true`) or denied (`false`).
    #[serde(default)]
    pub grant: bool,
    /// Resource identifiers the statement applies to, for example `biz_xxx` or `biz_xxx|pass_*`. Defaults to the key's owning resource when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<String>>,
}

impl CreateApiKeysRequestPermissionsStatementsItem {
    pub fn builder() -> CreateApiKeysRequestPermissionsStatementsItemBuilder {
        <CreateApiKeysRequestPermissionsStatementsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApiKeysRequestPermissionsStatementsItemBuilder {
    actions: Option<Vec<String>>,
    grant: Option<bool>,
    resources: Option<Vec<String>>,
}

impl CreateApiKeysRequestPermissionsStatementsItemBuilder {
    pub fn actions(mut self, value: Vec<String>) -> Self {
        self.actions = Some(value);
        self
    }

    pub fn grant(mut self, value: bool) -> Self {
        self.grant = Some(value);
        self
    }

    pub fn resources(mut self, value: Vec<String>) -> Self {
        self.resources = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateApiKeysRequestPermissionsStatementsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actions`](CreateApiKeysRequestPermissionsStatementsItemBuilder::actions)
    /// - [`grant`](CreateApiKeysRequestPermissionsStatementsItemBuilder::grant)
    pub fn build(self) -> Result<CreateApiKeysRequestPermissionsStatementsItem, BuildError> {
        Ok(CreateApiKeysRequestPermissionsStatementsItem {
            actions: self
                .actions
                .ok_or_else(|| BuildError::missing_field("actions"))?,
            grant: self
                .grant
                .ok_or_else(|| BuildError::missing_field("grant"))?,
            resources: self.resources,
        })
    }
}
