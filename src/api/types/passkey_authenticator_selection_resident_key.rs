pub use crate::prelude::*;

/// The authenticator must store a discoverable passkey, so it can sign in without a username.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PasskeyAuthenticatorSelectionResidentKey {
    #[serde(rename = "required")]
    Required,
}
impl fmt::Display for PasskeyAuthenticatorSelectionResidentKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Required => "required",
        };
        write!(f, "{}", s)
    }
}
