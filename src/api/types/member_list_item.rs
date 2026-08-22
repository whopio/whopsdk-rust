pub use crate::prelude::*;

/// A member represents a user's relationship with a company on Whop, including their access level, status, and spending history.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemberListItem {
    /// The member's content access level. `admin` means their team role grants administrative content access, `customer` means they hold a valid product membership, and `no_access` means they cannot access company content.
    pub access_level: AccessLevel,
    /// The member's token balance for this company. Computed live from the ledger, not from a cache.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub company_token_balance: f64,
    /// The datetime the company member was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The unique identifier for the company member.
    #[serde(default)]
    pub id: String,
    /// When the member joined the company
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub joined_at: DateTime<FixedOffset>,
    /// The most recent action the member has taken.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub most_recent_action: Option<MemberMostRecentActions>,
    /// The time for the most recent action, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub most_recent_action_at: Option<DateTime<FixedOffset>>,
    /// The phone number for the member, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// The status of the member
    pub status: MemberStatuses,
    /// The datetime the company member was last updated.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub updated_at: DateTime<FixedOffset>,
    /// How much money this customer has spent on the company's products and plans
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub usd_total_spent: f64,
    /// The user for this member, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<MemberListItemUser>,
}

impl MemberListItem {
    pub fn builder() -> MemberListItemBuilder {
        <MemberListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MemberListItemBuilder {
    access_level: Option<AccessLevel>,
    company_token_balance: Option<f64>,
    created_at: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    joined_at: Option<DateTime<FixedOffset>>,
    most_recent_action: Option<MemberMostRecentActions>,
    most_recent_action_at: Option<DateTime<FixedOffset>>,
    phone: Option<String>,
    status: Option<MemberStatuses>,
    updated_at: Option<DateTime<FixedOffset>>,
    usd_total_spent: Option<f64>,
    user: Option<MemberListItemUser>,
}

impl MemberListItemBuilder {
    pub fn access_level(mut self, value: AccessLevel) -> Self {
        self.access_level = Some(value);
        self
    }

    pub fn company_token_balance(mut self, value: f64) -> Self {
        self.company_token_balance = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn joined_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.joined_at = Some(value);
        self
    }

    pub fn most_recent_action(mut self, value: MemberMostRecentActions) -> Self {
        self.most_recent_action = Some(value);
        self
    }

    pub fn most_recent_action_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.most_recent_action_at = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn status(mut self, value: MemberStatuses) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn usd_total_spent(mut self, value: f64) -> Self {
        self.usd_total_spent = Some(value);
        self
    }

    pub fn user(mut self, value: MemberListItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MemberListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_level`](MemberListItemBuilder::access_level)
    /// - [`company_token_balance`](MemberListItemBuilder::company_token_balance)
    /// - [`created_at`](MemberListItemBuilder::created_at)
    /// - [`id`](MemberListItemBuilder::id)
    /// - [`joined_at`](MemberListItemBuilder::joined_at)
    /// - [`status`](MemberListItemBuilder::status)
    /// - [`updated_at`](MemberListItemBuilder::updated_at)
    /// - [`usd_total_spent`](MemberListItemBuilder::usd_total_spent)
    pub fn build(self) -> Result<MemberListItem, BuildError> {
        Ok(MemberListItem {
            access_level: self
                .access_level
                .ok_or_else(|| BuildError::missing_field("access_level"))?,
            company_token_balance: self
                .company_token_balance
                .ok_or_else(|| BuildError::missing_field("company_token_balance"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            joined_at: self
                .joined_at
                .ok_or_else(|| BuildError::missing_field("joined_at"))?,
            most_recent_action: self.most_recent_action,
            most_recent_action_at: self.most_recent_action_at,
            phone: self.phone,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            usd_total_spent: self
                .usd_total_spent
                .ok_or_else(|| BuildError::missing_field("usd_total_spent"))?,
            user: self.user,
        })
    }
}
