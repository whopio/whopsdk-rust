pub use crate::prelude::*;

/// The platform to create the social account on. `facebook` requires the account's `banner_image`, `logo`, and `description`; configure them with [Update Account](/api-reference/beta/accounts/update-account).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CreateSocialAccountsRequestPlatform {
    #[serde(rename = "facebook")]
    Facebook,
}
impl fmt::Display for CreateSocialAccountsRequestPlatform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Facebook => "facebook",
        };
        write!(f, "{}", s)
    }
}
