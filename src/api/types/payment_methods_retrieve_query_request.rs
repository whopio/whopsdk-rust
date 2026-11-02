pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodsRetrieveQueryRequest {
    /// The unique identifier of the member. Provide either this or account_id, not both. Omit both to address your own saved payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// The unique identifier of the company. Provide either this or member_id, not both. Omit both to address your own saved payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
}

impl PaymentMethodsRetrieveQueryRequest {
    pub fn builder() -> PaymentMethodsRetrieveQueryRequestBuilder {
        <PaymentMethodsRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodsRetrieveQueryRequestBuilder {
    member_id: Option<String>,
    account_id: Option<String>,
}

impl PaymentMethodsRetrieveQueryRequestBuilder {
    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodsRetrieveQueryRequest`].
    pub fn build(self) -> Result<PaymentMethodsRetrieveQueryRequest, BuildError> {
        Ok(PaymentMethodsRetrieveQueryRequest {
            member_id: self.member_id,
            account_id: self.account_id,
        })
    }
}
