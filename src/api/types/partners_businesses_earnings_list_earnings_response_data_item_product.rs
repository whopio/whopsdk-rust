pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEarningsResponseDataItemProduct {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub route: String,
    #[serde(default)]
    pub title: String,
}

impl ListEarningsResponseDataItemProduct {
    pub fn builder() -> ListEarningsResponseDataItemProductBuilder {
        <ListEarningsResponseDataItemProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEarningsResponseDataItemProductBuilder {
    id: Option<String>,
    route: Option<String>,
    title: Option<String>,
}

impl ListEarningsResponseDataItemProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEarningsResponseDataItemProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListEarningsResponseDataItemProductBuilder::id)
    /// - [`route`](ListEarningsResponseDataItemProductBuilder::route)
    /// - [`title`](ListEarningsResponseDataItemProductBuilder::title)
    pub fn build(self) -> Result<ListEarningsResponseDataItemProduct, BuildError> {
        Ok(ListEarningsResponseDataItemProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            route: self
                .route
                .ok_or_else(|| BuildError::missing_field("route"))?,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
