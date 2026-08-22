pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PixelValidation {
    /// False when the event lookup failed, meaning `host_events` and `last_seen_days` are incomplete.
    #[serde(default)]
    pub firing_data_ok: bool,
    #[serde(default)]
    pub host_events: Vec<String>,
    /// Whether the pixel was seen. Without a `url` this answers for the whole account: true when it has sent events recently. With a `url` it answers for THAT page only — true when the page is hosted on Whop, when the page itself has sent events recently, or when the pixel was found in its source. Events the account sent from other pages do not make a given `url` installed.
    #[serde(default)]
    pub installed: bool,
    /// Event name to whole days since that event last fired, e.g. `{ "lead": 3 }`. Carries events that fired too long ago to count as installed, so you can prompt to re-check rather than report them missing.
    #[serde(default)]
    pub last_fired_days: HashMap<String, serde_json::Value>,
    /// Days since the pixel last sent an event, within a 30-day window. `null` when it hasn't sent one in that window — which includes a pixel installed moments ago.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub last_seen_days: Option<f64>,
    /// True when `url` is hosted on Whop, so no pixel snippet is required.
    #[serde(default)]
    pub native_tracking: bool,
    #[serde(default)]
    pub page_events: Vec<String>,
    /// Whether the page could be loaded. `null` when the request included no URL, and `true` when events settled the answer without a fetch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reachable: Option<bool>,
    /// The URL that was checked, after normalization. `null` when the request didn't include one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl PixelValidation {
    pub fn builder() -> PixelValidationBuilder {
        <PixelValidationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PixelValidationBuilder {
    firing_data_ok: Option<bool>,
    host_events: Option<Vec<String>>,
    installed: Option<bool>,
    last_fired_days: Option<HashMap<String, serde_json::Value>>,
    last_seen_days: Option<f64>,
    native_tracking: Option<bool>,
    page_events: Option<Vec<String>>,
    reachable: Option<bool>,
    url: Option<String>,
}

impl PixelValidationBuilder {
    pub fn firing_data_ok(mut self, value: bool) -> Self {
        self.firing_data_ok = Some(value);
        self
    }

    pub fn host_events(mut self, value: Vec<String>) -> Self {
        self.host_events = Some(value);
        self
    }

    pub fn installed(mut self, value: bool) -> Self {
        self.installed = Some(value);
        self
    }

    pub fn last_fired_days(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.last_fired_days = Some(value);
        self
    }

    pub fn last_seen_days(mut self, value: f64) -> Self {
        self.last_seen_days = Some(value);
        self
    }

    pub fn native_tracking(mut self, value: bool) -> Self {
        self.native_tracking = Some(value);
        self
    }

    pub fn page_events(mut self, value: Vec<String>) -> Self {
        self.page_events = Some(value);
        self
    }

    pub fn reachable(mut self, value: bool) -> Self {
        self.reachable = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PixelValidation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`firing_data_ok`](PixelValidationBuilder::firing_data_ok)
    /// - [`host_events`](PixelValidationBuilder::host_events)
    /// - [`installed`](PixelValidationBuilder::installed)
    /// - [`last_fired_days`](PixelValidationBuilder::last_fired_days)
    /// - [`native_tracking`](PixelValidationBuilder::native_tracking)
    /// - [`page_events`](PixelValidationBuilder::page_events)
    pub fn build(self) -> Result<PixelValidation, BuildError> {
        Ok(PixelValidation {
            firing_data_ok: self
                .firing_data_ok
                .ok_or_else(|| BuildError::missing_field("firing_data_ok"))?,
            host_events: self
                .host_events
                .ok_or_else(|| BuildError::missing_field("host_events"))?,
            installed: self
                .installed
                .ok_or_else(|| BuildError::missing_field("installed"))?,
            last_fired_days: self
                .last_fired_days
                .ok_or_else(|| BuildError::missing_field("last_fired_days"))?,
            last_seen_days: self.last_seen_days,
            native_tracking: self
                .native_tracking
                .ok_or_else(|| BuildError::missing_field("native_tracking"))?,
            page_events: self
                .page_events
                .ok_or_else(|| BuildError::missing_field("page_events"))?,
            reachable: self.reachable,
            url: self.url,
        })
    }
}
