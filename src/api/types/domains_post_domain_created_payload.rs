pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostDomainCreatedPayload {
    /// The account ID that this webhook event is associated with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The API version for this webhook
    pub api_version: PostDomainCreatedPayloadApiVersion,
    /// The dated API version (Api-Version-Date) the payload is serialized to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_date: Option<String>,
    #[serde(default)]
    pub data: Domain,
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
    pub r#type: PostDomainCreatedPayloadType,
}

impl PostDomainCreatedPayload {
    pub fn builder() -> PostDomainCreatedPayloadBuilder {
        <PostDomainCreatedPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostDomainCreatedPayloadBuilder {
    account_id: Option<String>,
    api_version: Option<PostDomainCreatedPayloadApiVersion>,
    api_version_date: Option<String>,
    data: Option<Domain>,
    id: Option<String>,
    previous_attributes: Option<HashMap<String, serde_json::Value>>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<PostDomainCreatedPayloadType>,
}

impl PostDomainCreatedPayloadBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn api_version(mut self, value: PostDomainCreatedPayloadApiVersion) -> Self {
        self.api_version = Some(value);
        self
    }

    pub fn api_version_date(mut self, value: impl Into<String>) -> Self {
        self.api_version_date = Some(value.into());
        self
    }

    pub fn data(mut self, value: Domain) -> Self {
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

    pub fn r#type(mut self, value: PostDomainCreatedPayloadType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostDomainCreatedPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version`](PostDomainCreatedPayloadBuilder::api_version)
    /// - [`data`](PostDomainCreatedPayloadBuilder::data)
    /// - [`id`](PostDomainCreatedPayloadBuilder::id)
    /// - [`timestamp`](PostDomainCreatedPayloadBuilder::timestamp)
    /// - [`r#type`](PostDomainCreatedPayloadBuilder::r#type)
    pub fn build(self) -> Result<PostDomainCreatedPayload, BuildError> {
        Ok(PostDomainCreatedPayload {
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
