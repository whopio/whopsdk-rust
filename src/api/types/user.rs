pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct User {
    /// The user's balance: personal cash + other holdings + deposits still in flight, plus account balances for accounts they own. Computed only on the self view (retrieved with the reserved id `me`) for callers with balance-read scope; `null` otherwise, or when `include_balance=false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<UserBalance>,
    /// The user's cumulative wallet balance over time (USD `{ t, v }` points plus last/min/max), for the balance chart. Opt in with `include_balance_history=true` when retrieving yourself with the reserved id `me`; populated only for callers with balance-read scope and `null` otherwise. A user with no wallet activity returns an empty series.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_history: Option<UserBalanceHistory>,
    #[serde(default)]
    pub balances: Vec<AccountBalanceToken>,
    /// The user's profile banner wrapper. `null` when the user has no banner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub banner: Option<UserBanner>,
    /// The user's biography
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bio: Option<String>,
    /// Where the user's personal card application stands. Populated only on the self view (retrieved with the reserved id `me`) for callers with balance-read scope; `null` otherwise, or when the user has never applied for a card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cards: Option<AccountCards>,
    /// The user's city, derived from their IP address. `null` when unknown or the user turned off location sharing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The user's country as an ISO 3166-1 alpha-2 code, derived from their IP address. `null` when unknown or the user turned off location sharing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// When the user was created, as an ISO 8601 timestamp
    #[serde(default)]
    pub created_at: String,
    /// The user's date of birth as an ISO 8601 date, like `1998-04-23`. Populated only on the self view (retrieved with the reserved id `me`) for callers with email-read scope; `null` otherwise, or when the user has not set one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// The user's gross USD income over time, including a Partner commission breakdown. Populated only on single-user self reads for callers with balance-read scope; `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earnings_usd: Option<UserEarnings>,
    /// The user's email address. Populated only on the self view (retrieved with the reserved id `me`) for callers with email-read scope; `null` otherwise, or while the account has no confirmed email yet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// The user's Whop Levels progress, counted across every account they own. Populated only on the self view (retrieved with the reserved id `me`) for callers with balance-read scope; `null` otherwise, and when none of their accounts are in Whop Levels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<OwnerLevel>,
    /// The user's display name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Avatar wrapper. Its `url` is always present, and is a generated placeholder when the user set no picture.
    #[serde(default)]
    pub profile_picture: UserProfilePicture,
    #[serde(default)]
    pub social_links: Vec<UserSocialLink>,
    /// The user's state or region, derived from their IP address. `null` when unknown or the user turned off location sharing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Total USD value of the user's personal holdings in `balances`, across those with known exchange rates. Computed only on the self view (retrieved with the reserved id `me`) for callers with balance-read scope; `null` otherwise, when `include_balance=false`, or when the balance source is unavailable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_usd: Option<String>,
    /// The trading account address, a snapshot of its margin and open positions, and its WebSocket subscriptions. Opt in with `include_trading=true` when retrieving `me`. `null` otherwise, without trading permission, or without an Ethereum wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trading: Option<TradingAccount>,
    /// The user's unique username
    #[serde(default)]
    pub username: String,
    /// Identity verification status for the user's `individual` (KYC) and `business` (KYB) profiles. Each is `null` until created, otherwise a `status` of `not_started`, `pending`, `approved`, or `rejected`.
    #[serde(default)]
    pub verification: HashMap<String, serde_json::Value>,
    /// When the user became an enrolled Whop Partner, as an ISO 8601 timestamp. `null` if never enrolled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whop_partner_enabled_at: Option<String>,
    /// Number of accounts the user referred to Whop as a Whop Partner that have processed more than $1 in volume attributed to the user. Populated only when retrieving a single user who is a Verified Whop Partner; `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whop_partner_onboarded_accounts_count: Option<i64>,
    /// When the user became a Verified Whop Partner, as an ISO 8601 timestamp. `null` for users who are not Verified Whop Partners, including partners who left the program and suspended users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whop_partner_verified_at: Option<String>,
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
    balances: Option<Vec<AccountBalanceToken>>,
    banner: Option<UserBanner>,
    bio: Option<String>,
    cards: Option<AccountCards>,
    city: Option<String>,
    country: Option<String>,
    created_at: Option<String>,
    date_of_birth: Option<String>,
    earnings_usd: Option<UserEarnings>,
    email: Option<String>,
    id: Option<String>,
    level: Option<OwnerLevel>,
    name: Option<String>,
    profile_picture: Option<UserProfilePicture>,
    social_links: Option<Vec<UserSocialLink>>,
    state: Option<String>,
    total_usd: Option<String>,
    trading: Option<TradingAccount>,
    username: Option<String>,
    verification: Option<HashMap<String, serde_json::Value>>,
    whop_partner_enabled_at: Option<String>,
    whop_partner_onboarded_accounts_count: Option<i64>,
    whop_partner_verified_at: Option<String>,
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

    pub fn balances(mut self, value: Vec<AccountBalanceToken>) -> Self {
        self.balances = Some(value);
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

    pub fn cards(mut self, value: AccountCards) -> Self {
        self.cards = Some(value);
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn date_of_birth(mut self, value: impl Into<String>) -> Self {
        self.date_of_birth = Some(value.into());
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

    pub fn level(mut self, value: OwnerLevel) -> Self {
        self.level = Some(value);
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

    pub fn social_links(mut self, value: Vec<UserSocialLink>) -> Self {
        self.social_links = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn total_usd(mut self, value: impl Into<String>) -> Self {
        self.total_usd = Some(value.into());
        self
    }

    pub fn trading(mut self, value: TradingAccount) -> Self {
        self.trading = Some(value);
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

    pub fn whop_partner_onboarded_accounts_count(mut self, value: i64) -> Self {
        self.whop_partner_onboarded_accounts_count = Some(value);
        self
    }

    pub fn whop_partner_verified_at(mut self, value: impl Into<String>) -> Self {
        self.whop_partner_verified_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`User`].
    /// This method will fail if any of the following fields are not set:
    /// - [`balances`](UserBuilder::balances)
    /// - [`created_at`](UserBuilder::created_at)
    /// - [`id`](UserBuilder::id)
    /// - [`profile_picture`](UserBuilder::profile_picture)
    /// - [`social_links`](UserBuilder::social_links)
    /// - [`username`](UserBuilder::username)
    /// - [`verification`](UserBuilder::verification)
    pub fn build(self) -> Result<User, BuildError> {
        Ok(User {
            balance: self.balance,
            balance_history: self.balance_history,
            balances: self
                .balances
                .ok_or_else(|| BuildError::missing_field("balances"))?,
            banner: self.banner,
            bio: self.bio,
            cards: self.cards,
            city: self.city,
            country: self.country,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            date_of_birth: self.date_of_birth,
            earnings_usd: self.earnings_usd,
            email: self.email,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            level: self.level,
            name: self.name,
            profile_picture: self
                .profile_picture
                .ok_or_else(|| BuildError::missing_field("profile_picture"))?,
            social_links: self
                .social_links
                .ok_or_else(|| BuildError::missing_field("social_links"))?,
            state: self.state,
            total_usd: self.total_usd,
            trading: self.trading,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
            verification: self
                .verification
                .ok_or_else(|| BuildError::missing_field("verification"))?,
            whop_partner_enabled_at: self.whop_partner_enabled_at,
            whop_partner_onboarded_accounts_count: self.whop_partner_onboarded_accounts_count,
            whop_partner_verified_at: self.whop_partner_verified_at,
        })
    }
}
