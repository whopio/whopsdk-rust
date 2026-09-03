pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrieveBreakdownResponse {
    pub activity_filters: RetrieveBreakdownResponseActivityFilters,
    pub bucket: RetrieveBreakdownResponseBucket,
    #[serde(default)]
    pub currency: String,
    pub direction: RetrieveBreakdownResponseDirection,
    #[serde(default)]
    pub items: Vec<RetrieveBreakdownResponseItemsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_amount: Option<Money>,
    #[serde(default)]
    pub other_name: String,
}

impl RetrieveBreakdownResponse {
    pub fn builder() -> RetrieveBreakdownResponseBuilder {
        <RetrieveBreakdownResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBreakdownResponseBuilder {
    activity_filters: Option<RetrieveBreakdownResponseActivityFilters>,
    bucket: Option<RetrieveBreakdownResponseBucket>,
    currency: Option<String>,
    direction: Option<RetrieveBreakdownResponseDirection>,
    items: Option<Vec<RetrieveBreakdownResponseItemsItem>>,
    other_amount: Option<Money>,
    other_name: Option<String>,
}

impl RetrieveBreakdownResponseBuilder {
    pub fn activity_filters(mut self, value: RetrieveBreakdownResponseActivityFilters) -> Self {
        self.activity_filters = Some(value);
        self
    }

    pub fn bucket(mut self, value: RetrieveBreakdownResponseBucket) -> Self {
        self.bucket = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn direction(mut self, value: RetrieveBreakdownResponseDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<RetrieveBreakdownResponseItemsItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn other_amount(mut self, value: Money) -> Self {
        self.other_amount = Some(value);
        self
    }

    pub fn other_name(mut self, value: impl Into<String>) -> Self {
        self.other_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBreakdownResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`activity_filters`](RetrieveBreakdownResponseBuilder::activity_filters)
    /// - [`bucket`](RetrieveBreakdownResponseBuilder::bucket)
    /// - [`currency`](RetrieveBreakdownResponseBuilder::currency)
    /// - [`direction`](RetrieveBreakdownResponseBuilder::direction)
    /// - [`items`](RetrieveBreakdownResponseBuilder::items)
    /// - [`other_name`](RetrieveBreakdownResponseBuilder::other_name)
    pub fn build(self) -> Result<RetrieveBreakdownResponse, BuildError> {
        Ok(RetrieveBreakdownResponse {
            activity_filters: self
                .activity_filters
                .ok_or_else(|| BuildError::missing_field("activity_filters"))?,
            bucket: self
                .bucket
                .ok_or_else(|| BuildError::missing_field("bucket"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            direction: self
                .direction
                .ok_or_else(|| BuildError::missing_field("direction"))?,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            other_amount: self.other_amount,
            other_name: self
                .other_name
                .ok_or_else(|| BuildError::missing_field("other_name"))?,
        })
    }
}
