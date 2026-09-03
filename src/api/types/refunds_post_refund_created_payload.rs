pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostRefundCreatedPayload {
    /// The account ID that this webhook event is associated with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The API version for this webhook
    pub api_version: PostRefundCreatedPayloadApiVersion,
    /// The dated API version (Api-Version-Date) the payload is serialized to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_date: Option<String>,
    pub data: RefundLegacy,
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
    pub r#type: PostRefundCreatedPayloadType,
}

impl PostRefundCreatedPayload {
    pub fn builder() -> PostRefundCreatedPayloadBuilder {
        <PostRefundCreatedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostRefundCreatedPayloadBuilder {
    account_id: Option<String>,
    api_version: Option<PostRefundCreatedPayloadApiVersion>,
    api_version_date: Option<String>,
    data: Option<RefundLegacy>,
    id: Option<String>,
    previous_attributes: Option<HashMap<String, serde_json::Value>>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<PostRefundCreatedPayloadType>,
}

impl PostRefundCreatedPayloadBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn api_version(mut self, value: PostRefundCreatedPayloadApiVersion) -> Self {
        self.api_version = Some(value);
        self
    }

    pub fn api_version_date(mut self, value: impl Into<String>) -> Self {
        self.api_version_date = Some(value.into());
        self
    }

    pub fn data(mut self, value: RefundLegacy) -> Self {
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

    pub fn r#type(mut self, value: PostRefundCreatedPayloadType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostRefundCreatedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version`](PostRefundCreatedPayloadBuilder::api_version)
    /// - [`data`](PostRefundCreatedPayloadBuilder::data)
    /// - [`id`](PostRefundCreatedPayloadBuilder::id)
    /// - [`timestamp`](PostRefundCreatedPayloadBuilder::timestamp)
    /// - [`r#type`](PostRefundCreatedPayloadBuilder::r#type)
    pub fn build(self) -> Result<PostRefundCreatedPayload, BuildError> {
        Ok(PostRefundCreatedPayload {
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
