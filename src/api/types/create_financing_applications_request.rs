pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateFinancingApplicationsRequest {
    /// The account applying, prefixed `biz_`. Defaults to the account behind an account credential; required with a user credential.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl CreateFinancingApplicationsRequest {
    pub fn builder() -> CreateFinancingApplicationsRequestBuilder {
        <CreateFinancingApplicationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateFinancingApplicationsRequestBuilder {
    account_id: Option<String>,
}

impl CreateFinancingApplicationsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateFinancingApplicationsRequest`].
    pub fn build(self) -> Result<CreateFinancingApplicationsRequest, BuildError> {
        Ok(CreateFinancingApplicationsRequest {
            account_id: self.account_id,
        })
    }
}
