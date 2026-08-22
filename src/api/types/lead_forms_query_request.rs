pub use crate::prelude::*;

/// Query parameters for leadForms
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeadFormsQueryRequest {
    /// The Account (a biz_ identifier) the social account is connected to.
    #[serde(default)]
    pub account_id: String,
}

impl LeadFormsQueryRequest {
    pub fn builder() -> LeadFormsQueryRequestBuilder {
        <LeadFormsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeadFormsQueryRequestBuilder {
    account_id: Option<String>,
}

impl LeadFormsQueryRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeadFormsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](LeadFormsQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<LeadFormsQueryRequest, BuildError> {
        Ok(LeadFormsQueryRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
        })
    }
}
