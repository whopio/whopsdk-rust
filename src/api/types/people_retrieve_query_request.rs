pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PeopleRetrieveQueryRequest {
    /// Account ID, prefixed `biz_`. Optional for account API keys; required for credentials that can access multiple accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl PeopleRetrieveQueryRequest {
    pub fn builder() -> PeopleRetrieveQueryRequestBuilder {
        <PeopleRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeopleRetrieveQueryRequestBuilder {
    account_id: Option<String>,
}

impl PeopleRetrieveQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PeopleRetrieveQueryRequest`].
    pub fn build(self) -> Result<PeopleRetrieveQueryRequest, BuildError> {
        Ok(PeopleRetrieveQueryRequest {
            account_id: self.account_id,
        })
    }
}
