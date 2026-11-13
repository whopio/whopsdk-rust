pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AutomationStepPreview {
    /// The step's action type: `email` or `message`.
    pub action_type: AutomationStepPreviewActionType,
    /// The email's HTML, with Whop's footer. Its unsubscribe link is a placeholder until the email is sent. `null` unless an `email` step sends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    /// What the step does for this member: `sent` its message, `skipped` it and moved on to the next step, or `exited` the automation.
    pub outcome: AutomationStepPreviewOutcome,
    /// The inbox preview line after the subject. `null` when the email has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preheader: Option<String>,
    /// Why the step skipped or exited, as the step's code said. `null` when it sends or gave no reason.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// The email's subject. `null` unless an `email` step sends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// The message as plain text: the message on a `message` step, or the email's plain-text version with Whop's footer. `null` unless the step sends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl AutomationStepPreview {
    pub fn builder() -> AutomationStepPreviewBuilder {
        <AutomationStepPreviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutomationStepPreviewBuilder {
    action_type: Option<AutomationStepPreviewActionType>,
    html: Option<String>,
    outcome: Option<AutomationStepPreviewOutcome>,
    preheader: Option<String>,
    reason: Option<String>,
    subject: Option<String>,
    text: Option<String>,
}

impl AutomationStepPreviewBuilder {
    pub fn action_type(mut self, value: AutomationStepPreviewActionType) -> Self {
        self.action_type = Some(value);
        self
    }

    pub fn html(mut self, value: impl Into<String>) -> Self {
        self.html = Some(value.into());
        self
    }

    pub fn outcome(mut self, value: AutomationStepPreviewOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn preheader(mut self, value: impl Into<String>) -> Self {
        self.preheader = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn subject(mut self, value: impl Into<String>) -> Self {
        self.subject = Some(value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutomationStepPreview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action_type`](AutomationStepPreviewBuilder::action_type)
    /// - [`outcome`](AutomationStepPreviewBuilder::outcome)
    pub fn build(self) -> Result<AutomationStepPreview, BuildError> {
        Ok(AutomationStepPreview {
            action_type: self
                .action_type
                .ok_or_else(|| BuildError::missing_field("action_type"))?,
            html: self.html,
            outcome: self
                .outcome
                .ok_or_else(|| BuildError::missing_field("outcome"))?,
            preheader: self.preheader,
            reason: self.reason,
            subject: self.subject,
            text: self.text,
        })
    }
}
