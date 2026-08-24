pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Member {
    /// What the member can reach on the account: `customer` for paying members, `admin` for team members, `no_access` once every grant has lapsed.
    pub access_level: MemberAccessLevel,
    /// The account this member belongs to, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// When the member record was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Member ID, prefixed `mber_`.
    #[serde(default)]
    pub id: String,
    /// When the member first joined the account, as an ISO 8601 timestamp.
    #[serde(default)]
    pub joined_at: String,
    /// When the member last opened the account's content, as an ISO 8601 timestamp. `null` if they never have.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<String>,
    /// `joined` while the member is part of the account, `left` after they leave.
    pub status: MemberStatus,
    /// The user behind this member. `null` when the buyer is another business rather than a person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserSummary>,
}

impl Member {
    pub fn builder() -> MemberBuilder {
        <MemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemberBuilder {
    access_level: Option<MemberAccessLevel>,
    account_id: Option<String>,
    created_at: Option<String>,
    id: Option<String>,
    joined_at: Option<String>,
    last_accessed_at: Option<String>,
    status: Option<MemberStatus>,
    user: Option<UserSummary>,
}

impl MemberBuilder {
    pub fn access_level(mut self, value: MemberAccessLevel) -> Self {
        self.access_level = Some(value);
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
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

    pub fn joined_at(mut self, value: impl Into<String>) -> Self {
        self.joined_at = Some(value.into());
        self
    }

    pub fn last_accessed_at(mut self, value: impl Into<String>) -> Self {
        self.last_accessed_at = Some(value.into());
        self
    }

    pub fn status(mut self, value: MemberStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn user(mut self, value: UserSummary) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Member`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_level`](MemberBuilder::access_level)
    /// - [`account_id`](MemberBuilder::account_id)
    /// - [`created_at`](MemberBuilder::created_at)
    /// - [`id`](MemberBuilder::id)
    /// - [`joined_at`](MemberBuilder::joined_at)
    /// - [`status`](MemberBuilder::status)
    pub fn build(self) -> Result<Member, BuildError> {
        Ok(Member {
            access_level: self
                .access_level
                .ok_or_else(|| BuildError::missing_field("access_level"))?,
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            joined_at: self
                .joined_at
                .ok_or_else(|| BuildError::missing_field("joined_at"))?,
            last_accessed_at: self.last_accessed_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            user: self.user,
        })
    }
}
