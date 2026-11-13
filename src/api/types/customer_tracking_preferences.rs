pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomerTrackingPreferences {
    /// Accent color, as a `#rrggbb` hex color. Defaults to `#1d4ed8`.
    #[serde(default)]
    pub accent_color: String,
    /// Whether the public tracking page and guest lookup are available. Defaults to false.
    #[serde(default)]
    pub enabled: bool,
    /// Whether to display the business name on the tracking page and Element. Defaults to true.
    #[serde(default)]
    pub show_business_name: bool,
    /// Whether to display the logo on the tracking page and Element. Defaults to true.
    #[serde(default)]
    pub show_logo: bool,
    /// Customer support email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// HTTPS customer support URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
}

impl CustomerTrackingPreferences {
    pub fn builder() -> CustomerTrackingPreferencesBuilder {
        <CustomerTrackingPreferencesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerTrackingPreferencesBuilder {
    accent_color: Option<String>,
    enabled: Option<bool>,
    show_business_name: Option<bool>,
    show_logo: Option<bool>,
    support_email: Option<String>,
    support_url: Option<String>,
}

impl CustomerTrackingPreferencesBuilder {
    pub fn accent_color(mut self, value: impl Into<String>) -> Self {
        self.accent_color = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn show_business_name(mut self, value: bool) -> Self {
        self.show_business_name = Some(value);
        self
    }

    pub fn show_logo(mut self, value: bool) -> Self {
        self.show_logo = Some(value);
        self
    }

    pub fn support_email(mut self, value: impl Into<String>) -> Self {
        self.support_email = Some(value.into());
        self
    }

    pub fn support_url(mut self, value: impl Into<String>) -> Self {
        self.support_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CustomerTrackingPreferences`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accent_color`](CustomerTrackingPreferencesBuilder::accent_color)
    /// - [`enabled`](CustomerTrackingPreferencesBuilder::enabled)
    /// - [`show_business_name`](CustomerTrackingPreferencesBuilder::show_business_name)
    /// - [`show_logo`](CustomerTrackingPreferencesBuilder::show_logo)
    pub fn build(self) -> Result<CustomerTrackingPreferences, BuildError> {
        Ok(CustomerTrackingPreferences {
            accent_color: self
                .accent_color
                .ok_or_else(|| BuildError::missing_field("accent_color"))?,
            enabled: self
                .enabled
                .ok_or_else(|| BuildError::missing_field("enabled"))?,
            show_business_name: self
                .show_business_name
                .ok_or_else(|| BuildError::missing_field("show_business_name"))?,
            show_logo: self
                .show_logo
                .ok_or_else(|| BuildError::missing_field("show_logo"))?,
            support_email: self.support_email,
            support_url: self.support_url,
        })
    }
}
