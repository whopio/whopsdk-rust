pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MembershipExternalAccount {
    /// The member's user ID on the platform, such as a Discord user ID. Prefixed with the platform (`discord_<external_id>`), it is the `external_account` value List Memberships filters by.
    #[serde(default)]
    pub external_id: String,
    /// The platform the member's account is on.
    pub platform: MembershipExternalAccountPlatform,
    /// The member's username on the platform. `null` when Whop doesn't know it, such as for a membership imported with only a Discord ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl MembershipExternalAccount {
    pub fn builder() -> MembershipExternalAccountBuilder {
        <MembershipExternalAccountBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipExternalAccountBuilder {
    external_id: Option<String>,
    platform: Option<MembershipExternalAccountPlatform>,
    username: Option<String>,
}

impl MembershipExternalAccountBuilder {
    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn platform(mut self, value: MembershipExternalAccountPlatform) -> Self {
        self.platform = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipExternalAccount`].
    /// This method will fail if any of the following fields are not set:
    /// - [`external_id`](MembershipExternalAccountBuilder::external_id)
    /// - [`platform`](MembershipExternalAccountBuilder::platform)
    pub fn build(self) -> Result<MembershipExternalAccount, BuildError> {
        Ok(MembershipExternalAccount {
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
