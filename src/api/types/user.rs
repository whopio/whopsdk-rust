pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct User {
    /// The user's balance: personal cash + crypto + in-flight treasury deposits, plus account balances for accounts they own. Computed only on the self view (retrieved with the reserved id `me`) for callers with balance-read scope; `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<UserBalance>,
    /// The user's cumulative wallet balance over time (USD `{ t, v }` points plus last/min/max), for the balance chart. Opt in with `include_balance_history=true` when retrieving yourself with the reserved id `me`; populated only for callers with balance-read scope and `null` otherwise. A user with no wallet activity returns an empty series.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_history: Option<UserBalanceHistory>,
    /// The user's profile banner wrapper. `null` when the user has no banner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<UserBanner>,
    /// The user's biography
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// When the user was created, as an ISO 8601 timestamp
    #[serde(default)]
    pub created_at: String,
    /// The user's gross USD income over time. Populated only on single-user self reads for callers with balance-read scope; `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earnings_usd: Option<UserEarnings>,
    /// The user's email address. Populated only on the self view (retrieved with the reserved id `me`) for callers with email-read scope; `null` otherwise, or while the account has no confirmed email yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// The user's display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Avatar wrapper; its `url` is always present, using a generated placeholder when the user set no picture.
    #[serde(default)]
    pub profile_picture: UserProfilePicture,
    #[serde(default)]
    pub social_accounts: Vec<SocialAccount>,
    /// Whop staff access flags. Populated only on the self view (retrieved with the reserved id `me`) for callers with staff-read scope; `null` there for every user who is not Whop staff, and always `null` elsewhere.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staff: Option<UserStaffAccess>,
    /// The user's unique username
    #[serde(default)]
    pub username: String,
    /// Identity verification status for the user's `individual` (KYC) and `business` (KYB) profiles. Each is `null` until created, otherwise a `status` of `not_started`, `pending`, `approved`, or `rejected`.
    #[serde(default)]
    pub verification: HashMap<String, serde_json::Value>,
    /// When the user became an enrolled Whop Partner, as an ISO 8601 timestamp. `null` if never enrolled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whop_partner_enabled_at: Option<String>,
}

impl User {
    pub fn builder() -> UserBuilder {
        <UserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserBuilder {
    balance: Option<UserBalance>,
    balance_history: Option<UserBalanceHistory>,
    banner: Option<UserBanner>,
    bio: Option<String>,
    created_at: Option<String>,
    earnings_usd: Option<UserEarnings>,
    email: Option<String>,
    id: Option<String>,
    name: Option<String>,
    profile_picture: Option<UserProfilePicture>,
    social_accounts: Option<Vec<SocialAccount>>,
    staff: Option<UserStaffAccess>,
    username: Option<String>,
    verification: Option<HashMap<String, serde_json::Value>>,
    whop_partner_enabled_at: Option<String>,
}

impl UserBuilder {
    pub fn balance(mut self, value: UserBalance) -> Self {
        self.balance = Some(value);
        self
    }

    pub fn balance_history(mut self, value: UserBalanceHistory) -> Self {
        self.balance_history = Some(value);
        self
    }

    pub fn banner(mut self, value: UserBanner) -> Self {
        self.banner = Some(value);
        self
    }

    pub fn bio(mut self, value: impl Into<String>) -> Self {
        self.bio = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn earnings_usd(mut self, value: UserEarnings) -> Self {
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

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn profile_picture(mut self, value: UserProfilePicture) -> Self {
        self.profile_picture = Some(value);
        self
    }

    pub fn social_accounts(mut self, value: Vec<SocialAccount>) -> Self {
        self.social_accounts = Some(value);
        self
    }

    pub fn staff(mut self, value: UserStaffAccess) -> Self {
        self.staff = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn verification(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.verification = Some(value);
        self
    }

    pub fn whop_partner_enabled_at(mut self, value: impl Into<String>) -> Self {
        self.whop_partner_enabled_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`User`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](UserBuilder::created_at)
    /// - [`id`](UserBuilder::id)
    /// - [`profile_picture`](UserBuilder::profile_picture)
    /// - [`social_accounts`](UserBuilder::social_accounts)
    /// - [`username`](UserBuilder::username)
    /// - [`verification`](UserBuilder::verification)
    pub fn build(self) -> Result<User, BuildError> {
        Ok(User {
            balance: self.balance,
            balance_history: self.balance_history,
            banner: self.banner,
            bio: self.bio,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            earnings_usd: self.earnings_usd,
            email: self.email,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            profile_picture: self
                .profile_picture
                .ok_or_else(|| BuildError::missing_field("profile_picture"))?,
            social_accounts: self
                .social_accounts
                .ok_or_else(|| BuildError::missing_field("social_accounts"))?,
            staff: self.staff,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
            verification: self
                .verification
                .ok_or_else(|| BuildError::missing_field("verification"))?,
            whop_partner_enabled_at: self.whop_partner_enabled_at,
        })
    }
}
