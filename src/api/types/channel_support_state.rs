pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ChannelSupportState {
    #[serde(default)]
    pub awaiting_reply: bool,
    #[serde(default)]
    pub customer_user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_message_by: Option<ChannelSupportStateLastMessageBy>,
    /// When the support conversation was resolved, as an ISO 8601 timestamp. Null while unresolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_at: Option<String>,
    pub status: ChannelSupportStateStatus,
}

impl ChannelSupportState {
    pub fn builder() -> ChannelSupportStateBuilder {
        <ChannelSupportStateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChannelSupportStateBuilder {
    awaiting_reply: Option<bool>,
    customer_user_id: Option<String>,
    last_message_by: Option<ChannelSupportStateLastMessageBy>,
    resolved_at: Option<String>,
    status: Option<ChannelSupportStateStatus>,
}

impl ChannelSupportStateBuilder {
    pub fn awaiting_reply(mut self, value: bool) -> Self {
        self.awaiting_reply = Some(value);
        self
    }

    pub fn customer_user_id(mut self, value: impl Into<String>) -> Self {
        self.customer_user_id = Some(value.into());
        self
    }

    pub fn last_message_by(mut self, value: ChannelSupportStateLastMessageBy) -> Self {
        self.last_message_by = Some(value);
        self
    }

    pub fn resolved_at(mut self, value: impl Into<String>) -> Self {
        self.resolved_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: ChannelSupportStateStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChannelSupportState`].
    /// This method will fail if any of the following fields are not set:
    /// - [`awaiting_reply`](ChannelSupportStateBuilder::awaiting_reply)
    /// - [`customer_user_id`](ChannelSupportStateBuilder::customer_user_id)
    /// - [`status`](ChannelSupportStateBuilder::status)
    pub fn build(self) -> Result<ChannelSupportState, BuildError> {
        Ok(ChannelSupportState {
            awaiting_reply: self
                .awaiting_reply
                .ok_or_else(|| BuildError::missing_field("awaiting_reply"))?,
            customer_user_id: self
                .customer_user_id
                .ok_or_else(|| BuildError::missing_field("customer_user_id"))?,
            last_message_by: self.last_message_by,
            resolved_at: self.resolved_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
