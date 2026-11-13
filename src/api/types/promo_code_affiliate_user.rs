pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromoCodeAffiliateUser {
    /// Affiliate user ID, prefixed `user_`.
    #[serde(default)]
    pub id: String,
    /// Display name of the affiliate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Affiliate avatar URL, including the generated fallback when no avatar is set.
    #[serde(default)]
    pub profile_pic: String,
    /// Public username of the affiliate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl PromoCodeAffiliateUser {
    pub fn builder() -> PromoCodeAffiliateUserBuilder {
        <PromoCodeAffiliateUserBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromoCodeAffiliateUserBuilder {
    id: Option<String>,
    name: Option<String>,
    profile_pic: Option<String>,
    username: Option<String>,
}

impl PromoCodeAffiliateUserBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn profile_pic(mut self, value: impl Into<String>) -> Self {
        self.profile_pic = Some(value.into());
        self
    }

    pub fn username(mut self, value: impl Into<String>) -> Self {
        self.username = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PromoCodeAffiliateUser`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PromoCodeAffiliateUserBuilder::id)
    /// - [`profile_pic`](PromoCodeAffiliateUserBuilder::profile_pic)
    pub fn build(self) -> Result<PromoCodeAffiliateUser, BuildError> {
        Ok(PromoCodeAffiliateUser {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name,
            profile_pic: self
                .profile_pic
                .ok_or_else(|| BuildError::missing_field("profile_pic"))?,
            username: self.username,
        })
    }
}
