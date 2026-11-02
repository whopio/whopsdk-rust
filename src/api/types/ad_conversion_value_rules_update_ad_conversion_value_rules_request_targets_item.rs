pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateAdConversionValueRulesRequestTargetsItem {
    pub platform: UpdateAdConversionValueRulesRequestTargetsItemPlatform,
    /// Campaign, ad group, or ad ID. Null for a business target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    pub scope: UpdateAdConversionValueRulesRequestTargetsItemScope,
}

impl UpdateAdConversionValueRulesRequestTargetsItem {
    pub fn builder() -> UpdateAdConversionValueRulesRequestTargetsItemBuilder {
        <UpdateAdConversionValueRulesRequestTargetsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdConversionValueRulesRequestTargetsItemBuilder {
    platform: Option<UpdateAdConversionValueRulesRequestTargetsItemPlatform>,
    resource_id: Option<String>,
    scope: Option<UpdateAdConversionValueRulesRequestTargetsItemScope>,
}

impl UpdateAdConversionValueRulesRequestTargetsItemBuilder {
    pub fn platform(
        mut self,
        value: UpdateAdConversionValueRulesRequestTargetsItemPlatform,
    ) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn scope(mut self, value: UpdateAdConversionValueRulesRequestTargetsItemScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdConversionValueRulesRequestTargetsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`platform`](UpdateAdConversionValueRulesRequestTargetsItemBuilder::platform)
    /// - [`scope`](UpdateAdConversionValueRulesRequestTargetsItemBuilder::scope)
    pub fn build(self) -> Result<UpdateAdConversionValueRulesRequestTargetsItem, BuildError> {
        Ok(UpdateAdConversionValueRulesRequestTargetsItem {
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
