pub use crate::prelude::*;

/// The account, variant and promo code for a purchase.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaymentInput {
    /// The account the purchase belongs to, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// The promo code as the buyer typed it, matched within the account regardless of case and surrounding spaces, as checkout matches it. It must be valid for the variant. Send it or `promo_code_id`, not both; an empty or whitespace-only string counts as not sent. A code the account does not have, or one that is no longer active, is refused before anything is written, with the error code `promo_invalid`. A code this purchase cannot use, such as one with no uses left or one restricted to other variants, products or buyers, is refused with `promo_invalid` too, and the error's message says why.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code: Option<String>,
    /// An active promo code to apply, prefixed `promo_`. It must belong to the account and be valid for the variant. Send it or `promo_code`, not both. A code this purchase cannot use is refused with the error code `promo_invalid`, and the error's message says why.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_code_id: Option<String>,
    /// The variant purchased, described by its attributes instead of an id: the variant with exactly these attributes is used, and one is created when none exists. Mutually exclusive with `variant_id` and `line_items`. Creating a variant requires `plan:create`; creating or updating a product requires the corresponding product permission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant: Option<PaymentInputVariant>,
    /// The variant purchased, prefixed `plan_`. It must belong to the account. Mutually exclusive with `variant` and `line_items`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variant_id: Option<String>,
}

impl PaymentInput {
    pub fn builder() -> PaymentInputBuilder {
        <PaymentInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentInputBuilder {
    account_id: Option<String>,
    promo_code: Option<String>,
    promo_code_id: Option<String>,
    variant: Option<PaymentInputVariant>,
    variant_id: Option<String>,
}

impl PaymentInputBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn promo_code(mut self, value: impl Into<String>) -> Self {
        self.promo_code = Some(value.into());
        self
    }

    pub fn promo_code_id(mut self, value: impl Into<String>) -> Self {
        self.promo_code_id = Some(value.into());
        self
    }

    pub fn variant(mut self, value: PaymentInputVariant) -> Self {
        self.variant = Some(value);
        self
    }

    pub fn variant_id(mut self, value: impl Into<String>) -> Self {
        self.variant_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PaymentInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](PaymentInputBuilder::account_id)
    pub fn build(self) -> Result<PaymentInput, BuildError> {
        Ok(PaymentInput {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            promo_code: self.promo_code,
            promo_code_id: self.promo_code_id,
            variant: self.variant,
            variant_id: self.variant_id,
        })
    }
}
