pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListVerificationsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<ListVerificationsResponseDataItem>>,
}

impl ListVerificationsResponse {
    pub fn builder() -> ListVerificationsResponseBuilder {
        <ListVerificationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVerificationsResponseBuilder {
    data: Option<Vec<ListVerificationsResponseDataItem>>,
}

impl ListVerificationsResponseBuilder {
    pub fn data(mut self, value: Vec<ListVerificationsResponseDataItem>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListVerificationsResponse`].
    pub fn build(self) -> Result<ListVerificationsResponse, BuildError> {
        Ok(ListVerificationsResponse { data: self.data })
    }
}
