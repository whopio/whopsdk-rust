pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateVerificationsResponseRequestedInformationItemErrorsItem {
    /// Stable error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Why it was rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl UpdateVerificationsResponseRequestedInformationItemErrorsItem {
    pub fn builder() -> UpdateVerificationsResponseRequestedInformationItemErrorsItemBuilder {
        <UpdateVerificationsResponseRequestedInformationItemErrorsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVerificationsResponseRequestedInformationItemErrorsItemBuilder {
    code: Option<String>,
    reason: Option<String>,
}

impl UpdateVerificationsResponseRequestedInformationItemErrorsItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateVerificationsResponseRequestedInformationItemErrorsItem`].
    pub fn build(
        self,
    ) -> Result<UpdateVerificationsResponseRequestedInformationItemErrorsItem, BuildError> {
        Ok(
            UpdateVerificationsResponseRequestedInformationItemErrorsItem {
                code: self.code,
                reason: self.reason,
            },
        )
    }
}
