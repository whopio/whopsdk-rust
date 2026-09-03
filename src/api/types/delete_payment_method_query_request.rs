pub use crate::prelude::*;

/// Query parameters for deletePaymentMethod
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeletePaymentMethodQueryRequest {
    /// The unique identifier of the member. Provide either this or company_id, not both. Omit both to address your own saved payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// The unique identifier of the company. Provide either this or member_id, not both. Omit both to address your own saved payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl DeletePaymentMethodQueryRequest {
    pub fn builder() -> DeletePaymentMethodQueryRequestBuilder {
        <DeletePaymentMethodQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePaymentMethodQueryRequestBuilder {
    member_id: Option<String>,
    account_id: Option<String>,
}

impl DeletePaymentMethodQueryRequestBuilder {
    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeletePaymentMethodQueryRequest`].
    pub fn build(self) -> Result<DeletePaymentMethodQueryRequest, BuildError> {
        Ok(DeletePaymentMethodQueryRequest {
            member_id: self.member_id,
            account_id: self.account_id,
        })
    }
}
