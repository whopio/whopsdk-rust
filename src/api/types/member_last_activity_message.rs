pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MemberLastActivityMessage {
    /// Who sent the message: the member, or someone on the account's team. `null` when the sender's account no longer exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserSummary>,
    /// The support chat the message was sent in.
    #[serde(default)]
    pub channel_id: String,
    /// When the message was sent, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The message ID.
    #[serde(default)]
    pub id: String,
    /// Always `message`.
    pub object: MemberLastActivityMessageObject,
    /// The message text without formatting. `null` when the message has no text, such as an attachment on its own.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_content: Option<String>,
}

impl MemberLastActivityMessage {
    pub fn builder() -> MemberLastActivityMessageBuilder {
        <MemberLastActivityMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemberLastActivityMessageBuilder {
    author: Option<UserSummary>,
    channel_id: Option<String>,
    created_at: Option<String>,
    id: Option<String>,
    object: Option<MemberLastActivityMessageObject>,
    plain_content: Option<String>,
}

impl MemberLastActivityMessageBuilder {
    pub fn author(mut self, value: UserSummary) -> Self {
        self.author = Some(value);
        self
    }

    pub fn channel_id(mut self, value: impl Into<String>) -> Self {
        self.channel_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: MemberLastActivityMessageObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn plain_content(mut self, value: impl Into<String>) -> Self {
        self.plain_content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MemberLastActivityMessage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](MemberLastActivityMessageBuilder::channel_id)
    /// - [`created_at`](MemberLastActivityMessageBuilder::created_at)
    /// - [`id`](MemberLastActivityMessageBuilder::id)
    /// - [`object`](MemberLastActivityMessageBuilder::object)
    pub fn build(self) -> Result<MemberLastActivityMessage, BuildError> {
        Ok(MemberLastActivityMessage {
            author: self.author,
            channel_id: self
                .channel_id
                .ok_or_else(|| BuildError::missing_field("channel_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            plain_content: self.plain_content,
        })
    }
}
