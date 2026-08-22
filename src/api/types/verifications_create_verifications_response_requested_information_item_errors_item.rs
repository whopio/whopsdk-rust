pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateVerificationsResponseRequestedInformationItemErrorsItem {
    /// Stable error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Why it was rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl CreateVerificationsResponseRequestedInformationItemErrorsItem {
    pub fn builder() -> CreateVerificationsResponseRequestedInformationItemErrorsItemBuilder {
        <CreateVerificationsResponseRequestedInformationItemErrorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVerificationsResponseRequestedInformationItemErrorsItemBuilder {
    code: Option<String>,
    reason: Option<String>,
}

impl CreateVerificationsResponseRequestedInformationItemErrorsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateVerificationsResponseRequestedInformationItemErrorsItem`].
    pub fn build(
        self,
    ) -> Result<CreateVerificationsResponseRequestedInformationItemErrorsItem, BuildError> {
        Ok(
            CreateVerificationsResponseRequestedInformationItemErrorsItem {
                code: self.code,
                reason: self.reason,
            },
        )
    }
}
