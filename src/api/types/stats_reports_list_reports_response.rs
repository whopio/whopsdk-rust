pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListReportsResponse {
    #[serde(default)]
    pub data: Vec<ListReportsResponseDataItem>,
}

impl ListReportsResponse {
    pub fn builder() -> ListReportsResponseBuilder {
        <ListReportsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListReportsResponseBuilder {
    data: Option<Vec<ListReportsResponseDataItem>>,
}

impl ListReportsResponseBuilder {
    pub fn data(mut self, value: Vec<ListReportsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListReportsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListReportsResponseBuilder::data)
    pub fn build(self) -> Result<ListReportsResponse, BuildError> {
        Ok(ListReportsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
