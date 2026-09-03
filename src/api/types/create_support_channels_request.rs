pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSupportChannelsRequest {
    /// The unique identifier of the company to create the support channel in.
    #[serde(default)]
    pub account_id: String,
    /// Optional custom display name for the support channel.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// Whether Whop app notifications are enabled for this support channel. Webhooks still fire.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
    /// The user ID (e.g. 'user_xxxxx') or username of the customer to open a support channel for.
    #[serde(default)]
    pub user_id: String,
}

impl CreateSupportChannelsRequest {
    pub fn builder() -> CreateSupportChannelsRequestBuilder {
        <CreateSupportChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSupportChannelsRequestBuilder {
    account_id: Option<String>,
    custom_name: Option<String>,
    notifications_enabled: Option<bool>,
    user_id: Option<String>,
}

impl CreateSupportChannelsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
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

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSupportChannelsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateSupportChannelsRequestBuilder::account_id)
    /// - [`user_id`](CreateSupportChannelsRequestBuilder::user_id)
    pub fn build(self) -> Result<CreateSupportChannelsRequest, BuildError> {
        Ok(CreateSupportChannelsRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            custom_name: self.custom_name,
            notifications_enabled: self.notifications_enabled,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
