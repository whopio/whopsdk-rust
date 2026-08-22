pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEventsResponseDataItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ListEventsResponseDataItemContext>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    #[serde(default)]
    pub event_id: String,
    #[serde(default)]
    pub event_name: String,
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub event_time: DateTime<FixedOffset>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(default)]
    pub person_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub questions: Option<Vec<ListEventsResponseDataItemQuestionsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_action_chain_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommended_action_shown_position: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer_url: Option<String>,
    /// Hydrated details for the records this event references. Only present keys resolved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related: Option<ListEventsResponseDataItemRelated>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_usd_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ListEventsResponseDataItemUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<f64>,
}

impl ListEventsResponseDataItem {
    pub fn builder() -> ListEventsResponseDataItemBuilder {
        <ListEventsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemBuilder {
    context: Option<ListEventsResponseDataItemContext>,
    currency: Option<String>,
    custom_name: Option<String>,
    event_id: Option<String>,
    event_name: Option<String>,
    event_time: Option<DateTime<FixedOffset>>,
    id: Option<String>,
    path: Option<String>,
    person_id: Option<String>,
    questions: Option<Vec<ListEventsResponseDataItemQuestionsItem>>,
    recommended_action_chain_id: Option<String>,
    recommended_action_shown_position: Option<i64>,
    referrer_url: Option<String>,
    related: Option<ListEventsResponseDataItemRelated>,
    total_usd_amount: Option<f64>,
    url: Option<String>,
    user: Option<ListEventsResponseDataItemUser>,
    value: Option<f64>,
}

impl ListEventsResponseDataItemBuilder {
    pub fn context(mut self, value: ListEventsResponseDataItemContext) -> Self {
        self.context = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn custom_name(mut self, value: impl Into<String>) -> Self {
        self.custom_name = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn event_name(mut self, value: impl Into<String>) -> Self {
        self.event_name = Some(value.into());
        self
    }

    pub fn event_time(mut self, value: DateTime<FixedOffset>) -> Self {
        self.event_time = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn person_id(mut self, value: impl Into<String>) -> Self {
        self.person_id = Some(value.into());
        self
    }

    pub fn questions(mut self, value: Vec<ListEventsResponseDataItemQuestionsItem>) -> Self {
        self.questions = Some(value);
        self
    }

    pub fn recommended_action_chain_id(mut self, value: impl Into<String>) -> Self {
        self.recommended_action_chain_id = Some(value.into());
        self
    }

    pub fn recommended_action_shown_position(mut self, value: i64) -> Self {
        self.recommended_action_shown_position = Some(value);
        self
    }

    pub fn referrer_url(mut self, value: impl Into<String>) -> Self {
        self.referrer_url = Some(value.into());
        self
    }

    pub fn related(mut self, value: ListEventsResponseDataItemRelated) -> Self {
        self.related = Some(value);
        self
    }

    pub fn total_usd_amount(mut self, value: f64) -> Self {
        self.total_usd_amount = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    pub fn user(mut self, value: ListEventsResponseDataItemUser) -> Self {
        self.user = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEventsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_id`](ListEventsResponseDataItemBuilder::event_id)
    /// - [`event_name`](ListEventsResponseDataItemBuilder::event_name)
    /// - [`event_time`](ListEventsResponseDataItemBuilder::event_time)
    /// - [`id`](ListEventsResponseDataItemBuilder::id)
    /// - [`person_id`](ListEventsResponseDataItemBuilder::person_id)
    pub fn build(self) -> Result<ListEventsResponseDataItem, BuildError> {
        Ok(ListEventsResponseDataItem {
            context: self.context,
            currency: self.currency,
            custom_name: self.custom_name,
            event_id: self
                .event_id
                .ok_or_else(|| BuildError::missing_field("event_id"))?,
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
            event_time: self
                .event_time
                .ok_or_else(|| BuildError::missing_field("event_time"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            path: self.path,
            person_id: self
                .person_id
                .ok_or_else(|| BuildError::missing_field("person_id"))?,
            questions: self.questions,
            recommended_action_chain_id: self.recommended_action_chain_id,
            recommended_action_shown_position: self.recommended_action_shown_position,
            referrer_url: self.referrer_url,
            related: self.related,
            total_usd_amount: self.total_usd_amount,
            url: self.url,
            user: self.user,
            value: self.value,
        })
    }
}
