pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCardsResponse {
    #[serde(default)]
    pub data: Vec<ListCardsResponseDataItem>,
}

impl ListCardsResponse {
    pub fn builder() -> ListCardsResponseBuilder {
        <ListCardsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCardsResponseBuilder {
    data: Option<Vec<ListCardsResponseDataItem>>,
}

impl ListCardsResponseBuilder {
    pub fn data(mut self, value: Vec<ListCardsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListCardsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListCardsResponseBuilder::data)
    pub fn build(self) -> Result<ListCardsResponse, BuildError> {
        Ok(ListCardsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
