pub use crate::prelude::*;

/// Types of optimization results tracked from external ad platforms
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResultLabelKeys {
    AppInstalls,
    MessagingConversationsStarted,
    PostEngagement,
    EventResponses,
    Impressions,
    WebsitePurchases,
    LandingPageViews,
    Leads,
    LinkClicks,
    QualityCalls,
    AppointmentsBooked,
    MessagingPurchases,
    PageLikes,
    InstagramProfileVisits,
    Reach,
    RemindersSet,
    NewSubscribers,
    VideoViews,
    Registrations,
    ContentViews,
    Searches,
    AddsToCart,
    AddsToWishlist,
    AddsOfPaymentInfo,
    CheckoutsInitiated,
    WebsiteSchedules,
    WebsiteSubmitApplications,
    WebsiteTrialsStarted,
    WebsiteSubscriptions,
    WebsiteContacts,
    WebsiteDonations,
    WebsiteFindLocations,
    WebsiteProductCustomizations,
    Custom,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for ResultLabelKeys {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AppInstalls => serializer.serialize_str("app_installs"),
            Self::MessagingConversationsStarted => {
                serializer.serialize_str("messaging_conversations_started")
            }
            Self::PostEngagement => serializer.serialize_str("post_engagement"),
            Self::EventResponses => serializer.serialize_str("event_responses"),
            Self::Impressions => serializer.serialize_str("impressions"),
            Self::WebsitePurchases => serializer.serialize_str("website_purchases"),
            Self::LandingPageViews => serializer.serialize_str("landing_page_views"),
            Self::Leads => serializer.serialize_str("leads"),
            Self::LinkClicks => serializer.serialize_str("link_clicks"),
            Self::QualityCalls => serializer.serialize_str("quality_calls"),
            Self::AppointmentsBooked => serializer.serialize_str("appointments_booked"),
            Self::MessagingPurchases => serializer.serialize_str("messaging_purchases"),
            Self::PageLikes => serializer.serialize_str("page_likes"),
            Self::InstagramProfileVisits => serializer.serialize_str("instagram_profile_visits"),
            Self::Reach => serializer.serialize_str("reach"),
            Self::RemindersSet => serializer.serialize_str("reminders_set"),
            Self::NewSubscribers => serializer.serialize_str("new_subscribers"),
            Self::VideoViews => serializer.serialize_str("video_views"),
            Self::Registrations => serializer.serialize_str("registrations"),
            Self::ContentViews => serializer.serialize_str("content_views"),
            Self::Searches => serializer.serialize_str("searches"),
            Self::AddsToCart => serializer.serialize_str("adds_to_cart"),
            Self::AddsToWishlist => serializer.serialize_str("adds_to_wishlist"),
            Self::AddsOfPaymentInfo => serializer.serialize_str("adds_of_payment_info"),
            Self::CheckoutsInitiated => serializer.serialize_str("checkouts_initiated"),
            Self::WebsiteSchedules => serializer.serialize_str("website_schedules"),
            Self::WebsiteSubmitApplications => {
                serializer.serialize_str("website_submit_applications")
            }
            Self::WebsiteTrialsStarted => serializer.serialize_str("website_trials_started"),
            Self::WebsiteSubscriptions => serializer.serialize_str("website_subscriptions"),
            Self::WebsiteContacts => serializer.serialize_str("website_contacts"),
            Self::WebsiteDonations => serializer.serialize_str("website_donations"),
            Self::WebsiteFindLocations => serializer.serialize_str("website_find_locations"),
            Self::WebsiteProductCustomizations => {
                serializer.serialize_str("website_product_customizations")
            }
            Self::Custom => serializer.serialize_str("custom"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for ResultLabelKeys {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "app_installs" => Ok(Self::AppInstalls),
            "messaging_conversations_started" => Ok(Self::MessagingConversationsStarted),
            "post_engagement" => Ok(Self::PostEngagement),
            "event_responses" => Ok(Self::EventResponses),
            "impressions" => Ok(Self::Impressions),
            "website_purchases" => Ok(Self::WebsitePurchases),
            "landing_page_views" => Ok(Self::LandingPageViews),
            "leads" => Ok(Self::Leads),
            "link_clicks" => Ok(Self::LinkClicks),
            "quality_calls" => Ok(Self::QualityCalls),
            "appointments_booked" => Ok(Self::AppointmentsBooked),
            "messaging_purchases" => Ok(Self::MessagingPurchases),
            "page_likes" => Ok(Self::PageLikes),
            "instagram_profile_visits" => Ok(Self::InstagramProfileVisits),
            "reach" => Ok(Self::Reach),
            "reminders_set" => Ok(Self::RemindersSet),
            "new_subscribers" => Ok(Self::NewSubscribers),
            "video_views" => Ok(Self::VideoViews),
            "registrations" => Ok(Self::Registrations),
            "content_views" => Ok(Self::ContentViews),
            "searches" => Ok(Self::Searches),
            "adds_to_cart" => Ok(Self::AddsToCart),
            "adds_to_wishlist" => Ok(Self::AddsToWishlist),
            "adds_of_payment_info" => Ok(Self::AddsOfPaymentInfo),
            "checkouts_initiated" => Ok(Self::CheckoutsInitiated),
            "website_schedules" => Ok(Self::WebsiteSchedules),
            "website_submit_applications" => Ok(Self::WebsiteSubmitApplications),
            "website_trials_started" => Ok(Self::WebsiteTrialsStarted),
            "website_subscriptions" => Ok(Self::WebsiteSubscriptions),
            "website_contacts" => Ok(Self::WebsiteContacts),
            "website_donations" => Ok(Self::WebsiteDonations),
            "website_find_locations" => Ok(Self::WebsiteFindLocations),
            "website_product_customizations" => Ok(Self::WebsiteProductCustomizations),
            "custom" => Ok(Self::Custom),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for ResultLabelKeys {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AppInstalls => write!(f, "app_installs"),
            Self::MessagingConversationsStarted => write!(f, "messaging_conversations_started"),
            Self::PostEngagement => write!(f, "post_engagement"),
            Self::EventResponses => write!(f, "event_responses"),
            Self::Impressions => write!(f, "impressions"),
            Self::WebsitePurchases => write!(f, "website_purchases"),
            Self::LandingPageViews => write!(f, "landing_page_views"),
            Self::Leads => write!(f, "leads"),
            Self::LinkClicks => write!(f, "link_clicks"),
            Self::QualityCalls => write!(f, "quality_calls"),
            Self::AppointmentsBooked => write!(f, "appointments_booked"),
            Self::MessagingPurchases => write!(f, "messaging_purchases"),
            Self::PageLikes => write!(f, "page_likes"),
            Self::InstagramProfileVisits => write!(f, "instagram_profile_visits"),
            Self::Reach => write!(f, "reach"),
            Self::RemindersSet => write!(f, "reminders_set"),
            Self::NewSubscribers => write!(f, "new_subscribers"),
            Self::VideoViews => write!(f, "video_views"),
            Self::Registrations => write!(f, "registrations"),
            Self::ContentViews => write!(f, "content_views"),
            Self::Searches => write!(f, "searches"),
            Self::AddsToCart => write!(f, "adds_to_cart"),
            Self::AddsToWishlist => write!(f, "adds_to_wishlist"),
            Self::AddsOfPaymentInfo => write!(f, "adds_of_payment_info"),
            Self::CheckoutsInitiated => write!(f, "checkouts_initiated"),
            Self::WebsiteSchedules => write!(f, "website_schedules"),
            Self::WebsiteSubmitApplications => write!(f, "website_submit_applications"),
            Self::WebsiteTrialsStarted => write!(f, "website_trials_started"),
            Self::WebsiteSubscriptions => write!(f, "website_subscriptions"),
            Self::WebsiteContacts => write!(f, "website_contacts"),
            Self::WebsiteDonations => write!(f, "website_donations"),
            Self::WebsiteFindLocations => write!(f, "website_find_locations"),
            Self::WebsiteProductCustomizations => write!(f, "website_product_customizations"),
            Self::Custom => write!(f, "custom"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
