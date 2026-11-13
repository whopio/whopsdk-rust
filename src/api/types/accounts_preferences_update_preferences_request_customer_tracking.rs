pub use crate::prelude::*;

/// Customer tracking settings for the hosted tracking page and Element, replaced as a whole: fields left out are reset to their defaults. Pass null to clear them, which turns tracking off. Requires the `company:manage_checkout` scope on your API key.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdatePreferencesRequestCustomerTracking {
    /// Accent color, as a `#rrggbb` hex color. Omitted or null resets to `#1d4ed8`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
    /// Turns the public tracking page and guest lookup on. Omitted or null resets to false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// Show the business name on the tracking page and Element. Omitted or null resets to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_business_name: Option<bool>,
    /// Show the account logo on the tracking page and Element. Omitted or null resets to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_logo: Option<bool>,
    /// Customer support email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// HTTPS customer support URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
}

impl UpdatePreferencesRequestCustomerTracking {
    pub fn builder() -> UpdatePreferencesRequestCustomerTrackingBuilder {
        <UpdatePreferencesRequestCustomerTrackingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePreferencesRequestCustomerTrackingBuilder {
    accent_color: Option<String>,
    enabled: Option<bool>,
    show_business_name: Option<bool>,
    show_logo: Option<bool>,
    support_email: Option<String>,
    support_url: Option<String>,
}

impl UpdatePreferencesRequestCustomerTrackingBuilder {
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

    /// Consumes the builder and constructs a [`UpdatePreferencesRequestCustomerTracking`].
    pub fn build(self) -> Result<UpdatePreferencesRequestCustomerTracking, BuildError> {
        Ok(UpdatePreferencesRequestCustomerTracking {
            accent_color: self.accent_color,
            enabled: self.enabled,
            show_business_name: self.show_business_name,
            show_logo: self.show_logo,
            support_email: self.support_email,
            support_url: self.support_url,
        })
    }
}
