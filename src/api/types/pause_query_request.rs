pub use crate::prelude::*;

/// Query parameters for pause
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PauseQueryRequest {
    /// Owning account or internal. Required when id is a flag key; optional for an expt_ ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl PauseQueryRequest {
    pub fn builder() -> PauseQueryRequestBuilder {
        <PauseQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PauseQueryRequestBuilder {
    account_id: Option<String>,
}

impl PauseQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PauseQueryRequest`].
    pub fn build(self) -> Result<PauseQueryRequest, BuildError> {
        Ok(PauseQueryRequest {
            account_id: self.account_id,
        })
    }
}
