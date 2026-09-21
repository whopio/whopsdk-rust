pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpdateAdConversionValueRulesRequestEventsItem {
    /// Exact custom event name. Required for custom events; null for standard events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    pub event_name: UpdateAdConversionValueRulesRequestEventsItemEventName,
}

impl UpdateAdConversionValueRulesRequestEventsItem {
    pub fn builder() -> UpdateAdConversionValueRulesRequestEventsItemBuilder {
        <UpdateAdConversionValueRulesRequestEventsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdConversionValueRulesRequestEventsItemBuilder {
    custom_name: Option<String>,
    event_name: Option<UpdateAdConversionValueRulesRequestEventsItemEventName>,
}

impl UpdateAdConversionValueRulesRequestEventsItemBuilder {
    pub fn custom_name(mut self, value: impl Into<String>) -> Self {
        self.custom_name = Some(value.into());
        self
    }

    pub fn event_name(
        mut self,
        value: UpdateAdConversionValueRulesRequestEventsItemEventName,
    ) -> Self {
        self.event_name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdConversionValueRulesRequestEventsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_name`](UpdateAdConversionValueRulesRequestEventsItemBuilder::event_name)
    pub fn build(self) -> Result<UpdateAdConversionValueRulesRequestEventsItem, BuildError> {
        Ok(UpdateAdConversionValueRulesRequestEventsItem {
            custom_name: self.custom_name,
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
        })
    }
}
