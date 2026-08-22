pub use crate::prelude::*;

/// Ways this post can't be used on an ad. `lead_form_ineligible`: post can't be used in lead form ads. `promotion_ineligible`: the platform won't promote this post at all — always present when that's true, alongside a reason code when one is identified. `copyrighted_music`: post uses music the platform does not allow for ads. `messenger_destination`, `instagram_destination`, `whatsapp_destination`: the post's own button opens that chat app, so the post can only run in an ad group that sends people to the same one — always accompanied by `lead_form_ineligible`. Empty when the post has no restrictions.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SocialAccountPostRestrictionsItem {
    LeadFormIneligible,
    PromotionIneligible,
    CopyrightedMusic,
    MessengerDestination,
    InstagramDestination,
    WhatsappDestination,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SocialAccountPostRestrictionsItem {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::LeadFormIneligible => serializer.serialize_str("lead_form_ineligible"),
            Self::PromotionIneligible => serializer.serialize_str("promotion_ineligible"),
            Self::CopyrightedMusic => serializer.serialize_str("copyrighted_music"),
            Self::MessengerDestination => serializer.serialize_str("messenger_destination"),
            Self::InstagramDestination => serializer.serialize_str("instagram_destination"),
            Self::WhatsappDestination => serializer.serialize_str("whatsapp_destination"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SocialAccountPostRestrictionsItem {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "lead_form_ineligible" => Ok(Self::LeadFormIneligible),
            "promotion_ineligible" => Ok(Self::PromotionIneligible),
            "copyrighted_music" => Ok(Self::CopyrightedMusic),
            "messenger_destination" => Ok(Self::MessengerDestination),
            "instagram_destination" => Ok(Self::InstagramDestination),
            "whatsapp_destination" => Ok(Self::WhatsappDestination),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SocialAccountPostRestrictionsItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeadFormIneligible => write!(f, "lead_form_ineligible"),
            Self::PromotionIneligible => write!(f, "promotion_ineligible"),
            Self::CopyrightedMusic => write!(f, "copyrighted_music"),
            Self::MessengerDestination => write!(f, "messenger_destination"),
            Self::InstagramDestination => write!(f, "instagram_destination"),
            Self::WhatsappDestination => write!(f, "whatsapp_destination"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
