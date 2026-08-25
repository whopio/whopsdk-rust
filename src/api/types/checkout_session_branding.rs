pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutSessionBranding {
    /// Page background, as a `#rrggbb` hex color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// `black` or `white`, whichever reads against the background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_text_color: Option<String>,
    /// The corner style the seller chose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_style: Option<CheckoutSessionBrandingBorderStyle>,
    /// Pay button fill, as a `#rrggbb` hex color.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_color: Option<String>,
    /// The CSS `border-radius` for buttons at that border style.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_radius: Option<String>,
    /// `black` or `white`, whichever reads against the button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_text_color: Option<String>,
    /// The CSS `border-radius` for containers at that border style.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_radius: Option<String>,
    /// The font the seller chose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_family: Option<CheckoutSessionBrandingFontFamily>,
    /// The stylesheet to load for that font, or `null` for the system font.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_url: Option<String>,
    /// The CSS `border-radius` for inputs at that border style.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_radius: Option<String>,
    /// The CSS `font-family` value for that font, or `null` for the system font.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_font_family: Option<String>,
}

impl CheckoutSessionBranding {
    pub fn builder() -> CheckoutSessionBrandingBuilder {
        <CheckoutSessionBrandingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionBrandingBuilder {
    background_color: Option<String>,
    background_text_color: Option<String>,
    border_style: Option<CheckoutSessionBrandingBorderStyle>,
    button_color: Option<String>,
    button_radius: Option<String>,
    button_text_color: Option<String>,
    container_radius: Option<String>,
    font_family: Option<CheckoutSessionBrandingFontFamily>,
    font_url: Option<String>,
    input_radius: Option<String>,
    resolved_font_family: Option<String>,
}

impl CheckoutSessionBrandingBuilder {
    pub fn background_color(mut self, value: impl Into<String>) -> Self {
        self.background_color = Some(value.into());
        self
    }

    pub fn background_text_color(mut self, value: impl Into<String>) -> Self {
        self.background_text_color = Some(value.into());
        self
    }

    pub fn border_style(mut self, value: CheckoutSessionBrandingBorderStyle) -> Self {
        self.border_style = Some(value);
        self
    }

    pub fn button_color(mut self, value: impl Into<String>) -> Self {
        self.button_color = Some(value.into());
        self
    }

    pub fn button_radius(mut self, value: impl Into<String>) -> Self {
        self.button_radius = Some(value.into());
        self
    }

    pub fn button_text_color(mut self, value: impl Into<String>) -> Self {
        self.button_text_color = Some(value.into());
        self
    }

    pub fn container_radius(mut self, value: impl Into<String>) -> Self {
        self.container_radius = Some(value.into());
        self
    }

    pub fn font_family(mut self, value: CheckoutSessionBrandingFontFamily) -> Self {
        self.font_family = Some(value);
        self
    }

    pub fn font_url(mut self, value: impl Into<String>) -> Self {
        self.font_url = Some(value.into());
        self
    }

    pub fn input_radius(mut self, value: impl Into<String>) -> Self {
        self.input_radius = Some(value.into());
        self
    }

    pub fn resolved_font_family(mut self, value: impl Into<String>) -> Self {
        self.resolved_font_family = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionBranding`].
    pub fn build(self) -> Result<CheckoutSessionBranding, BuildError> {
        Ok(CheckoutSessionBranding {
            background_color: self.background_color,
            background_text_color: self.background_text_color,
            border_style: self.border_style,
            button_color: self.button_color,
            button_radius: self.button_radius,
            button_text_color: self.button_text_color,
            container_radius: self.container_radius,
            font_family: self.font_family,
            font_url: self.font_url,
            input_radius: self.input_radius,
            resolved_font_family: self.resolved_font_family,
        })
    }
}
