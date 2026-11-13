pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Member {
    /// What the member can reach on the account: `customer` for paying members, `admin` for team members, `no_access` once every grant has lapsed.
    pub access_level: MemberAccessLevel,
    /// The account this member belongs to, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// When the member record was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The name to show for this member: the user's name, or their username when they have not set one. The buying business's name when `user` is `null`.
    #[serde(default)]
    pub display_name: String,
    /// The user's gross income across Whop, in USD, refreshed periodically. `null` when the user hides their earnings or the buyer is another business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earnings_usd: Option<UserEarningsAmount>,
    /// The user's email address. `null` without `member:email:read` on the account, when the buyer is another business, or when the user has no assigned email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Member ID, prefixed `mber_`.
    #[serde(default)]
    pub id: String,
    /// When the member first joined the account, as an ISO 8601 timestamp.
    #[serde(default)]
    pub joined_at: String,
    /// When the member last opened the account's content, as an ISO 8601 timestamp. `null` if they never have.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_accessed_at: Option<String>,
    /// What happened at `last_activity_at`. `object` says which kind of activity it is. `null` without `support_chat:read` on the account, or when `last_activity_at` is `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity: Option<MemberLastActivity>,
    /// When the member and the account last interacted, as an ISO 8601 timestamp. For example, a message in the member's support chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_activity_at: Option<String>,
    /// Your own key-value data stored on this member. `null` when none is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The member's latest lifecycle state on the account, derived from their memberships, such as `paid_subscriber`, `canceling`, or `churned`. `null` until it is first computed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_action: Option<MemberMostRecentAction>,
    /// When `most_recent_action` took effect, as an ISO 8601 timestamp. `null` when `most_recent_action` is `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_action_at: Option<String>,
    /// The name the account gave this customer with `PATCH /members/:id`. Invoices show it as the customer name on every invoice for this member, in place of the user's own name. `null` when the account has not set one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The member's phone number: the most recent number collected (or verified) at checkout, otherwise their account number when they have shared one with this seller. `null` without `member:phone:read` on the account, or when there is neither.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    /// `joined` while the member is part of the account, `left` after they leave.
    pub status: MemberStatus,
    /// The member's current token balance for this account, computed from token transactions.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub token_balance: f64,
    /// `unsubscribed` once the member opts out of the account's marketing communications, `null` while they're subscribed. Only the member can change it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unsubscribe_status: Option<MemberUnsubscribeStatus>,
    /// How much the member has spent on the account's products, in USD.
    #[serde(default)]
    pub usd_total_spent: Money,
    /// The user behind this member. `null` when the buyer is another business rather than a person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<MemberUser>,
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
    display_name: Option<String>,
    earnings_usd: Option<UserEarningsAmount>,
    email: Option<String>,
    id: Option<String>,
    joined_at: Option<String>,
    last_accessed_at: Option<String>,
    last_activity: Option<MemberLastActivity>,
    last_activity_at: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    most_recent_action: Option<MemberMostRecentAction>,
    most_recent_action_at: Option<String>,
    name: Option<String>,
    phone_number: Option<String>,
    status: Option<MemberStatus>,
    token_balance: Option<f64>,
    unsubscribe_status: Option<MemberUnsubscribeStatus>,
    usd_total_spent: Option<Money>,
    user: Option<MemberUser>,
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

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn earnings_usd(mut self, value: UserEarningsAmount) -> Self {
        self.earnings_usd = Some(value);
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

    pub fn joined_at(mut self, value: impl Into<String>) -> Self {
        self.joined_at = Some(value.into());
        self
    }

    pub fn last_accessed_at(mut self, value: impl Into<String>) -> Self {
        self.last_accessed_at = Some(value.into());
        self
    }

    pub fn last_activity(mut self, value: MemberLastActivity) -> Self {
        self.last_activity = Some(value);
        self
    }

    pub fn last_activity_at(mut self, value: impl Into<String>) -> Self {
        self.last_activity_at = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn most_recent_action(mut self, value: MemberMostRecentAction) -> Self {
        self.most_recent_action = Some(value);
        self
    }

    pub fn most_recent_action_at(mut self, value: impl Into<String>) -> Self {
        self.most_recent_action_at = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn phone_number(mut self, value: impl Into<String>) -> Self {
        self.phone_number = Some(value.into());
        self
    }

    pub fn status(mut self, value: MemberStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn token_balance(mut self, value: f64) -> Self {
        self.token_balance = Some(value);
        self
    }

    pub fn unsubscribe_status(mut self, value: MemberUnsubscribeStatus) -> Self {
        self.unsubscribe_status = Some(value);
        self
    }

    pub fn usd_total_spent(mut self, value: Money) -> Self {
        self.usd_total_spent = Some(value);
        self
    }

    pub fn user(mut self, value: MemberUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Member`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_level`](MemberBuilder::access_level)
    /// - [`account_id`](MemberBuilder::account_id)
    /// - [`created_at`](MemberBuilder::created_at)
    /// - [`display_name`](MemberBuilder::display_name)
    /// - [`id`](MemberBuilder::id)
    /// - [`joined_at`](MemberBuilder::joined_at)
    /// - [`status`](MemberBuilder::status)
    /// - [`token_balance`](MemberBuilder::token_balance)
    /// - [`usd_total_spent`](MemberBuilder::usd_total_spent)
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
            display_name: self
                .display_name
                .ok_or_else(|| BuildError::missing_field("display_name"))?,
            earnings_usd: self.earnings_usd,
            email: self.email,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            joined_at: self
                .joined_at
                .ok_or_else(|| BuildError::missing_field("joined_at"))?,
            last_accessed_at: self.last_accessed_at,
            last_activity: self.last_activity,
            last_activity_at: self.last_activity_at,
            metadata: self.metadata,
            most_recent_action: self.most_recent_action,
            most_recent_action_at: self.most_recent_action_at,
            name: self.name,
            phone_number: self.phone_number,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            token_balance: self
                .token_balance
                .ok_or_else(|| BuildError::missing_field("token_balance"))?,
            unsubscribe_status: self.unsubscribe_status,
            usd_total_spent: self
                .usd_total_spent
                .ok_or_else(|| BuildError::missing_field("usd_total_spent"))?,
            user: self.user,
        })
    }
}
