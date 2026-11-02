pub use crate::prelude::*;

/// Interaction that qualifies a person for this rule.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AudienceEngagementInstagramProfileRuleEvent {
    All,
    Engaged,
    Visited,
    Messaged,
    Saved,
    AdLiked,
    AdCommented,
    AdShared,
    AdSaved,
    AdCtaClicked,
    AdCarouselSwiped,
    OrganicLiked,
    OrganicCommented,
    OrganicShared,
    OrganicSaved,
    OrganicSwiped,
    OrganicCarouselSwiped,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for AudienceEngagementInstagramProfileRuleEvent {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::All => serializer.serialize_str("all"),
            Self::Engaged => serializer.serialize_str("engaged"),
            Self::Visited => serializer.serialize_str("visited"),
            Self::Messaged => serializer.serialize_str("messaged"),
            Self::Saved => serializer.serialize_str("saved"),
            Self::AdLiked => serializer.serialize_str("ad_liked"),
            Self::AdCommented => serializer.serialize_str("ad_commented"),
            Self::AdShared => serializer.serialize_str("ad_shared"),
            Self::AdSaved => serializer.serialize_str("ad_saved"),
            Self::AdCtaClicked => serializer.serialize_str("ad_cta_clicked"),
            Self::AdCarouselSwiped => serializer.serialize_str("ad_carousel_swiped"),
            Self::OrganicLiked => serializer.serialize_str("organic_liked"),
            Self::OrganicCommented => serializer.serialize_str("organic_commented"),
            Self::OrganicShared => serializer.serialize_str("organic_shared"),
            Self::OrganicSaved => serializer.serialize_str("organic_saved"),
            Self::OrganicSwiped => serializer.serialize_str("organic_swiped"),
            Self::OrganicCarouselSwiped => serializer.serialize_str("organic_carousel_swiped"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for AudienceEngagementInstagramProfileRuleEvent {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "all" => Ok(Self::All),
            "engaged" => Ok(Self::Engaged),
            "visited" => Ok(Self::Visited),
            "messaged" => Ok(Self::Messaged),
            "saved" => Ok(Self::Saved),
            "ad_liked" => Ok(Self::AdLiked),
            "ad_commented" => Ok(Self::AdCommented),
            "ad_shared" => Ok(Self::AdShared),
            "ad_saved" => Ok(Self::AdSaved),
            "ad_cta_clicked" => Ok(Self::AdCtaClicked),
            "ad_carousel_swiped" => Ok(Self::AdCarouselSwiped),
            "organic_liked" => Ok(Self::OrganicLiked),
            "organic_commented" => Ok(Self::OrganicCommented),
            "organic_shared" => Ok(Self::OrganicShared),
            "organic_saved" => Ok(Self::OrganicSaved),
            "organic_swiped" => Ok(Self::OrganicSwiped),
            "organic_carousel_swiped" => Ok(Self::OrganicCarouselSwiped),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for AudienceEngagementInstagramProfileRuleEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::All => write!(f, "all"),
            Self::Engaged => write!(f, "engaged"),
            Self::Visited => write!(f, "visited"),
            Self::Messaged => write!(f, "messaged"),
            Self::Saved => write!(f, "saved"),
            Self::AdLiked => write!(f, "ad_liked"),
            Self::AdCommented => write!(f, "ad_commented"),
            Self::AdShared => write!(f, "ad_shared"),
            Self::AdSaved => write!(f, "ad_saved"),
            Self::AdCtaClicked => write!(f, "ad_cta_clicked"),
            Self::AdCarouselSwiped => write!(f, "ad_carousel_swiped"),
            Self::OrganicLiked => write!(f, "organic_liked"),
            Self::OrganicCommented => write!(f, "organic_commented"),
            Self::OrganicShared => write!(f, "organic_shared"),
            Self::OrganicSaved => write!(f, "organic_saved"),
            Self::OrganicSwiped => write!(f, "organic_swiped"),
            Self::OrganicCarouselSwiped => write!(f, "organic_carousel_swiped"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
