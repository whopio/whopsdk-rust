pub use crate::prelude::*;

/// What stopped the confirm. The session: `session_expired`, `session_completed`, `confirm_in_progress`, `attestation_required`. The price: `quote_expired` (already refreshed), `quote_changed` (the stored quote is not what this confirm would charge — the buyer who resolved at confirm prices differently, e.g. a returning buyer skipped past the plan's free trial; the session is already re-quoted, so re-read it, show the new total, and confirm again), `rate_expired` (the exchange rate behind a local-currency price aged out — re-read the session for a fresh one). The payment method: `token_invalid`, `payment_method_unsupported`, `payment_method_unavailable` (a saved method the token names no longer exists in the buyer's wallet — collect a fresh one). What is being sold, and whether it may still be sold: `seller_unavailable`, `promo_invalid`, `shipping_address_required`, `custom_field_invalid` (a required question is unanswered, or an answer names a question this product does not ask), `plan_archived`, `product_archived`, `out_of_stock`, `purchases_disabled`, `blocked_country`, `custom_password` (the plan's purchase password is unanswered — update the session's `password` and confirm again), `waitlist_unavailable` (this waitlist cannot be joined as asked — the message says why). The buyer: `user_suspended`, `user_banned_from_whop`, `email_required`, `free_plan_already_owned`, `waitlist_entry_pending`, `waitlist_already_member`, and `buyer_ineligible` — an account-state refusal read by someone who never proved they hold that account, so which state stays undisclosed. The payment itself: `payment_failed`. And `engine_unavailable` for a session whose mode this checkout cannot charge yet. Every one of these is terminal for this attempt as it stands — a refusal the buyer can resolve by doing something arrives as `next_action` instead.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CheckoutSessionConfirmErrorCode {
    SessionExpired,
    SessionCompleted,
    AttestationRequired,
    QuoteExpired,
    QuoteChanged,
    RateExpired,
    TokenInvalid,
    ConfirmInProgress,
    EngineUnavailable,
    SellerUnavailable,
    PromoInvalid,
    PaymentMethodUnsupported,
    PaymentMethodUnavailable,
    ShippingAddressRequired,
    CustomFieldInvalid,
    PaymentFailed,
    WaitlistUnavailable,
    PurchasesDisabled,
    PlanArchived,
    ProductArchived,
    OutOfStock,
    CustomPassword,
    BlockedCountry,
    UserSuspended,
    UserBannedFromWhop,
    EmailRequired,
    FreePlanAlreadyOwned,
    WaitlistEntryPending,
    WaitlistAlreadyMember,
    BuyerIneligible,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CheckoutSessionConfirmErrorCode {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::SessionExpired => serializer.serialize_str("session_expired"),
            Self::SessionCompleted => serializer.serialize_str("session_completed"),
            Self::AttestationRequired => serializer.serialize_str("attestation_required"),
            Self::QuoteExpired => serializer.serialize_str("quote_expired"),
            Self::QuoteChanged => serializer.serialize_str("quote_changed"),
            Self::RateExpired => serializer.serialize_str("rate_expired"),
            Self::TokenInvalid => serializer.serialize_str("token_invalid"),
            Self::ConfirmInProgress => serializer.serialize_str("confirm_in_progress"),
            Self::EngineUnavailable => serializer.serialize_str("engine_unavailable"),
            Self::SellerUnavailable => serializer.serialize_str("seller_unavailable"),
            Self::PromoInvalid => serializer.serialize_str("promo_invalid"),
            Self::PaymentMethodUnsupported => {
                serializer.serialize_str("payment_method_unsupported")
            }
            Self::PaymentMethodUnavailable => {
                serializer.serialize_str("payment_method_unavailable")
            }
            Self::ShippingAddressRequired => serializer.serialize_str("shipping_address_required"),
            Self::CustomFieldInvalid => serializer.serialize_str("custom_field_invalid"),
            Self::PaymentFailed => serializer.serialize_str("payment_failed"),
            Self::WaitlistUnavailable => serializer.serialize_str("waitlist_unavailable"),
            Self::PurchasesDisabled => serializer.serialize_str("purchases_disabled"),
            Self::PlanArchived => serializer.serialize_str("plan_archived"),
            Self::ProductArchived => serializer.serialize_str("product_archived"),
            Self::OutOfStock => serializer.serialize_str("out_of_stock"),
            Self::CustomPassword => serializer.serialize_str("custom_password"),
            Self::BlockedCountry => serializer.serialize_str("blocked_country"),
            Self::UserSuspended => serializer.serialize_str("user_suspended"),
            Self::UserBannedFromWhop => serializer.serialize_str("user_banned_from_whop"),
            Self::EmailRequired => serializer.serialize_str("email_required"),
            Self::FreePlanAlreadyOwned => serializer.serialize_str("free_plan_already_owned"),
            Self::WaitlistEntryPending => serializer.serialize_str("waitlist_entry_pending"),
            Self::WaitlistAlreadyMember => serializer.serialize_str("waitlist_already_member"),
            Self::BuyerIneligible => serializer.serialize_str("buyer_ineligible"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CheckoutSessionConfirmErrorCode {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "session_expired" => Ok(Self::SessionExpired),
            "session_completed" => Ok(Self::SessionCompleted),
            "attestation_required" => Ok(Self::AttestationRequired),
            "quote_expired" => Ok(Self::QuoteExpired),
            "quote_changed" => Ok(Self::QuoteChanged),
            "rate_expired" => Ok(Self::RateExpired),
            "token_invalid" => Ok(Self::TokenInvalid),
            "confirm_in_progress" => Ok(Self::ConfirmInProgress),
            "engine_unavailable" => Ok(Self::EngineUnavailable),
            "seller_unavailable" => Ok(Self::SellerUnavailable),
            "promo_invalid" => Ok(Self::PromoInvalid),
            "payment_method_unsupported" => Ok(Self::PaymentMethodUnsupported),
            "payment_method_unavailable" => Ok(Self::PaymentMethodUnavailable),
            "shipping_address_required" => Ok(Self::ShippingAddressRequired),
            "custom_field_invalid" => Ok(Self::CustomFieldInvalid),
            "payment_failed" => Ok(Self::PaymentFailed),
            "waitlist_unavailable" => Ok(Self::WaitlistUnavailable),
            "purchases_disabled" => Ok(Self::PurchasesDisabled),
            "plan_archived" => Ok(Self::PlanArchived),
            "product_archived" => Ok(Self::ProductArchived),
            "out_of_stock" => Ok(Self::OutOfStock),
            "custom_password" => Ok(Self::CustomPassword),
            "blocked_country" => Ok(Self::BlockedCountry),
            "user_suspended" => Ok(Self::UserSuspended),
            "user_banned_from_whop" => Ok(Self::UserBannedFromWhop),
            "email_required" => Ok(Self::EmailRequired),
            "free_plan_already_owned" => Ok(Self::FreePlanAlreadyOwned),
            "waitlist_entry_pending" => Ok(Self::WaitlistEntryPending),
            "waitlist_already_member" => Ok(Self::WaitlistAlreadyMember),
            "buyer_ineligible" => Ok(Self::BuyerIneligible),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CheckoutSessionConfirmErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SessionExpired => write!(f, "session_expired"),
            Self::SessionCompleted => write!(f, "session_completed"),
            Self::AttestationRequired => write!(f, "attestation_required"),
            Self::QuoteExpired => write!(f, "quote_expired"),
            Self::QuoteChanged => write!(f, "quote_changed"),
            Self::RateExpired => write!(f, "rate_expired"),
            Self::TokenInvalid => write!(f, "token_invalid"),
            Self::ConfirmInProgress => write!(f, "confirm_in_progress"),
            Self::EngineUnavailable => write!(f, "engine_unavailable"),
            Self::SellerUnavailable => write!(f, "seller_unavailable"),
            Self::PromoInvalid => write!(f, "promo_invalid"),
            Self::PaymentMethodUnsupported => write!(f, "payment_method_unsupported"),
            Self::PaymentMethodUnavailable => write!(f, "payment_method_unavailable"),
            Self::ShippingAddressRequired => write!(f, "shipping_address_required"),
            Self::CustomFieldInvalid => write!(f, "custom_field_invalid"),
            Self::PaymentFailed => write!(f, "payment_failed"),
            Self::WaitlistUnavailable => write!(f, "waitlist_unavailable"),
            Self::PurchasesDisabled => write!(f, "purchases_disabled"),
            Self::PlanArchived => write!(f, "plan_archived"),
            Self::ProductArchived => write!(f, "product_archived"),
            Self::OutOfStock => write!(f, "out_of_stock"),
            Self::CustomPassword => write!(f, "custom_password"),
            Self::BlockedCountry => write!(f, "blocked_country"),
            Self::UserSuspended => write!(f, "user_suspended"),
            Self::UserBannedFromWhop => write!(f, "user_banned_from_whop"),
            Self::EmailRequired => write!(f, "email_required"),
            Self::FreePlanAlreadyOwned => write!(f, "free_plan_already_owned"),
            Self::WaitlistEntryPending => write!(f, "waitlist_entry_pending"),
            Self::WaitlistAlreadyMember => write!(f, "waitlist_already_member"),
            Self::BuyerIneligible => write!(f, "buyer_ineligible"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
