pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePayoutsResponseTimelineItem {
    /// Failure message, only with payout:destination:read.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub estimated_arrival: Option<DateTime<FixedOffset>>,
    pub status: RetrievePayoutsResponseTimelineItemStatus,
    /// Informational event detail; security_review identifies manual review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub timestamp: Option<DateTime<FixedOffset>>,
}

impl RetrievePayoutsResponseTimelineItem {
    pub fn builder() -> RetrievePayoutsResponseTimelineItemBuilder {
        <RetrievePayoutsResponseTimelineItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePayoutsResponseTimelineItemBuilder {
    error_message: Option<String>,
    estimated_arrival: Option<DateTime<FixedOffset>>,
    status: Option<RetrievePayoutsResponseTimelineItemStatus>,
    status_detail: Option<String>,
    timestamp: Option<DateTime<FixedOffset>>,
}

impl RetrievePayoutsResponseTimelineItemBuilder {
    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn estimated_arrival(mut self, value: DateTime<FixedOffset>) -> Self {
        self.estimated_arrival = Some(value);
        self
    }

    pub fn status(mut self, value: RetrievePayoutsResponseTimelineItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_detail(mut self, value: impl Into<String>) -> Self {
        self.status_detail = Some(value.into());
        self
    }

    pub fn timestamp(mut self, value: DateTime<FixedOffset>) -> Self {
        self.timestamp = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePayoutsResponseTimelineItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](RetrievePayoutsResponseTimelineItemBuilder::status)
    pub fn build(self) -> Result<RetrievePayoutsResponseTimelineItem, BuildError> {
        Ok(RetrievePayoutsResponseTimelineItem {
            error_message: self.error_message,
            estimated_arrival: self.estimated_arrival,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            status_detail: self.status_detail,
            timestamp: self.timestamp,
        })
    }
}
