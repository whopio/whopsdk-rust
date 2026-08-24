pub use crate::prelude::*;

/// The promotional code currently applied to this membership's billing. Null if no promo code is active.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MembershipListItemPromoCode {
    /// The unique identifier for the promo code.
    #[serde(default)]
    pub id: String,
}

impl MembershipListItemPromoCode {
    pub fn builder() -> MembershipListItemPromoCodeBuilder {
        <MembershipListItemPromoCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MembershipListItemPromoCodeBuilder {
    id: Option<String>,
}

impl MembershipListItemPromoCodeBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MembershipListItemPromoCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](MembershipListItemPromoCodeBuilder::id)
    pub fn build(self) -> Result<MembershipListItemPromoCode, BuildError> {
        Ok(MembershipListItemPromoCode {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
