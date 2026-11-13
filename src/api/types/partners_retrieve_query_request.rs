pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PartnersRetrieveQueryRequest {
    /// Return `payout_rates` for this business, prefixed `biz_`, instead of the caller's own: the business's rate rows, else the platform defaults. Requires company:basic:read on that business, which every team member holds. The rest of the profile stays the caller's.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl PartnersRetrieveQueryRequest {
    pub fn builder() -> PartnersRetrieveQueryRequestBuilder {
        <PartnersRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnersRetrieveQueryRequestBuilder {
    account_id: Option<String>,
}

impl PartnersRetrieveQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PartnersRetrieveQueryRequest`].
    pub fn build(self) -> Result<PartnersRetrieveQueryRequest, BuildError> {
        Ok(PartnersRetrieveQueryRequest {
            account_id: self.account_id,
        })
    }
}
