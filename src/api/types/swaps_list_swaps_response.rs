pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSwapsResponse {
    /// Swaps returned for this account.
    #[serde(default)]
    pub data: Vec<ListSwapsResponseDataItem>,
}

impl ListSwapsResponse {
    pub fn builder() -> ListSwapsResponseBuilder {
        <ListSwapsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSwapsResponseBuilder {
    data: Option<Vec<ListSwapsResponseDataItem>>,
}

impl ListSwapsResponseBuilder {
    pub fn data(mut self, value: Vec<ListSwapsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListSwapsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListSwapsResponseBuilder::data)
    pub fn build(self) -> Result<ListSwapsResponse, BuildError> {
        Ok(ListSwapsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
