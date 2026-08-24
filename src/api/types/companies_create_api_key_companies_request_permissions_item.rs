pub use crate::prelude::*;

/// Input for a single permissions statement
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateApiKeyCompaniesRequestPermissionsItem {
    /// Actions covered by this statement
    #[serde(default)]
    pub actions: Vec<String>,
    /// Whether the actions are granted or denied
    #[serde(default)]
    pub grant: bool,
    /// Resource identifiers. Can look like 'biz_xxxx' or 'biz_xxx|pass_*|exp_xxx' or 'biz_xxx|app_xxx' or 'biz_xxx|pass_xxx|exp_xxx' or 'biz_xxx|pass_xxx' or 'biz_xxx|pass_*'
    #[serde(default)]
    pub resources: Vec<String>,
}

impl CreateApiKeyCompaniesRequestPermissionsItem {
    pub fn builder() -> CreateApiKeyCompaniesRequestPermissionsItemBuilder {
        <CreateApiKeyCompaniesRequestPermissionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApiKeyCompaniesRequestPermissionsItemBuilder {
    actions: Option<Vec<String>>,
    grant: Option<bool>,
    resources: Option<Vec<String>>,
}

impl CreateApiKeyCompaniesRequestPermissionsItemBuilder {
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

    /// Consumes the builder and constructs a [`CreateApiKeyCompaniesRequestPermissionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`actions`](CreateApiKeyCompaniesRequestPermissionsItemBuilder::actions)
    /// - [`grant`](CreateApiKeyCompaniesRequestPermissionsItemBuilder::grant)
    /// - [`resources`](CreateApiKeyCompaniesRequestPermissionsItemBuilder::resources)
    pub fn build(self) -> Result<CreateApiKeyCompaniesRequestPermissionsItem, BuildError> {
        Ok(CreateApiKeyCompaniesRequestPermissionsItem {
            actions: self
                .actions
                .ok_or_else(|| BuildError::missing_field("actions"))?,
            grant: self
                .grant
                .ok_or_else(|| BuildError::missing_field("grant"))?,
            resources: self
                .resources
                .ok_or_else(|| BuildError::missing_field("resources"))?,
        })
    }
}
