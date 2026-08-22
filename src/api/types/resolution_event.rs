pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResolutionEvent {
    /// The action recorded in this event.
    pub action: ResolutionEventAction,
    #[serde(default)]
    pub attachments: Vec<ResolutionAttachment>,
    /// When the event occurred, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// The message body or additional context provided with the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// Unique identifier for the event, prefixed `revt_`.
    #[serde(default)]
    pub id: String,
    /// The party that performed the action.
    pub reporter_type: ResolutionEventReporterType,
    /// Whether the customer can see this event in the timeline.
    #[serde(default)]
    pub viewable_by_customer: bool,
    /// Whether the merchant can see this event in the timeline.
    #[serde(default)]
    pub viewable_by_merchant: bool,
}

impl ResolutionEvent {
    pub fn builder() -> ResolutionEventBuilder {
        <ResolutionEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionEventBuilder {
    action: Option<ResolutionEventAction>,
    attachments: Option<Vec<ResolutionAttachment>>,
    created_at: Option<String>,
    details: Option<String>,
    id: Option<String>,
    reporter_type: Option<ResolutionEventReporterType>,
    viewable_by_customer: Option<bool>,
    viewable_by_merchant: Option<bool>,
}

impl ResolutionEventBuilder {
    pub fn action(mut self, value: ResolutionEventAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn attachments(mut self, value: Vec<ResolutionAttachment>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn details(mut self, value: impl Into<String>) -> Self {
        self.details = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn reporter_type(mut self, value: ResolutionEventReporterType) -> Self {
        self.reporter_type = Some(value);
        self
    }

    pub fn viewable_by_customer(mut self, value: bool) -> Self {
        self.viewable_by_customer = Some(value);
        self
    }

    pub fn viewable_by_merchant(mut self, value: bool) -> Self {
        self.viewable_by_merchant = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResolutionEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](ResolutionEventBuilder::action)
    /// - [`attachments`](ResolutionEventBuilder::attachments)
    /// - [`created_at`](ResolutionEventBuilder::created_at)
    /// - [`id`](ResolutionEventBuilder::id)
    /// - [`reporter_type`](ResolutionEventBuilder::reporter_type)
    /// - [`viewable_by_customer`](ResolutionEventBuilder::viewable_by_customer)
    /// - [`viewable_by_merchant`](ResolutionEventBuilder::viewable_by_merchant)
    pub fn build(self) -> Result<ResolutionEvent, BuildError> {
        Ok(ResolutionEvent {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            attachments: self
                .attachments
                .ok_or_else(|| BuildError::missing_field("attachments"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            details: self.details,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reporter_type: self
                .reporter_type
                .ok_or_else(|| BuildError::missing_field("reporter_type"))?,
            viewable_by_customer: self
                .viewable_by_customer
                .ok_or_else(|| BuildError::missing_field("viewable_by_customer"))?,
            viewable_by_merchant: self
                .viewable_by_merchant
                .ok_or_else(|| BuildError::missing_field("viewable_by_merchant"))?,
        })
    }
}
