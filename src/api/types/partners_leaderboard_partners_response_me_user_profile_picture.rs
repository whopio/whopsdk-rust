pub use crate::prelude::*;

/// The user's profile picture. Present only on the caller's own entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LeaderboardPartnersResponseMeUserProfilePicture {
    /// The user's profile picture URL.
    #[serde(default)]
    pub url: String,
}

impl LeaderboardPartnersResponseMeUserProfilePicture {
    pub fn builder() -> LeaderboardPartnersResponseMeUserProfilePictureBuilder {
        <LeaderboardPartnersResponseMeUserProfilePictureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeaderboardPartnersResponseMeUserProfilePictureBuilder {
    url: Option<String>,
}

impl LeaderboardPartnersResponseMeUserProfilePictureBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LeaderboardPartnersResponseMeUserProfilePicture`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](LeaderboardPartnersResponseMeUserProfilePictureBuilder::url)
    pub fn build(self) -> Result<LeaderboardPartnersResponseMeUserProfilePicture, BuildError> {
        Ok(LeaderboardPartnersResponseMeUserProfilePicture {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
