pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FunnelStep {
    /// The Whop app build ID attached to the event. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_build_id: Option<String>,
    /// The event's app id. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// The event's browser. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub browser: Option<String>,
    /// The event's city. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The event's country. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The custom event name. Requires event=pixel.custom. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// The event's device. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<String>,
    /// The event name, such as pixel.page, pixel.custom, experiment.exposure, or payment.completed. Exactly one event per step.
    #[serde(default)]
    pub event: String,
    /// An experiment ID owned by account_id. Requires event=experiment.exposure. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experiment_id: Option<String>,
    /// The event's hostname. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    /// The event's os. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// The URL pathname. Use * to match zero or more characters, including slashes. Other characters match literally. Missing pages do not match. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,
    /// The plan ID attached to the event. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<String>,
    /// The product ID attached to the event. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The Whop ad-click source of this page view, not conversion attribution. Use whop:* for any Whop ad click, whop:<campaign>:* for a campaign, or whop:<campaign>:<group>:<ad> for an exact ad. Only ad-click page-view events support source. With breakdown_by=source, trailing :* levels select campaign, ad-group, or ad detail. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The event's utm source. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_source: Option<String>,
    /// The exposed variant name. Requires event=experiment.exposure. Comma-separated alternatives are ORed; different filters are ANDed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<String>,
}

impl FunnelStep {
    pub fn builder() -> FunnelStepBuilder {
        <FunnelStepBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FunnelStepBuilder {
    app_build_id: Option<String>,
    app_id: Option<String>,
    browser: Option<String>,
    city: Option<String>,
    country: Option<String>,
    custom_name: Option<String>,
    device: Option<String>,
    event: Option<String>,
    experiment_id: Option<String>,
    hostname: Option<String>,
    os: Option<String>,
    page: Option<String>,
    plan_id: Option<String>,
    product_id: Option<String>,
    source: Option<String>,
    utm_source: Option<String>,
    variant: Option<String>,
}

impl FunnelStepBuilder {
    pub fn app_build_id(mut self, value: impl Into<String>) -> Self {
        self.app_build_id = Some(value.into());
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn browser(mut self, value: impl Into<String>) -> Self {
        self.browser = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn custom_name(mut self, value: impl Into<String>) -> Self {
        self.custom_name = Some(value.into());
        self
    }

    pub fn device(mut self, value: impl Into<String>) -> Self {
        self.device = Some(value.into());
        self
    }

    pub fn event(mut self, value: impl Into<String>) -> Self {
        self.event = Some(value.into());
        self
    }

    pub fn experiment_id(mut self, value: impl Into<String>) -> Self {
        self.experiment_id = Some(value.into());
        self
    }

    pub fn hostname(mut self, value: impl Into<String>) -> Self {
        self.hostname = Some(value.into());
        self
    }

    pub fn os(mut self, value: impl Into<String>) -> Self {
        self.os = Some(value.into());
        self
    }

    pub fn page(mut self, value: impl Into<String>) -> Self {
        self.page = Some(value.into());
        self
    }

    pub fn plan_id(mut self, value: impl Into<String>) -> Self {
        self.plan_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn utm_source(mut self, value: impl Into<String>) -> Self {
        self.utm_source = Some(value.into());
        self
    }

    pub fn variant(mut self, value: impl Into<String>) -> Self {
        self.variant = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`FunnelStep`].
    /// This method will fail if any of the following fields are not set:
    /// - [`event`](FunnelStepBuilder::event)
    pub fn build(self) -> Result<FunnelStep, BuildError> {
        Ok(FunnelStep {
            app_build_id: self.app_build_id,
            app_id: self.app_id,
            browser: self.browser,
            city: self.city,
            country: self.country,
            custom_name: self.custom_name,
            device: self.device,
            event: self
                .event
                .ok_or_else(|| BuildError::missing_field("event"))?,
            experiment_id: self.experiment_id,
            hostname: self.hostname,
            os: self.os,
            page: self.page,
            plan_id: self.plan_id,
            product_id: self.product_id,
            source: self.source,
            utm_source: self.utm_source,
            variant: self.variant,
        })
    }
}
