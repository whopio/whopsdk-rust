pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AccountCustomerTracking {
    /// Accent color, as a `#rrggbb` hex color.
    #[serde(default)]
    pub accent_color: String,
    /// The business name shown on its customer tracking page.
    #[serde(default)]
    pub business_name: String,
    /// `black` or `white`, whichever reads against the accent color.
    pub button_text_color: AccountCustomerTrackingButtonTextColor,
    /// The account logo image URL, or `null` when it has not uploaded one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_url: Option<String>,
    /// Whether to display the business name on the tracking page and Element.
    #[serde(default)]
    pub show_business_name: bool,
    /// Whether to display the logo on the tracking page and Element.
    #[serde(default)]
    pub show_logo: bool,
    /// Customer support email address, or `null` when not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_email: Option<String>,
    /// HTTPS customer support URL, or `null` when not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_url: Option<String>,
}

impl AccountCustomerTracking {
    pub fn builder() -> AccountCustomerTrackingBuilder {
        <AccountCustomerTrackingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AccountCustomerTrackingBuilder {
    accent_color: Option<String>,
    business_name: Option<String>,
    button_text_color: Option<AccountCustomerTrackingButtonTextColor>,
    logo_url: Option<String>,
    show_business_name: Option<bool>,
    show_logo: Option<bool>,
    support_email: Option<String>,
    support_url: Option<String>,
}

impl AccountCustomerTrackingBuilder {
    pub fn accent_color(mut self, value: impl Into<String>) -> Self {
        self.accent_color = Some(value.into());
        self
    }

    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
        self
    }

    pub fn button_text_color(mut self, value: AccountCustomerTrackingButtonTextColor) -> Self {
        self.button_text_color = Some(value);
        self
    }

    pub fn logo_url(mut self, value: impl Into<String>) -> Self {
        self.logo_url = Some(value.into());
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

    /// Consumes the builder and constructs a [`AccountCustomerTracking`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accent_color`](AccountCustomerTrackingBuilder::accent_color)
    /// - [`business_name`](AccountCustomerTrackingBuilder::business_name)
    /// - [`button_text_color`](AccountCustomerTrackingBuilder::button_text_color)
    /// - [`show_business_name`](AccountCustomerTrackingBuilder::show_business_name)
    /// - [`show_logo`](AccountCustomerTrackingBuilder::show_logo)
    pub fn build(self) -> Result<AccountCustomerTracking, BuildError> {
        Ok(AccountCustomerTracking {
            accent_color: self
                .accent_color
                .ok_or_else(|| BuildError::missing_field("accent_color"))?,
            business_name: self
                .business_name
                .ok_or_else(|| BuildError::missing_field("business_name"))?,
            button_text_color: self
                .button_text_color
                .ok_or_else(|| BuildError::missing_field("button_text_color"))?,
            logo_url: self.logo_url,
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
