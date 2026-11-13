pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateChannelsRequest {
    /// Owning account ID, prefixed `biz_`. Accepted for all channel kinds. Required for community and support channels unless supplied by the credential's account context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Customer ID, prefixed `user_`. Support channels only; requires support management permission. Omit to open your own support conversation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_user_id: Option<String>,
    /// Channel icon as a file reference, for community channels and private channels with two or more participant IDs. Image files only. Group icons can be up to 5 MB. For community channels it sets the experience logo. Used only when a channel is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<CreateChannelsRequestIcon>,
    /// Channel kind. `private` is a one-to-one or group conversation, `community` creates a Chat experience on the account, and `support` opens a conversation between the account and a customer.
    pub kind: CreateChannelsRequestKind,
    /// Channel display name, accepted for all channel kinds. Used only when a channel is created. For community channels, names the new Chat experience; defaults to the Chat app's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether app notifications are on for any channel kind. Webhooks fire either way. Defaults to `true`. Used only when a channel is created; change an existing channel with `PATCH /channels/:id`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notifications_enabled: Option<bool>,
    /// Other participant IDs, prefixed `user_`, excluding the sender. Required for private channels and accepted only for them. One ID creates a one-to-one conversation; more create a group.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub participant_user_ids: Option<Vec<String>>,
    /// Product ID, prefixed `prod_`, to attach the new community channel to. Accepted only for community channels. Must belong to the owning account and requires the `experience:attach` permission on it. Omit to leave the channel unattached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Sender ID, prefixed `user_`. Private channels only. Required for account API keys; omit when authenticating as a user. Requires channel management permission on the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_user_id: Option<String>,
}

impl CreateChannelsRequest {
    pub fn builder() -> CreateChannelsRequestBuilder {
        <CreateChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateChannelsRequestBuilder {
    account_id: Option<String>,
    customer_user_id: Option<String>,
    icon: Option<CreateChannelsRequestIcon>,
    kind: Option<CreateChannelsRequestKind>,
    name: Option<String>,
    notifications_enabled: Option<bool>,
    participant_user_ids: Option<Vec<String>>,
    product_id: Option<String>,
    sender_user_id: Option<String>,
}

impl CreateChannelsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn customer_user_id(mut self, value: impl Into<String>) -> Self {
        self.customer_user_id = Some(value.into());
        self
    }

    pub fn icon(mut self, value: CreateChannelsRequestIcon) -> Self {
        self.icon = Some(value);
        self
    }

    pub fn kind(mut self, value: CreateChannelsRequestKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn notifications_enabled(mut self, value: bool) -> Self {
        self.notifications_enabled = Some(value);
        self
    }

    pub fn participant_user_ids(mut self, value: Vec<String>) -> Self {
        self.participant_user_ids = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn sender_user_id(mut self, value: impl Into<String>) -> Self {
        self.sender_user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateChannelsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](CreateChannelsRequestBuilder::kind)
    pub fn build(self) -> Result<CreateChannelsRequest, BuildError> {
        Ok(CreateChannelsRequest {
            account_id: self.account_id,
            customer_user_id: self.customer_user_id,
            icon: self.icon,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            name: self.name,
            notifications_enabled: self.notifications_enabled,
            participant_user_ids: self.participant_user_ids,
            product_id: self.product_id,
            sender_user_id: self.sender_user_id,
        })
    }
}
