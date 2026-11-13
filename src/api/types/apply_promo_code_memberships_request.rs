pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplyPromoCodeMembershipsRequest {
    /// The promo code to apply, as customers enter it at checkout (for example `SAVE20`). Its stock, variant eligibility, and expiry are checked as at checkout.
    #[serde(default)]
    pub promo_code: String,
}

impl ApplyPromoCodeMembershipsRequest {
    pub fn builder() -> ApplyPromoCodeMembershipsRequestBuilder {
        <ApplyPromoCodeMembershipsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplyPromoCodeMembershipsRequestBuilder {
    promo_code: Option<String>,
}

impl ApplyPromoCodeMembershipsRequestBuilder {
    pub fn promo_code(mut self, value: impl Into<String>) -> Self {
        self.promo_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApplyPromoCodeMembershipsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`promo_code`](ApplyPromoCodeMembershipsRequestBuilder::promo_code)
    pub fn build(self) -> Result<ApplyPromoCodeMembershipsRequest, BuildError> {
        Ok(ApplyPromoCodeMembershipsRequest {
            promo_code: self
                .promo_code
                .ok_or_else(|| BuildError::missing_field("promo_code"))?,
        })
    }
}
