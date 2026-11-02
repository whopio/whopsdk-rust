pub use crate::prelude::*;

/// The product associated with the disputed payment. Null if the dispute is not linked to a specific product.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeLegacyProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
}

impl DisputeLegacyProduct {
    pub fn builder() -> DisputeLegacyProductBuilder {
        <DisputeLegacyProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeLegacyProductBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl DisputeLegacyProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeLegacyProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DisputeLegacyProductBuilder::id)
    /// - [`title`](DisputeLegacyProductBuilder::title)
    pub fn build(self) -> Result<DisputeLegacyProduct, BuildError> {
        Ok(DisputeLegacyProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
