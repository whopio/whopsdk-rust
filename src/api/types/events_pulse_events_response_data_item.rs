pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PulseEventsResponseDataItem {
    /// The underlying event recorded. Every movement on this feed is a ledger line, so switch on `type` rather than this.
    pub event_name: PulseEventsResponseDataItemEventName,
    /// When the event happened, coarsened to the start of the minute.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub event_time: DateTime<FixedOffset>,
    /// The USD amount of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_usd_amount: Option<f64>,
    /// What moved: a purchase, an affiliate commission, Whop card spend, ad spend, app revenue, an off-platform sale, a wallet deposit, a card load, a claimed drop, a transfer between accounts, or a referral bonus.
    pub r#type: PulseEventsResponseDataItemType,
    /// Coarse location, shaped like the event `user` block. It belongs to the owner of the wallet the money moved into or out of — the party the event is about, never their counterparty. Omitted entirely when nothing is known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<PulseEventsResponseDataItemUser>,
}

impl PulseEventsResponseDataItem {
    pub fn builder() -> PulseEventsResponseDataItemBuilder {
        <PulseEventsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PulseEventsResponseDataItemBuilder {
    event_name: Option<PulseEventsResponseDataItemEventName>,
    event_time: Option<DateTime<FixedOffset>>,
    total_usd_amount: Option<f64>,
    r#type: Option<PulseEventsResponseDataItemType>,
    user: Option<PulseEventsResponseDataItemUser>,
}

impl PulseEventsResponseDataItemBuilder {
    pub fn event_name(mut self, value: PulseEventsResponseDataItemEventName) -> Self {
        self.event_name = Some(value);
        self
    }

    pub fn event_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.event_time = Some(value);
        self
    }

    pub fn total_usd_amount(mut self, value: f64) -> Self {
        self.total_usd_amount = Some(value);
        self
    }

    pub fn r#type(mut self, value: PulseEventsResponseDataItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn user(mut self, value: PulseEventsResponseDataItemUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PulseEventsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_name`](PulseEventsResponseDataItemBuilder::event_name)
    /// - [`event_time`](PulseEventsResponseDataItemBuilder::event_time)
    /// - [`r#type`](PulseEventsResponseDataItemBuilder::r#type)
    pub fn build(self) -> Result<PulseEventsResponseDataItem, BuildError> {
        Ok(PulseEventsResponseDataItem {
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
            event_time: self
                .event_time
                .ok_or_else(|| BuildError::missing_field("event_time"))?,
            total_usd_amount: self.total_usd_amount,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            user: self.user,
        })
    }
}
