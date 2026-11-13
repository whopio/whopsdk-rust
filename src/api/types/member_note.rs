pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MemberNote {
    /// The user who wrote the note, or `null` when it was added by an API key or automatically by Whop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<UserSummary>,
    /// The note text.
    #[serde(default)]
    pub content: String,
    /// When the note was added, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Member note ID, prefixed `note_`.
    #[serde(default)]
    pub id: String,
    /// The member the note is about, prefixed `mber_`.
    #[serde(default)]
    pub member_id: String,
    /// The customer the note is about, prefixed `user_`.
    #[serde(default)]
    pub user_id: String,
}

impl MemberNote {
    pub fn builder() -> MemberNoteBuilder {
        <MemberNoteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemberNoteBuilder {
    author: Option<UserSummary>,
    content: Option<String>,
    created_at: Option<String>,
    id: Option<String>,
    member_id: Option<String>,
    user_id: Option<String>,
}

impl MemberNoteBuilder {
    pub fn author(mut self, value: UserSummary) -> Self {
        self.author = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
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

    pub fn member_id(mut self, value: impl Into<String>) -> Self {
        self.member_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MemberNote`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](MemberNoteBuilder::content)
    /// - [`created_at`](MemberNoteBuilder::created_at)
    /// - [`id`](MemberNoteBuilder::id)
    /// - [`member_id`](MemberNoteBuilder::member_id)
    /// - [`user_id`](MemberNoteBuilder::user_id)
    pub fn build(self) -> Result<MemberNote, BuildError> {
        Ok(MemberNote {
            author: self.author,
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            member_id: self
                .member_id
                .ok_or_else(|| BuildError::missing_field("member_id"))?,
            user_id: self
                .user_id
                .ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}
