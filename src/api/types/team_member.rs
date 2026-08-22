pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TeamMember {
    /// The account this membership belongs to, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Custom role assigned to this member, or `null` when the member has a system role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorized_role: Option<TeamMemberAuthorizedRole>,
    /// When the member joined or the invite was sent, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The member's email address. For accepted members, `null` unless the caller holds the email read scope; for invites, the invited address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Team member ID — `ausr_` for accepted members, `ausri_` for pending invites.
    #[serde(default)]
    pub id: String,
    /// Whether this member is an agent (app-controlled account) rather than a human team member. Always `false` for invites.
    #[serde(default)]
    pub is_agent: bool,
    /// The member's role on the account. `custom` means a bespoke dashboard-managed role; the API can read but not grant it.
    pub role: TeamMemberRole,
    /// `joined` for accepted members, `pending` while the invite is pending.
    pub status: TeamMemberStatus,
    /// When the membership was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
    /// The user behind this team membership. `null` for an invite sent to an email with no Whop account yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserSummary>,
}

impl TeamMember {
    pub fn builder() -> TeamMemberBuilder {
        <TeamMemberBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeamMemberBuilder {
    account_id: Option<String>,
    authorized_role: Option<TeamMemberAuthorizedRole>,
    created_at: Option<String>,
    email: Option<String>,
    id: Option<String>,
    is_agent: Option<bool>,
    role: Option<TeamMemberRole>,
    status: Option<TeamMemberStatus>,
    updated_at: Option<String>,
    user: Option<UserSummary>,
}

impl TeamMemberBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn authorized_role(mut self, value: TeamMemberAuthorizedRole) -> Self {
        self.authorized_role = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn is_agent(mut self, value: bool) -> Self {
        self.is_agent = Some(value);
        self
    }

    pub fn role(mut self, value: TeamMemberRole) -> Self {
        self.role = Some(value);
        self
    }

    pub fn status(mut self, value: TeamMemberStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn user(mut self, value: UserSummary) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TeamMember`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](TeamMemberBuilder::account_id)
    /// - [`created_at`](TeamMemberBuilder::created_at)
    /// - [`id`](TeamMemberBuilder::id)
    /// - [`is_agent`](TeamMemberBuilder::is_agent)
    /// - [`role`](TeamMemberBuilder::role)
    /// - [`status`](TeamMemberBuilder::status)
    /// - [`updated_at`](TeamMemberBuilder::updated_at)
    pub fn build(self) -> Result<TeamMember, BuildError> {
        Ok(TeamMember {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            authorized_role: self.authorized_role,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            email: self.email,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            is_agent: self
                .is_agent
                .ok_or_else(|| BuildError::missing_field("is_agent"))?,
            role: self.role.ok_or_else(|| BuildError::missing_field("role"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            user: self.user,
        })
    }
}
