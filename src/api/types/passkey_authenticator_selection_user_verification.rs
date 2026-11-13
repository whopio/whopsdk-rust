pub use crate::prelude::*;

/// The authenticator must verify the user, with a biometric or the device's screen lock.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PasskeyAuthenticatorSelectionUserVerification {
    #[serde(rename = "required")]
    Required,
}
impl fmt::Display for PasskeyAuthenticatorSelectionUserVerification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Required => "required",
        };
        write!(f, "{}", s)
    }
}
