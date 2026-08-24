pub use crate::prelude::*;

/// Information to aid in pagination.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeliveriesWebhookResponsePageInfo {
    /// When paginating forwards, the cursor to continue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_cursor: Option<String>,
    /// When paginating forwards, are there more items?
    #[serde(default)]
    pub has_next_page: bool,
}

impl DeliveriesWebhookResponsePageInfo {
    pub fn builder() -> DeliveriesWebhookResponsePageInfoBuilder {
        <DeliveriesWebhookResponsePageInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeliveriesWebhookResponsePageInfoBuilder {
    end_cursor: Option<String>,
    has_next_page: Option<bool>,
}

impl DeliveriesWebhookResponsePageInfoBuilder {
    pub fn end_cursor(mut self, value: impl Into<String>) -> Self {
        self.end_cursor = Some(value.into());
        self
    }

    pub fn has_next_page(mut self, value: bool) -> Self {
        self.has_next_page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeliveriesWebhookResponsePageInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_next_page`](DeliveriesWebhookResponsePageInfoBuilder::has_next_page)
    pub fn build(self) -> Result<DeliveriesWebhookResponsePageInfo, BuildError> {
        Ok(DeliveriesWebhookResponsePageInfo {
            end_cursor: self.end_cursor,
            has_next_page: self
                .has_next_page
                .ok_or_else(|| BuildError::missing_field("has_next_page"))?,
        })
    }
}
