pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PostSetupIntentSucceededPayload {
    /// The account ID that this webhook event is associated with
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// The API version for this webhook
    pub api_version: PostSetupIntentSucceededPayloadApiVersion,
    /// The dated API version (Api-Version-Date) the payload is serialized to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version_date: Option<String>,
    pub data: SetupIntent,
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
    pub r#type: PostSetupIntentSucceededPayloadType,
}

impl PostSetupIntentSucceededPayload {
    pub fn builder() -> PostSetupIntentSucceededPayloadBuilder {
        <PostSetupIntentSucceededPayloadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostSetupIntentSucceededPayloadBuilder {
    account_id: Option<String>,
    api_version: Option<PostSetupIntentSucceededPayloadApiVersion>,
    api_version_date: Option<String>,
    data: Option<SetupIntent>,
    id: Option<String>,
    previous_attributes: Option<HashMap<String, serde_json::Value>>,
    timestamp: Option<DateTime<FixedOffset>>,
    r#type: Option<PostSetupIntentSucceededPayloadType>,
}

impl PostSetupIntentSucceededPayloadBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn api_version(mut self, value: PostSetupIntentSucceededPayloadApiVersion) -> Self {
        self.api_version = Some(value);
        self
    }

    pub fn api_version_date(mut self, value: impl Into<String>) -> Self {
        self.api_version_date = Some(value.into());
        self
    }

    pub fn data(mut self, value: SetupIntent) -> Self {
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

    pub fn r#type(mut self, value: PostSetupIntentSucceededPayloadType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PostSetupIntentSucceededPayload`].
    /// This method will fail if any of the following fields are not set:
    /// - [`api_version`](PostSetupIntentSucceededPayloadBuilder::api_version)
    /// - [`data`](PostSetupIntentSucceededPayloadBuilder::data)
    /// - [`id`](PostSetupIntentSucceededPayloadBuilder::id)
    /// - [`timestamp`](PostSetupIntentSucceededPayloadBuilder::timestamp)
    /// - [`r#type`](PostSetupIntentSucceededPayloadBuilder::r#type)
    pub fn build(self) -> Result<PostSetupIntentSucceededPayload, BuildError> {
        Ok(PostSetupIntentSucceededPayload {
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
