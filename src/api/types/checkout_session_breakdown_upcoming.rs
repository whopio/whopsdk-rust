pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum CheckoutSessionBreakdownUpcoming {
    #[serde(rename = "installments")]
    #[non_exhaustive]
    Installments {
        #[serde(default)]
        amount: Money,
        #[serde(default)]
        charge_at: String,
        #[serde(default)]
        description: String,
        #[serde(default)]
        interval_days: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        post_promo_amount: Option<Money>,
        #[serde(default)]
        remaining: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        trial_days: Option<i64>,
    },

    #[serde(rename = "one_time")]
    #[non_exhaustive]
    OneTime {
        #[serde(default)]
        amount: Money,
        #[serde(default)]
        charge_at: String,
        #[serde(default)]
        description: String,
    },

    #[serde(rename = "recurring")]
    #[non_exhaustive]
    Recurring {
        #[serde(default)]
        amount: Money,
        #[serde(default)]
        charge_at: String,
        #[serde(default)]
        description: String,
        #[serde(default)]
        interval_days: i64,
        #[serde(skip_serializing_if = "Option::is_none")]
        post_promo_amount: Option<Money>,
        #[serde(skip_serializing_if = "Option::is_none")]
        trial_days: Option<i64>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CheckoutSessionBreakdownUpcoming {
    pub fn installments(
        amount: Money,
        charge_at: String,
        description: String,
        interval_days: i64,
        remaining: i64,
    ) -> Self {
        Self::Installments {
            amount,
            charge_at,
            description,
            interval_days,
            post_promo_amount: None,
            remaining,
            trial_days: None,
        }
    }

    pub fn one_time(amount: Money, charge_at: String, description: String) -> Self {
        Self::OneTime {
            amount,
            charge_at,
            description,
        }
    }

    pub fn recurring(
        amount: Money,
        charge_at: String,
        description: String,
        interval_days: i64,
    ) -> Self {
        Self::Recurring {
            amount,
            charge_at,
            description,
            interval_days,
            post_promo_amount: None,
            trial_days: None,
        }
    }

    pub fn installments_with_post_promo_amount(
        amount: Money,
        charge_at: String,
        description: String,
        interval_days: i64,
        post_promo_amount: Money,
        remaining: i64,
        trial_days: Option<i64>,
    ) -> Self {
        Self::Installments {
            amount,
            charge_at,
            description,
            interval_days,
            post_promo_amount: Some(post_promo_amount),
            remaining,
            trial_days,
        }
    }

    pub fn installments_with_trial_days(
        amount: Money,
        charge_at: String,
        description: String,
        interval_days: i64,
        post_promo_amount: Option<Money>,
        remaining: i64,
        trial_days: i64,
    ) -> Self {
        Self::Installments {
            amount,
            charge_at,
            description,
            interval_days,
            post_promo_amount,
            remaining,
            trial_days: Some(trial_days),
        }
    }

    pub fn recurring_with_post_promo_amount(
        amount: Money,
        charge_at: String,
        description: String,
        interval_days: i64,
        post_promo_amount: Money,
        trial_days: Option<i64>,
    ) -> Self {
        Self::Recurring {
            amount,
            charge_at,
            description,
            interval_days,
            post_promo_amount: Some(post_promo_amount),
            trial_days,
        }
    }

    pub fn recurring_with_trial_days(
        amount: Money,
        charge_at: String,
        description: String,
        interval_days: i64,
        post_promo_amount: Option<Money>,
        trial_days: i64,
    ) -> Self {
        Self::Recurring {
            amount,
            charge_at,
            description,
            interval_days,
            post_promo_amount,
            trial_days: Some(trial_days),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
