pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PromoCodeAffiliate {
    /// Affiliate ID, prefixed `aff_`.
    #[serde(default)]
    pub id: String,
    /// User who is the affiliate.
    #[serde(default)]
    pub user: PromoCodeAffiliateUser,
}

impl PromoCodeAffiliate {
    pub fn builder() -> PromoCodeAffiliateBuilder {
        <PromoCodeAffiliateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PromoCodeAffiliateBuilder {
    id: Option<String>,
    user: Option<PromoCodeAffiliateUser>,
}

impl PromoCodeAffiliateBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn user(mut self, value: PromoCodeAffiliateUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PromoCodeAffiliate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](PromoCodeAffiliateBuilder::id)
    /// - [`user`](PromoCodeAffiliateBuilder::user)
    pub fn build(self) -> Result<PromoCodeAffiliate, BuildError> {
        Ok(PromoCodeAffiliate {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            user: self.user.ok_or_else(|| BuildError::missing_field("user"))?,
        })
    }
}
