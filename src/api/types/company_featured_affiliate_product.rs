pub use crate::prelude::*;

/// The product featured for affiliates to promote on this company's affiliate page. Null if none is configured.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompanyFeaturedAffiliateProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// The display name of the product shown to customers. Maximum 50 characters.
    #[serde(default)]
    pub name: String,
}

impl CompanyFeaturedAffiliateProduct {
    pub fn builder() -> CompanyFeaturedAffiliateProductBuilder {
        <CompanyFeaturedAffiliateProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompanyFeaturedAffiliateProductBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl CompanyFeaturedAffiliateProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompanyFeaturedAffiliateProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompanyFeaturedAffiliateProductBuilder::id)
    /// - [`name`](CompanyFeaturedAffiliateProductBuilder::name)
    pub fn build(self) -> Result<CompanyFeaturedAffiliateProduct, BuildError> {
        Ok(CompanyFeaturedAffiliateProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
