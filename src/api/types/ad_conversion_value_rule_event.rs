pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdConversionValueRuleEvent {
    /// Exact custom event name. Null for standard events.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// Event to adjust. Purchase includes Whop purchases and external purchase events.
    pub event_name: AdConversionValueRuleEventEventName,
}

impl AdConversionValueRuleEvent {
    pub fn builder() -> AdConversionValueRuleEventBuilder {
        <AdConversionValueRuleEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdConversionValueRuleEventBuilder {
    custom_name: Option<String>,
    event_name: Option<AdConversionValueRuleEventEventName>,
}

impl AdConversionValueRuleEventBuilder {
    pub fn custom_name(mut self, value: impl Into<String>) -> Self {
        self.custom_name = Some(value.into());
        self
    }

    pub fn event_name(mut self, value: AdConversionValueRuleEventEventName) -> Self {
        self.event_name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdConversionValueRuleEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event_name`](AdConversionValueRuleEventBuilder::event_name)
    pub fn build(self) -> Result<AdConversionValueRuleEvent, BuildError> {
        Ok(AdConversionValueRuleEvent {
            custom_name: self.custom_name,
            event_name: self
                .event_name
                .ok_or_else(|| BuildError::missing_field("event_name"))?,
        })
    }
}
