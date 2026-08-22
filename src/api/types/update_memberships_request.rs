pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateMembershipsRequest {
    /// `true` cancels at the end of the current billing period (the customer keeps access until then); `false` reverses a pending cancellation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at_period_end: Option<bool>,
    /// Key-value pairs to merge into the membership's metadata. Pass an empty object to clear it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl UpdateMembershipsRequest {
    pub fn builder() -> UpdateMembershipsRequestBuilder {
        <UpdateMembershipsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateMembershipsRequestBuilder {
    cancel_at_period_end: Option<bool>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl UpdateMembershipsRequestBuilder {
    pub fn cancel_at_period_end(mut self, value: bool) -> Self {
        self.cancel_at_period_end = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateMembershipsRequest`].
    pub fn build(self) -> Result<UpdateMembershipsRequest, BuildError> {
        Ok(UpdateMembershipsRequest {
            cancel_at_period_end: self.cancel_at_period_end,
            metadata: self.metadata,
        })
    }
}
