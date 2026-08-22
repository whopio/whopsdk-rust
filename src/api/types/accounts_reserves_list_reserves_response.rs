pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListReservesResponse {
    #[serde(default)]
    pub data: Vec<AccountReserve>,
}

impl ListReservesResponse {
    pub fn builder() -> ListReservesResponseBuilder {
        <ListReservesResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListReservesResponseBuilder {
    data: Option<Vec<AccountReserve>>,
}

impl ListReservesResponseBuilder {
    pub fn data(mut self, value: Vec<AccountReserve>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListReservesResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ListReservesResponseBuilder::data)
    pub fn build(self) -> Result<ListReservesResponse, BuildError> {
        Ok(ListReservesResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
