pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperimentsRetrieveQueryRequest {
    /// Owning account or internal. Required when id is a flag key; optional for an expt_ ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl ExperimentsRetrieveQueryRequest {
    pub fn builder() -> ExperimentsRetrieveQueryRequestBuilder {
        <ExperimentsRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperimentsRetrieveQueryRequestBuilder {
    account_id: Option<String>,
}

impl ExperimentsRetrieveQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExperimentsRetrieveQueryRequest`].
    pub fn build(self) -> Result<ExperimentsRetrieveQueryRequest, BuildError> {
        Ok(ExperimentsRetrieveQueryRequest {
            account_id: self.account_id,
        })
    }
}
