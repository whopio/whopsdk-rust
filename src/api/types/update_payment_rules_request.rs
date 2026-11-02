pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdatePaymentRulesRequest {
    /// Custom string-to-string values for your integration. Maximum 50 keys, 40 characters per key, 500 characters per value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// A name for this rule. Up to 255 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl UpdatePaymentRulesRequest {
    pub fn builder() -> UpdatePaymentRulesRequestBuilder {
        <UpdatePaymentRulesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePaymentRulesRequestBuilder {
    metadata: Option<HashMap<String, String>>,
    name: Option<String>,
}

impl UpdatePaymentRulesRequestBuilder {
    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdatePaymentRulesRequest`].
    pub fn build(self) -> Result<UpdatePaymentRulesRequest, BuildError> {
        Ok(UpdatePaymentRulesRequest {
            metadata: self.metadata,
            name: self.name,
        })
    }
}
