pub use crate::prelude::*;

/// The connection type for Reaction.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListReactionsResponse {
    /// A list of nodes.
    #[serde(default)]
    pub data: Vec<ReactionListItem>,
    /// Information to aid in pagination.
    #[serde(default)]
    pub page_info: PageInfo,
}

impl ListReactionsResponse {
    pub fn builder() -> ListReactionsResponseBuilder {
        <ListReactionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListReactionsResponseBuilder {
    data: Option<Vec<ReactionListItem>>,
    page_info: Option<PageInfo>,
}

impl ListReactionsResponseBuilder {
    pub fn data(mut self, value: Vec<ReactionListItem>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn page_info(mut self, value: PageInfo) -> Self {
        self.page_info = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListReactionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListReactionsResponseBuilder::data)
    /// - [`page_info`](ListReactionsResponseBuilder::page_info)
    pub fn build(self) -> Result<ListReactionsResponse, BuildError> {
        Ok(ListReactionsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            page_info: self
                .page_info
                .ok_or_else(|| BuildError::missing_field("page_info"))?,
        })
    }
}
