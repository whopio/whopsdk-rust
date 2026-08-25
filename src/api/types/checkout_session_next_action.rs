pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CheckoutSessionNextAction {
    #[serde(rename = "authenticate")]
    #[non_exhaustive]
    Authenticate {
        #[serde(default)]
        blocking: bool,
    },

    #[serde(rename = "await_claim")]
    #[non_exhaustive]
    AwaitClaim {
        #[serde(default)]
        blocking: bool,
        state: CheckoutSessionAwaitClaimActionState,
    },

    #[serde(rename = "complete")]
    #[non_exhaustive]
    Complete {
        #[serde(default)]
        blocking: bool,
        #[serde(default)]
        client_secret: String,
        kind: CheckoutSessionCompleteActionKind,
    },

    #[serde(rename = "redirect")]
    #[non_exhaustive]
    Redirect {
        #[serde(default)]
        blocking: bool,
        #[serde(default)]
        destination_url: String,
    },

    #[serde(rename = "upgrade_authentication")]
    #[non_exhaustive]
    UpgradeAuthentication {
        #[serde(default)]
        blocking: bool,
        #[serde(default)]
        email: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        session_intent_id: Option<String>,
        sign_in_intent: CheckoutSessionUpgradeAuthenticationActionSignInIntent,
    },

    #[serde(rename = "verify_phone")]
    #[non_exhaustive]
    VerifyPhone {
        #[serde(default)]
        blocking: bool,
    },

    #[serde(rename = "wait_for_payment")]
    #[non_exhaustive]
    WaitForPayment {
        #[serde(default)]
        blocking: bool,
        #[serde(default)]
        poll_after_seconds: i64,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CheckoutSessionNextAction {
    pub fn authenticate(blocking: bool) -> Self {
        Self::Authenticate { blocking }
    }

    pub fn await_claim(blocking: bool, state: CheckoutSessionAwaitClaimActionState) -> Self {
        Self::AwaitClaim { blocking, state }
    }

    pub fn complete(
        blocking: bool,
        client_secret: String,
        kind: CheckoutSessionCompleteActionKind,
    ) -> Self {
        Self::Complete {
            blocking,
            client_secret,
            kind,
        }
    }

    pub fn redirect(blocking: bool, destination_url: String) -> Self {
        Self::Redirect {
            blocking,
            destination_url,
        }
    }

    pub fn upgrade_authentication(
        blocking: bool,
        email: String,
        sign_in_intent: CheckoutSessionUpgradeAuthenticationActionSignInIntent,
    ) -> Self {
        Self::UpgradeAuthentication {
            blocking,
            email,
            session_intent_id: None,
            sign_in_intent,
        }
    }

    pub fn verify_phone(blocking: bool) -> Self {
        Self::VerifyPhone { blocking }
    }

    pub fn wait_for_payment(blocking: bool, poll_after_seconds: i64) -> Self {
        Self::WaitForPayment {
            blocking,
            poll_after_seconds,
        }
    }

    pub fn upgrade_authentication_with_session_intent_id(
        blocking: bool,
        email: String,
        session_intent_id: String,
        sign_in_intent: CheckoutSessionUpgradeAuthenticationActionSignInIntent,
    ) -> Self {
        Self::UpgradeAuthentication {
            blocking,
            email,
            session_intent_id: Some(session_intent_id),
            sign_in_intent,
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
