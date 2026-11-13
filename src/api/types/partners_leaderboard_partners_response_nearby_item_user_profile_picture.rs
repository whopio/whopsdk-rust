pub use crate::prelude::*;

/// The user's profile picture. Present only on the caller's own entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaderboardPartnersResponseNearbyItemUserProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl LeaderboardPartnersResponseNearbyItemUserProfilePicture {
    pub fn builder() -> LeaderboardPartnersResponseNearbyItemUserProfilePictureBuilder {
        <LeaderboardPartnersResponseNearbyItemUserProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaderboardPartnersResponseNearbyItemUserProfilePictureBuilder {
    url: Option<String>,
}

impl LeaderboardPartnersResponseNearbyItemUserProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeaderboardPartnersResponseNearbyItemUserProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](LeaderboardPartnersResponseNearbyItemUserProfilePictureBuilder::url)
    pub fn build(
        self,
    ) -> Result<LeaderboardPartnersResponseNearbyItemUserProfilePicture, BuildError> {
        Ok(LeaderboardPartnersResponseNearbyItemUserProfilePicture {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
