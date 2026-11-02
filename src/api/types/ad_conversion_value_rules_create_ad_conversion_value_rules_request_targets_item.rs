pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAdConversionValueRulesRequestTargetsItem {
    pub platform: CreateAdConversionValueRulesRequestTargetsItemPlatform,
    /// Campaign, ad group, or ad ID. Null for a business target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    pub scope: CreateAdConversionValueRulesRequestTargetsItemScope,
}

impl CreateAdConversionValueRulesRequestTargetsItem {
    pub fn builder() -> CreateAdConversionValueRulesRequestTargetsItemBuilder {
        <CreateAdConversionValueRulesRequestTargetsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdConversionValueRulesRequestTargetsItemBuilder {
    platform: Option<CreateAdConversionValueRulesRequestTargetsItemPlatform>,
    resource_id: Option<String>,
    scope: Option<CreateAdConversionValueRulesRequestTargetsItemScope>,
}

impl CreateAdConversionValueRulesRequestTargetsItemBuilder {
    pub fn platform(
        mut self,
        value: CreateAdConversionValueRulesRequestTargetsItemPlatform,
    ) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn scope(mut self, value: CreateAdConversionValueRulesRequestTargetsItemScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAdConversionValueRulesRequestTargetsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](CreateAdConversionValueRulesRequestTargetsItemBuilder::platform)
    /// - [`scope`](CreateAdConversionValueRulesRequestTargetsItemBuilder::scope)
    pub fn build(self) -> Result<CreateAdConversionValueRulesRequestTargetsItem, BuildError> {
        Ok(CreateAdConversionValueRulesRequestTargetsItem {
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            resource_id: self.resource_id,
            scope: self
                .scope
                .ok_or_else(|| BuildError::missing_field("scope"))?,
        })
    }
}
