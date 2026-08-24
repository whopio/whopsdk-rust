pub use crate::prelude::*;

/// The product that this invoice was generated for.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
}

impl InvoiceProduct {
    pub fn builder() -> InvoiceProductBuilder {
        <InvoiceProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceProductBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl InvoiceProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](InvoiceProductBuilder::id)
    /// - [`title`](InvoiceProductBuilder::title)
    pub fn build(self) -> Result<InvoiceProduct, BuildError> {
        Ok(InvoiceProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
