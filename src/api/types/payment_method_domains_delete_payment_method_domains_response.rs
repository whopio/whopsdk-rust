pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeletePaymentMethodDomainsResponse {
    #[serde(default)]
    pub deleted: bool,
    #[serde(default)]
    pub id: String,
}

impl DeletePaymentMethodDomainsResponse {
    pub fn builder() -> DeletePaymentMethodDomainsResponseBuilder {
        <DeletePaymentMethodDomainsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeletePaymentMethodDomainsResponseBuilder {
    deleted: Option<bool>,
    id: Option<String>,
}

impl DeletePaymentMethodDomainsResponseBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeletePaymentMethodDomainsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](DeletePaymentMethodDomainsResponseBuilder::deleted)
    /// - [`id`](DeletePaymentMethodDomainsResponseBuilder::id)
    pub fn build(self) -> Result<DeletePaymentMethodDomainsResponse, BuildError> {
        Ok(DeletePaymentMethodDomainsResponse {
            deleted: self
                .deleted
                .ok_or_else(|| BuildError::missing_field("deleted"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
