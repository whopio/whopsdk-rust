pub use crate::prelude::*;

/// A member's account on an outside platform, such as Discord or Telegram.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipListItemExternalAccountsItem {
    /// The member's user ID on the platform, such as a Discord user ID. Prefixed with the platform (`discord_<external_id>`), it is the `external_account` value List Memberships filters by.
    #[serde(default)]
    pub external_id: String,
    /// The platform the member's account is on: `discord` or `telegram`.
    #[serde(default)]
    pub platform: String,
    /// The member's username on the platform. Null when Whop doesn't know it, such as for a membership imported with only a Discord ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl MembershipListItemExternalAccountsItem {
    pub fn builder() -> MembershipListItemExternalAccountsItemBuilder {
        <MembershipListItemExternalAccountsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipListItemExternalAccountsItemBuilder {
    external_id: Option<String>,
    platform: Option<String>,
    username: Option<String>,
}

impl MembershipListItemExternalAccountsItemBuilder {
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn platform(mut self, value: impl Into<String>) -> Self {
        self.platform = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipListItemExternalAccountsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_id`](MembershipListItemExternalAccountsItemBuilder::external_id)
    /// - [`platform`](MembershipListItemExternalAccountsItemBuilder::platform)
    pub fn build(self) -> Result<MembershipListItemExternalAccountsItem, BuildError> {
        Ok(MembershipListItemExternalAccountsItem {
            external_id: self
                .external_id
                .ok_or_else(|| BuildError::missing_field("external_id"))?,
            platform: self
                .platform
                .ok_or_else(|| BuildError::missing_field("platform"))?,
            username: self.username,
        })
    }
}
