pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAdConversionValueRulesRequestEventsItem {
    /// Exact custom event name. Required for custom events; null for standard events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    pub event_name: CreateAdConversionValueRulesRequestEventsItemEventName,
}

impl CreateAdConversionValueRulesRequestEventsItem {
    pub fn builder() -> CreateAdConversionValueRulesRequestEventsItemBuilder {
        <CreateAdConversionValueRulesRequestEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdConversionValueRulesRequestEventsItemBuilder {
    custom_name: Option<String>,
    event_name: Option<CreateAdConversionValueRulesRequestEventsItemEventName>,
}

impl CreateAdConversionValueRulesRequestEventsItemBuilder {
    pub fn custom_name(mut self, value: impl Into<String>) -> Self {
        self.custom_name = Some(value.into());
        self
    }

    pub fn event_name(
        mut self,
        value: CreateAdConversionValueRulesRequestEventsItemEventName,
    ) -> Self {
        self.event_name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAdConversionValueRulesRequestEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_name`](CreateAdConversionValueRulesRequestEventsItemBuilder::event_name)
    pub fn build(self) -> Result<CreateAdConversionValueRulesRequestEventsItem, BuildError> {
        Ok(CreateAdConversionValueRulesRequestEventsItem {
            custom_name: self.custom_name,
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
        })
    }
}
