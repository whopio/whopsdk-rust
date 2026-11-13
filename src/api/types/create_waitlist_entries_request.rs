pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateWaitlistEntriesRequest {
    /// The account to join on behalf of, prefixed `biz_`. Leave it out to join as yourself. Required with an account credential, which passes its own account and records the account's owner as the user who joined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_account_id: Option<String>,
    /// Answers to the variant's checkout questions. Every required question must be answered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_responses: Option<Vec<CreateWaitlistEntriesRequestCustomFieldResponsesItem>>,
    /// Custom key-value pairs to store on the signup. Max 50 keys, 100 chars per key, 500 chars per string value. Ignored when the request returns an existing signup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The free waitlist variant to join, prefixed `plan_`.
    #[serde(default)]
    pub variant_id: String,
}

impl CreateWaitlistEntriesRequest {
    pub fn builder() -> CreateWaitlistEntriesRequestBuilder {
        <CreateWaitlistEntriesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWaitlistEntriesRequestBuilder {
    buyer_account_id: Option<String>,
    custom_field_responses: Option<Vec<CreateWaitlistEntriesRequestCustomFieldResponsesItem>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    variant_id: Option<String>,
}

impl CreateWaitlistEntriesRequestBuilder {
    pub fn buyer_account_id(mut self, value: impl Into<String>) -> Self {
        self.buyer_account_id = Some(value.into());
        self
    }

    pub fn custom_field_responses(
        mut self,
        value: Vec<CreateWaitlistEntriesRequestCustomFieldResponsesItem>,
    ) -> Self {
        self.custom_field_responses = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWaitlistEntriesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`variant_id`](CreateWaitlistEntriesRequestBuilder::variant_id)
    pub fn build(self) -> Result<CreateWaitlistEntriesRequest, BuildError> {
        Ok(CreateWaitlistEntriesRequest {
            buyer_account_id: self.buyer_account_id,
            custom_field_responses: self.custom_field_responses,
            metadata: self.metadata,
            variant_id: self
                .variant_id
                .ok_or_else(|| BuildError::missing_field("variant_id"))?,
        })
    }
}
