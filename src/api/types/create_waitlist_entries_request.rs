pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateWaitlistEntriesRequest {
    /// Answers to the plan's checkout questions. Every required question must be answered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_responses: Option<Vec<CreateWaitlistEntriesRequestCustomFieldResponsesItem>>,
    /// Custom key-value pairs to store on the signup. Max 50 keys, 100 chars per key, 500 chars per string value. Ignored when the request returns an existing signup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The free waitlist plan to join, prefixed `plan_`.
    #[serde(default)]
    pub plan_id: String,
}

impl CreateWaitlistEntriesRequest {
    pub fn builder() -> CreateWaitlistEntriesRequestBuilder {
        <CreateWaitlistEntriesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWaitlistEntriesRequestBuilder {
    custom_field_responses: Option<Vec<CreateWaitlistEntriesRequestCustomFieldResponsesItem>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    plan_id: Option<String>,
}

impl CreateWaitlistEntriesRequestBuilder {
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

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateWaitlistEntriesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`plan_id`](CreateWaitlistEntriesRequestBuilder::plan_id)
    pub fn build(self) -> Result<CreateWaitlistEntriesRequest, BuildError> {
        Ok(CreateWaitlistEntriesRequest {
            custom_field_responses: self.custom_field_responses,
            metadata: self.metadata,
            plan_id: self
                .plan_id
                .ok_or_else(|| BuildError::missing_field("plan_id"))?,
        })
    }
}
