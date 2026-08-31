pub use crate::prelude::*;

/// Billing details collected with the method. `email` is always required; cards additionally require `name` and an address with `line1` and `country`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateConfirmationTokensRequestBillingDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl CreateConfirmationTokensRequestBillingDetails {
    pub fn builder() -> CreateConfirmationTokensRequestBillingDetailsBuilder {
        <CreateConfirmationTokensRequestBillingDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateConfirmationTokensRequestBillingDetailsBuilder {
    address: Option<HashMap<String, serde_json::Value>>,
    email: Option<String>,
    name: Option<String>,
}

impl CreateConfirmationTokensRequestBillingDetailsBuilder {
    pub fn address(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.address = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateConfirmationTokensRequestBillingDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`email`](CreateConfirmationTokensRequestBillingDetailsBuilder::email)
    pub fn build(self) -> Result<CreateConfirmationTokensRequestBillingDetails, BuildError> {
        Ok(CreateConfirmationTokensRequestBillingDetails {
            address: self.address,
            email: self
                .email
                .ok_or_else(|| BuildError::missing_field("email"))?,
            name: self.name,
        })
    }
}
