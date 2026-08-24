pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteCheckoutConfigurationsResponse {
    /// Always true.
    #[serde(default)]
    pub deleted: bool,
    /// ID of the deleted checkout configuration.
    #[serde(default)]
    pub id: String,
}

impl DeleteCheckoutConfigurationsResponse {
    pub fn builder() -> DeleteCheckoutConfigurationsResponseBuilder {
        <DeleteCheckoutConfigurationsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteCheckoutConfigurationsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeleteCheckoutConfigurationsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteCheckoutConfigurationsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeleteCheckoutConfigurationsResponseBuilder::deleted)
    /// - [`id`](DeleteCheckoutConfigurationsResponseBuilder::id)
    pub fn build(self) -> Result<DeleteCheckoutConfigurationsResponse, BuildError> {
        Ok(DeleteCheckoutConfigurationsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
