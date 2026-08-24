pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrievePeopleResponsePurchasesItem {
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub occurred_at: DateTime<FixedOffset>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub usd_value: f64,
}

impl RetrievePeopleResponsePurchasesItem {
    pub fn builder() -> RetrievePeopleResponsePurchasesItemBuilder {
        <RetrievePeopleResponsePurchasesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePeopleResponsePurchasesItemBuilder {
    event_id: Option<String>,
    occurred_at: Option<DateTime<FixedOffset>>,
    usd_value: Option<f64>,
}

impl RetrievePeopleResponsePurchasesItemBuilder {
    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.occurred_at = Some(value);
        self
    }

    pub fn usd_value(mut self, value: f64) -> Self {
        self.usd_value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePeopleResponsePurchasesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](RetrievePeopleResponsePurchasesItemBuilder::event_id)
    /// - [`occurred_at`](RetrievePeopleResponsePurchasesItemBuilder::occurred_at)
    /// - [`usd_value`](RetrievePeopleResponsePurchasesItemBuilder::usd_value)
    pub fn build(self) -> Result<RetrievePeopleResponsePurchasesItem, BuildError> {
        Ok(RetrievePeopleResponsePurchasesItem {
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            occurred_at: self
                .occurred_at
                .ok_or_else(|| BuildError::missing_field("occurred_at"))?,
            usd_value: self
                .usd_value
                .ok_or_else(|| BuildError::missing_field("usd_value"))?,
        })
    }
}
