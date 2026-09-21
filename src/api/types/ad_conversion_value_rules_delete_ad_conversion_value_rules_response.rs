pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAdConversionValueRulesResponse {
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub id: String,
}

impl DeleteAdConversionValueRulesResponse {
    pub fn builder() -> DeleteAdConversionValueRulesResponseBuilder {
        <DeleteAdConversionValueRulesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAdConversionValueRulesResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteAdConversionValueRulesResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteAdConversionValueRulesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteAdConversionValueRulesResponseBuilder::deleted)
    /// - [`id`](DeleteAdConversionValueRulesResponseBuilder::id)
    pub fn build(self) -> Result<DeleteAdConversionValueRulesResponse, BuildError> {
        Ok(DeleteAdConversionValueRulesResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
