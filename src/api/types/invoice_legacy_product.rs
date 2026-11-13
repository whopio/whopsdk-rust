pub use crate::prelude::*;

/// The product that this invoice was generated for.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceLegacyProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
}

impl InvoiceLegacyProduct {
    pub fn builder() -> InvoiceLegacyProductBuilder {
        <InvoiceLegacyProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceLegacyProductBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl InvoiceLegacyProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceLegacyProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoiceLegacyProductBuilder::id)
    /// - [`title`](InvoiceLegacyProductBuilder::title)
    pub fn build(self) -> Result<InvoiceLegacyProduct, BuildError> {
        Ok(InvoiceLegacyProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
