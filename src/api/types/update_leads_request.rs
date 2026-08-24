pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateLeadsRequest {
    /// A JSON object of custom metadata to set on the lead, replacing any existing metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The updated referral URL for the lead, such as 'https://example.com/landing'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
}

impl UpdateLeadsRequest {
    pub fn builder() -> UpdateLeadsRequestBuilder {
        <UpdateLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateLeadsRequestBuilder {
    metadata: Option<HashMap<String, serde_json::Value>>,
    referrer: Option<String>,
}

impl UpdateLeadsRequestBuilder {
    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn referrer(mut self, value: impl Into<String>) -> Self {
        self.referrer = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateLeadsRequest`].
    pub fn build(self) -> Result<UpdateLeadsRequest, BuildError> {
        Ok(UpdateLeadsRequest {
            metadata: self.metadata,
            referrer: self.referrer,
        })
    }
}
