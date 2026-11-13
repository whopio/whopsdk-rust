pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum PaymentNextAction {
    #[serde(rename = "await_confirmation")]
    #[non_exhaustive]
    AwaitConfirmation {
        #[serde(default)]
        data: PaymentNextActionAwaitConfirmationData,
        #[serde(default)]
        render: Vec<PaymentNextActionAwaitConfirmationRenderItem>,
    },

    #[serde(rename = "collect_card_present")]
    #[non_exhaustive]
    CollectCardPresent {
        #[serde(default)]
        data: PaymentNextActionCollectCardPresentData,
    },

    #[serde(rename = "display_instructions")]
    #[non_exhaustive]
    DisplayInstructions {
        data: PaymentInstructions,
        #[serde(default)]
        render: Vec<PaymentNextActionDisplayInstructionsRenderItem>,
    },

    #[serde(rename = "redirect")]
    #[non_exhaustive]
    Redirect {
        #[serde(default)]
        data: PaymentNextActionRedirectData,
        #[serde(default)]
        render: Vec<PaymentNextActionRedirectRenderItem>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PaymentNextAction {
    pub fn await_confirmation(
        data: PaymentNextActionAwaitConfirmationData,
        render: Vec<PaymentNextActionAwaitConfirmationRenderItem>,
    ) -> Self {
        Self::AwaitConfirmation { data, render }
    }

    pub fn collect_card_present(data: PaymentNextActionCollectCardPresentData) -> Self {
        Self::CollectCardPresent { data }
    }

    pub fn display_instructions(
        data: PaymentInstructions,
        render: Vec<PaymentNextActionDisplayInstructionsRenderItem>,
    ) -> Self {
        Self::DisplayInstructions { data, render }
    }

    pub fn redirect(
        data: PaymentNextActionRedirectData,
        render: Vec<PaymentNextActionRedirectRenderItem>,
    ) -> Self {
        Self::Redirect { data, render }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
