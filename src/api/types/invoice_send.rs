pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoiceSend {
    /// The user who requested the send, when you have `company:authorized_user:read`. `null` for automatic sends, account API keys, apps, or when the caller cannot read team members. Whop employee identities are shown as Whop Team to external viewers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<UserSummary>,
    /// How batches are classified: manual batches use `send` when no earlier completed email history exists and `resend` otherwise. Classification happens when history is recorded. Earlier unrecorded emails are unknown. Automatic sends use `scheduled_send` for a scheduled send and `reminder` for a reminder after the due date. They use `payment_follow_up` for the payment link after a charge to the saved payment method failed or needed the customer to act.
    pub kind: InvoiceSendKind,
    /// How many addresses were submitted for delivery, each separately.
    #[serde(default)]
    pub recipient_count: i64,
    #[serde(default)]
    pub recipients: Vec<String>,
    /// When the worker finished submitting this batch for delivery, as an ISO 8601 timestamp. Partial batches are not recorded until a retry completes them. This does not confirm delivery to an inbox.
    #[serde(default)]
    pub sent_at: String,
}

impl InvoiceSend {
    pub fn builder() -> InvoiceSendBuilder {
        <InvoiceSendBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceSendBuilder {
    actor: Option<UserSummary>,
    kind: Option<InvoiceSendKind>,
    recipient_count: Option<i64>,
    recipients: Option<Vec<String>>,
    sent_at: Option<String>,
}

impl InvoiceSendBuilder {
    pub fn actor(mut self, value: UserSummary) -> Self {
        self.actor = Some(value);
        self
    }

    pub fn kind(mut self, value: InvoiceSendKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn recipient_count(mut self, value: i64) -> Self {
        self.recipient_count = Some(value);
        self
    }

    pub fn recipients(mut self, value: Vec<String>) -> Self {
        self.recipients = Some(value);
        self
    }

    pub fn sent_at(mut self, value: impl Into<String>) -> Self {
        self.sent_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceSend`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](InvoiceSendBuilder::kind)
    /// - [`recipient_count`](InvoiceSendBuilder::recipient_count)
    /// - [`recipients`](InvoiceSendBuilder::recipients)
    /// - [`sent_at`](InvoiceSendBuilder::sent_at)
    pub fn build(self) -> Result<InvoiceSend, BuildError> {
        Ok(InvoiceSend {
            actor: self.actor,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            recipient_count: self
                .recipient_count
                .ok_or_else(|| BuildError::missing_field("recipient_count"))?,
            recipients: self
                .recipients
                .ok_or_else(|| BuildError::missing_field("recipients"))?,
            sent_at: self
                .sent_at
                .ok_or_else(|| BuildError::missing_field("sent_at"))?,
        })
    }
}
