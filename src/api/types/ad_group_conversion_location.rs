pub use crate::prelude::*;

/// Where the outcome being optimized for occurs, such as a website visit, social-profile visit, messaging conversation, ad interaction, or lead-form submission.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AdGroupConversionLocation {
    Website,
    Profile,
    InstagramAndFacebook,
    InstagramProfile,
    Messaging,
    OnAd,
    InstantForms,
    InstantFormsAndMessenger,
    WebsiteAndInstantForms,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AdGroupConversionLocation {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Website => serializer.serialize_str("website"),
            Self::Profile => serializer.serialize_str("profile"),
            Self::InstagramAndFacebook => serializer.serialize_str("instagram_and_facebook"),
            Self::InstagramProfile => serializer.serialize_str("instagram_profile"),
            Self::Messaging => serializer.serialize_str("messaging"),
            Self::OnAd => serializer.serialize_str("on_ad"),
            Self::InstantForms => serializer.serialize_str("instant_forms"),
            Self::InstantFormsAndMessenger => {
                serializer.serialize_str("instant_forms_and_messenger")
            }
            Self::WebsiteAndInstantForms => serializer.serialize_str("website_and_instant_forms"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AdGroupConversionLocation {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "website" => Ok(Self::Website),
            "profile" => Ok(Self::Profile),
            "instagram_and_facebook" => Ok(Self::InstagramAndFacebook),
            "instagram_profile" => Ok(Self::InstagramProfile),
            "messaging" => Ok(Self::Messaging),
            "on_ad" => Ok(Self::OnAd),
            "instant_forms" => Ok(Self::InstantForms),
            "instant_forms_and_messenger" => Ok(Self::InstantFormsAndMessenger),
            "website_and_instant_forms" => Ok(Self::WebsiteAndInstantForms),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AdGroupConversionLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Website => write!(f, "website"),
            Self::Profile => write!(f, "profile"),
            Self::InstagramAndFacebook => write!(f, "instagram_and_facebook"),
            Self::InstagramProfile => write!(f, "instagram_profile"),
            Self::Messaging => write!(f, "messaging"),
            Self::OnAd => write!(f, "on_ad"),
            Self::InstantForms => write!(f, "instant_forms"),
            Self::InstantFormsAndMessenger => write!(f, "instant_forms_and_messenger"),
            Self::WebsiteAndInstantForms => write!(f, "website_and_instant_forms"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
