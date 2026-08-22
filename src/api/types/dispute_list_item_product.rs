pub use crate::prelude::*;

/// The product associated with the disputed payment. Null if the dispute is not linked to a specific product.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeListItemProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
}

impl DisputeListItemProduct {
    pub fn builder() -> DisputeListItemProductBuilder {
        <DisputeListItemProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeListItemProductBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl DisputeListItemProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeListItemProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DisputeListItemProductBuilder::id)
    /// - [`title`](DisputeListItemProductBuilder::title)
    pub fn build(self) -> Result<DisputeListItemProduct, BuildError> {
        Ok(DisputeListItemProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
