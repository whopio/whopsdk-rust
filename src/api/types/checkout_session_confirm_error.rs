pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionConfirmError {
    /// What stopped the confirm. The session: `session_expired`, `session_completed`, `confirm_in_progress`, `attestation_required`. The price: `quote_expired` (already refreshed), `quote_changed` (the stored quote is not what this confirm would charge — the buyer who resolved at confirm prices differently, e.g. a returning buyer skipped past the plan's free trial; the session is already re-quoted, so re-read it, show the new total, and confirm again), `rate_expired` (the exchange rate behind a local-currency price aged out — re-read the session for a fresh one). The payment method: `token_invalid`, `payment_method_unsupported`, `payment_method_unavailable` (a saved method the token names no longer exists in the buyer's wallet — collect a fresh one). What is being sold, and whether it may still be sold: `seller_unavailable`, `promo_invalid`, `shipping_address_required`, `custom_field_invalid` (a required question is unanswered, or an answer names a question this product does not ask), `plan_archived`, `product_archived`, `out_of_stock`, `purchases_disabled`, `blocked_country`, `custom_password` (the plan's purchase password is unanswered — update the session's `password` and confirm again), `waitlist_unavailable` (this waitlist cannot be joined as asked — the message says why). The buyer: `user_suspended`, `user_banned_from_whop`, `email_required`, `free_plan_already_owned`, `waitlist_entry_pending`, `waitlist_already_member`, and `buyer_ineligible` — an account-state refusal read by someone who never proved they hold that account, so which state stays undisclosed. The payment itself: `payment_failed`. And `engine_unavailable` for a session whose mode this checkout cannot charge yet. Every one of these is terminal for this attempt as it stands — a refusal the buyer can resolve by doing something arrives as `next_action` instead.
    pub code: CheckoutSessionConfirmErrorCode,
    /// A human-readable explanation of the failure, safe to show the buyer.
    #[serde(default)]
    pub message: String,
}

impl CheckoutSessionConfirmError {
    pub fn builder() -> CheckoutSessionConfirmErrorBuilder {
        <CheckoutSessionConfirmErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionConfirmErrorBuilder {
    code: Option<CheckoutSessionConfirmErrorCode>,
    message: Option<String>,
}

impl CheckoutSessionConfirmErrorBuilder {
    pub fn code(mut self, value: CheckoutSessionConfirmErrorCode) -> Self {
        self.code = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionConfirmError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CheckoutSessionConfirmErrorBuilder::code)
    /// - [`message`](CheckoutSessionConfirmErrorBuilder::message)
    pub fn build(self) -> Result<CheckoutSessionConfirmError, BuildError> {
        Ok(CheckoutSessionConfirmError {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
