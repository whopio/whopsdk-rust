pub use crate::prelude::*;

/// The user's profile picture. Present only on the caller's own entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaderboardPartnersResponseLeadersItemUserProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl LeaderboardPartnersResponseLeadersItemUserProfilePicture {
    pub fn builder() -> LeaderboardPartnersResponseLeadersItemUserProfilePictureBuilder {
        <LeaderboardPartnersResponseLeadersItemUserProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaderboardPartnersResponseLeadersItemUserProfilePictureBuilder {
    url: Option<String>,
}

impl LeaderboardPartnersResponseLeadersItemUserProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeaderboardPartnersResponseLeadersItemUserProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](LeaderboardPartnersResponseLeadersItemUserProfilePictureBuilder::url)
    pub fn build(
        self,
    ) -> Result<LeaderboardPartnersResponseLeadersItemUserProfilePicture, BuildError> {
        Ok(LeaderboardPartnersResponseLeadersItemUserProfilePicture {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
