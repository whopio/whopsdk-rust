pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetupStatus {
    /// The setup this status describes, prefixed `sint_`.
    #[serde(default)]
    pub id: String,
    /// Why the setup ended where it did, or `null` when nothing has failed. Present on `canceled` — a buyer who abandoned carries no code, one refused by the provider does.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_setup_error: Option<SetupLastSetupError>,
    /// What the buyer must do next while `status` is `requires_action`, otherwise `null`. `type` picks the shape and each variant carries only its own `data`, so switching on `type` gives you exactly that step's payload.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<PaymentNextAction>,
    /// Always `setup_status`.
    #[serde(default)]
    pub object: String,
    /// Where to send the buyer once the setup reaches a resting state, or `null` to leave them where they are.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    /// How far the setup has got. **A 200 means we answered, not that the method was saved — always branch on this.** `requires_action` — the buyer has a step outstanding; see `next_action`. `processing` — the buyer has done their part and the processor is deciding. `succeeded` — the payment method is saved, and only this one means saved. `canceled` — abandoned or refused; see `last_setup_error` to tell which.
    pub status: SetupStatusStatus,
}

impl SetupStatus {
    pub fn builder() -> SetupStatusBuilder {
        <SetupStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetupStatusBuilder {
    id: Option<String>,
    last_setup_error: Option<SetupLastSetupError>,
    next_action: Option<PaymentNextAction>,
    object: Option<String>,
    return_url: Option<String>,
    status: Option<SetupStatusStatus>,
}

impl SetupStatusBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_setup_error(mut self, value: SetupLastSetupError) -> Self {
        self.last_setup_error = Some(value);
        self
    }

    pub fn next_action(mut self, value: PaymentNextAction) -> Self {
        self.next_action = Some(value);
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: SetupStatusStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SetupStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SetupStatusBuilder::id)
    /// - [`object`](SetupStatusBuilder::object)
    /// - [`status`](SetupStatusBuilder::status)
    pub fn build(self) -> Result<SetupStatus, BuildError> {
        Ok(SetupStatus {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_setup_error: self.last_setup_error,
            next_action: self.next_action,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            return_url: self.return_url,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
