pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateDmChannelsRequest {
    /// The unique identifier of the company to scope this DM channel to. When set, the channel is visible only within that company context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// A custom display name for the DM channel. For example, 'Project Discussion'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// Whether Whop app notifications are enabled for this direct message channel. Webhooks still fire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
    /// The list of user identifiers to include in the DM channel. Each entry can be an email, username, or user ID (e.g. 'user_xxxxx').
    #[serde(default)]
    pub with_user_ids: Vec<String>,
}

impl CreateDmChannelsRequest {
    pub fn builder() -> CreateDmChannelsRequestBuilder {
        <CreateDmChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDmChannelsRequestBuilder {
    company_id: Option<String>,
    custom_name: Option<String>,
    notifications_enabled: Option<bool>,
    with_user_ids: Option<Vec<String>>,
}

impl CreateDmChannelsRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn custom_name(mut self, value: impl Into<String>) -> Self {
        self.custom_name = Some(value.into());
        self
    }

    pub fn notifications_enabled(mut self, value: bool) -> Self {
        self.notifications_enabled = Some(value);
        self
    }

    pub fn with_user_ids(mut self, value: Vec<String>) -> Self {
        self.with_user_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDmChannelsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`with_user_ids`](CreateDmChannelsRequestBuilder::with_user_ids)
    pub fn build(self) -> Result<CreateDmChannelsRequest, BuildError> {
        Ok(CreateDmChannelsRequest {
            company_id: self.company_id,
            custom_name: self.custom_name,
            notifications_enabled: self.notifications_enabled,
            with_user_ids: self
                .with_user_ids
                .ok_or_else(|| BuildError::missing_field("with_user_ids"))?,
        })
    }
}
