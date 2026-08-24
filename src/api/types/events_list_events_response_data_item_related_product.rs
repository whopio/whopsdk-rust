pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEventsResponseDataItemRelatedProduct {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ListEventsResponseDataItemRelatedProduct {
    pub fn builder() -> ListEventsResponseDataItemRelatedProductBuilder {
        <ListEventsResponseDataItemRelatedProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemRelatedProductBuilder {
    id: Option<String>,
    route: Option<String>,
    title: Option<String>,
}

impl ListEventsResponseDataItemRelatedProductBuilder {
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

    /// Consumes the builder and constructs a [`ListEventsResponseDataItemRelatedProduct`].
    pub fn build(self) -> Result<ListEventsResponseDataItemRelatedProduct, BuildError> {
        Ok(ListEventsResponseDataItemRelatedProduct {
            id: self.id,
            route: self.route,
            title: self.title,
        })
    }
}
