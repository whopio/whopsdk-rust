pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AccountPartner {
    /// Email address for contacting the partner. Null when the partner has not added their own email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// User ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// Display name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Avatar wrapper. Its `url` is always present, and is a generated placeholder when the user set no picture.
    #[serde(default)]
    pub profile_picture: UserProfilePicture,
    /// Business the partner referred this account on behalf of, when the referral belongs to their team. `null` for a personal referral.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referring_account: Option<AccountSummary>,
    /// Public username.
    #[serde(default)]
    pub username: String,
    /// When the user became a verified Whop Partner, as an ISO 8601 timestamp. Null if not verified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whop_partner_verified_at: Option<String>,
}

impl AccountPartner {
    pub fn builder() -> AccountPartnerBuilder {
        <AccountPartnerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountPartnerBuilder {
    email: Option<String>,
    id: Option<String>,
    name: Option<String>,
    profile_picture: Option<UserProfilePicture>,
    referring_account: Option<AccountSummary>,
    username: Option<String>,
    whop_partner_verified_at: Option<String>,
}

impl AccountPartnerBuilder {
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

    pub fn referring_account(mut self, value: AccountSummary) -> Self {
        self.referring_account = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    pub fn whop_partner_verified_at(mut self, value: impl Into<String>) -> Self {
        self.whop_partner_verified_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AccountPartner`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](AccountPartnerBuilder::id)
    /// - [`profile_picture`](AccountPartnerBuilder::profile_picture)
    /// - [`username`](AccountPartnerBuilder::username)
    pub fn build(self) -> Result<AccountPartner, BuildError> {
        Ok(AccountPartner {
            email: self.email,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            profile_picture: self
                .profile_picture
                .ok_or_else(|| BuildError::missing_field("profile_picture"))?,
            referring_account: self.referring_account,
            username: self
                .username
                .ok_or_else(|| BuildError::missing_field("username"))?,
            whop_partner_verified_at: self.whop_partner_verified_at,
        })
    }
}
