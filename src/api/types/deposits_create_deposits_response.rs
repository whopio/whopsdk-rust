pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateDepositsResponse {
    /// Account ID of the destination owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Requested deposit amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// URL of the hosted deposit page. Only present for business destinations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_url: Option<String>,
    /// Available deposit methods for destination.
    #[serde(default)]
    pub methods: CreateDepositsResponseMethods,
    pub object: CreateDepositsResponseObject,
}

impl CreateDepositsResponse {
    pub fn builder() -> CreateDepositsResponseBuilder {
        <CreateDepositsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDepositsResponseBuilder {
    account_id: Option<String>,
    amount: Option<String>,
    hosted_url: Option<String>,
    methods: Option<CreateDepositsResponseMethods>,
    object: Option<CreateDepositsResponseObject>,
}

impl CreateDepositsResponseBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn amount(mut self, value: impl Into<String>) -> Self {
        self.amount = Some(value.into());
        self
    }

    pub fn hosted_url(mut self, value: impl Into<String>) -> Self {
        self.hosted_url = Some(value.into());
        self
    }

    pub fn methods(mut self, value: CreateDepositsResponseMethods) -> Self {
        self.methods = Some(value);
        self
    }

    pub fn object(mut self, value: CreateDepositsResponseObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDepositsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`methods`](CreateDepositsResponseBuilder::methods)
    /// - [`object`](CreateDepositsResponseBuilder::object)
    pub fn build(self) -> Result<CreateDepositsResponse, BuildError> {
        Ok(CreateDepositsResponse {
            account_id: self.account_id,
            amount: self.amount,
            hosted_url: self.hosted_url,
            methods: self
                .methods
                .ok_or_else(|| BuildError::missing_field("methods"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}
