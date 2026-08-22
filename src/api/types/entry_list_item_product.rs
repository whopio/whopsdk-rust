pub use crate::prelude::*;

/// The product associated with this entry's waitlisted plan. Null if the plan is not tied to a product.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntryListItemProduct {
    /// The unique identifier for the product.
    #[serde(default)]
    pub id: String,
    /// The display name of the product shown to customers on the product page and in search results.
    #[serde(default)]
    pub title: String,
}

impl EntryListItemProduct {
    pub fn builder() -> EntryListItemProductBuilder {
        <EntryListItemProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntryListItemProductBuilder {
    id: Option<String>,
    title: Option<String>,
}

impl EntryListItemProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EntryListItemProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](EntryListItemProductBuilder::id)
    /// - [`title`](EntryListItemProductBuilder::title)
    pub fn build(self) -> Result<EntryListItemProduct, BuildError> {
        Ok(EntryListItemProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
