pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostCardCreatedPayload {
    /// The account ID that this webhook event is associated with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The API version for this webhook
    pub api_version: PostCardCreatedPayloadApiVersion,
    /// The dated API version (Api-Version-Date) the payload is serialized to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_date: Option<String>,
    pub data: PostCardCreatedPayloadData,
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
    pub r#type: PostCardCreatedPayloadType,
}

impl PostCardCreatedPayload {
    pub fn builder() -> PostCardCreatedPayloadBuilder {
        <PostCardCreatedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostCardCreatedPayloadBuilder {
    account_id: Option<String>,
    api_version: Option<PostCardCreatedPayloadApiVersion>,
    api_version_date: Option<String>,
    data: Option<PostCardCreatedPayloadData>,
    id: Option<String>,
    previous_attributes: Option<HashMap<String, serde_json::Value>>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<PostCardCreatedPayloadType>,
}

impl PostCardCreatedPayloadBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn api_version(mut self, value: PostCardCreatedPayloadApiVersion) -> Self {
        self.api_version = Some(value);
        self
    }

    pub fn api_version_date(mut self, value: impl Into<String>) -> Self {
        self.api_version_date = Some(value.into());
        self
    }

    pub fn data(mut self, value: PostCardCreatedPayloadData) -> Self {
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

    pub fn r#type(mut self, value: PostCardCreatedPayloadType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostCardCreatedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version`](PostCardCreatedPayloadBuilder::api_version)
    /// - [`data`](PostCardCreatedPayloadBuilder::data)
    /// - [`id`](PostCardCreatedPayloadBuilder::id)
    /// - [`timestamp`](PostCardCreatedPayloadBuilder::timestamp)
    /// - [`r#type`](PostCardCreatedPayloadBuilder::r#type)
    pub fn build(self) -> Result<PostCardCreatedPayload, BuildError> {
        Ok(PostCardCreatedPayload {
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
