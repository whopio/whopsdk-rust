pub use crate::prelude::*;

/// A resolution event is a message or action within a resolution case, such as a response, escalation, or status change.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResolutionCenterCaseLegacyResolutionEventsItem {
    /// The type of action recorded in this event.
    pub action: ResolutionCenterCaseActions,
    /// The datetime the resolution event was created.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub created_at: DateTime<FixedOffset>,
    /// The message body or additional context provided with this resolution event. Null if no details were included.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    /// The unique identifier for the resolution event.
    #[serde(default)]
    pub id: String,
    /// The party who performed this action.
    pub reporter_type: ResolutionCenterCaseReporters,
}

impl ResolutionCenterCaseLegacyResolutionEventsItem {
    pub fn builder() -> ResolutionCenterCaseLegacyResolutionEventsItemBuilder {
        <ResolutionCenterCaseLegacyResolutionEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolutionCenterCaseLegacyResolutionEventsItemBuilder {
    action: Option<ResolutionCenterCaseActions>,
    created_at: Option<DateTime<FixedOffset>>,
    details: Option<String>,
    id: Option<String>,
    reporter_type: Option<ResolutionCenterCaseReporters>,
}

impl ResolutionCenterCaseLegacyResolutionEventsItemBuilder {
    pub fn action(mut self, value: ResolutionCenterCaseActions) -> Self {
        self.action = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_at = Some(value);
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

    pub fn reporter_type(mut self, value: ResolutionCenterCaseReporters) -> Self {
        self.reporter_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResolutionCenterCaseLegacyResolutionEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`action`](ResolutionCenterCaseLegacyResolutionEventsItemBuilder::action)
    /// - [`created_at`](ResolutionCenterCaseLegacyResolutionEventsItemBuilder::created_at)
    /// - [`id`](ResolutionCenterCaseLegacyResolutionEventsItemBuilder::id)
    /// - [`reporter_type`](ResolutionCenterCaseLegacyResolutionEventsItemBuilder::reporter_type)
    pub fn build(self) -> Result<ResolutionCenterCaseLegacyResolutionEventsItem, BuildError> {
        Ok(ResolutionCenterCaseLegacyResolutionEventsItem {
            action: self
                .action
                .ok_or_else(|| BuildError::missing_field("action"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            details: self.details,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            reporter_type: self
                .reporter_type
                .ok_or_else(|| BuildError::missing_field("reporter_type"))?,
        })
    }
}
