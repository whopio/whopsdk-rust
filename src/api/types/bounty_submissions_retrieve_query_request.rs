pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BountySubmissionsRetrieveQueryRequest {
    /// Read the submission as this account (`biz_` tag), scoping the lookup to its bounties rather than the caller's own work. Requires read access to the account. Without it the lookup covers only what the credential owns — the submissions the caller authored plus those on bounties they posted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl BountySubmissionsRetrieveQueryRequest {
    pub fn builder() -> BountySubmissionsRetrieveQueryRequestBuilder {
        <BountySubmissionsRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BountySubmissionsRetrieveQueryRequestBuilder {
    account_id: Option<String>,
}

impl BountySubmissionsRetrieveQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BountySubmissionsRetrieveQueryRequest`].
    pub fn build(self) -> Result<BountySubmissionsRetrieveQueryRequest, BuildError> {
        Ok(BountySubmissionsRetrieveQueryRequest {
            account_id: self.account_id,
        })
    }
}
