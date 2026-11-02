pub use crate::prelude::*;

/// The promotional code currently applied to this membership's billing. Null if no promo code is active.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipLegacyPromoCode {
    /// The unique identifier for the promo code.
    #[serde(default)]
    pub id: String,
}

impl MembershipLegacyPromoCode {
    pub fn builder() -> MembershipLegacyPromoCodeBuilder {
        <MembershipLegacyPromoCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipLegacyPromoCodeBuilder {
    id: Option<String>,
}

impl MembershipLegacyPromoCodeBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipLegacyPromoCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MembershipLegacyPromoCodeBuilder::id)
    pub fn build(self) -> Result<MembershipLegacyPromoCode, BuildError> {
        Ok(MembershipLegacyPromoCode {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
