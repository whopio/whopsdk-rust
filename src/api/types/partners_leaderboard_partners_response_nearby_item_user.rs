pub use crate::prelude::*;

/// The ranked referrer. Identity fields (id, name, username, profile_picture) are returned only on the caller's own entry; other referrers expose coarse location only.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaderboardPartnersResponseNearbyItemUser {
    /// The city where the referrer is located, derived from their IP address. Null if location sharing is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The country where the referrer is located, derived from their IP address. Null if location sharing is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// User ID, prefixed `user_`. Present only on the caller's own entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The user's display name. Present only on the caller's own entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's profile picture. Present only on the caller's own entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile_picture: Option<LeaderboardPartnersResponseNearbyItemUserProfilePicture>,
    /// The user's unique username. Present only on the caller's own entry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl LeaderboardPartnersResponseNearbyItemUser {
    pub fn builder() -> LeaderboardPartnersResponseNearbyItemUserBuilder {
        <LeaderboardPartnersResponseNearbyItemUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaderboardPartnersResponseNearbyItemUserBuilder {
    city: Option<String>,
    country: Option<String>,
    id: Option<String>,
    name: Option<String>,
    profile_picture: Option<LeaderboardPartnersResponseNearbyItemUserProfilePicture>,
    username: Option<String>,
}

impl LeaderboardPartnersResponseNearbyItemUserBuilder {
    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
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

    pub fn profile_picture(
        mut self,
        value: LeaderboardPartnersResponseNearbyItemUserProfilePicture,
    ) -> Self {
        self.profile_picture = Some(value);
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeaderboardPartnersResponseNearbyItemUser`].
    pub fn build(self) -> Result<LeaderboardPartnersResponseNearbyItemUser, BuildError> {
        Ok(LeaderboardPartnersResponseNearbyItemUser {
            city: self.city,
            country: self.country,
            id: self.id,
            name: self.name,
            profile_picture: self.profile_picture,
            username: self.username,
        })
    }
}
