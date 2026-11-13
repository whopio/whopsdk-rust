pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostAccountFinancingApprovedPayload {
    /// The account ID that this webhook event is associated with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The API version for this webhook
    pub api_version: PostAccountFinancingApprovedPayloadApiVersion,
    /// The dated API version (Api-Version-Date) the payload is serialized to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_date: Option<String>,
    #[serde(default)]
    pub data: Account,
    /// A unique ID for every single webhook request
    #[serde(default)]
    pub id: String,
    /// For some `.updated` events, the old values of the payload fields that changed, keyed by field name. Omitted when no capture is available for the event
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_attributes: Option<HashMap<String, serde_json::Value>>,
    /// The timestamp in ISO 8601 format that the webhook was sent at on the server
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub timestamp: DateTime<FixedOffset>,
    /// The webhook event type
    pub r#type: PostAccountFinancingApprovedPayloadType,
}

impl PostAccountFinancingApprovedPayload {
    pub fn builder() -> PostAccountFinancingApprovedPayloadBuilder {
        <PostAccountFinancingApprovedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostAccountFinancingApprovedPayloadBuilder {
    account_id: Option<String>,
    api_version: Option<PostAccountFinancingApprovedPayloadApiVersion>,
    api_version_date: Option<String>,
    data: Option<Account>,
    id: Option<String>,
    previous_attributes: Option<HashMap<String, serde_json::Value>>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<PostAccountFinancingApprovedPayloadType>,
}

impl PostAccountFinancingApprovedPayloadBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn api_version(mut self, value: PostAccountFinancingApprovedPayloadApiVersion) -> Self {
        self.api_version = Some(value);
        self
    }

    pub fn api_version_date(mut self, value: impl Into<String>) -> Self {
        self.api_version_date = Some(value.into());
        self
    }

    pub fn data(mut self, value: Account) -> Self {
        self.data = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn previous_attributes(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.previous_attributes = Some(value);
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    pub fn r#type(mut self, value: PostAccountFinancingApprovedPayloadType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostAccountFinancingApprovedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version`](PostAccountFinancingApprovedPayloadBuilder::api_version)
    /// - [`data`](PostAccountFinancingApprovedPayloadBuilder::data)
    /// - [`id`](PostAccountFinancingApprovedPayloadBuilder::id)
    /// - [`timestamp`](PostAccountFinancingApprovedPayloadBuilder::timestamp)
    /// - [`r#type`](PostAccountFinancingApprovedPayloadBuilder::r#type)
    pub fn build(self) -> Result<PostAccountFinancingApprovedPayload, BuildError> {
        Ok(PostAccountFinancingApprovedPayload {
            account_id: self.account_id,
            api_version: self
                .api_version
                .ok_or_else(|| BuildError::missing_field("api_version"))?,
            api_version_date: self.api_version_date,
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            previous_attributes: self.previous_attributes,
            timestamp: self
                .timestamp
                .ok_or_else(|| BuildError::missing_field("timestamp"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
