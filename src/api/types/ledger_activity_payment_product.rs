pub use crate::prelude::*;

/// Product associated with the payment, when applicable.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LedgerActivityPaymentProduct {
    /// Product ID, prefixed `prod_`.
    #[serde(default)]
    pub id: String,
    /// Product name.
    #[serde(default)]
    pub name: String,
}

impl LedgerActivityPaymentProduct {
    pub fn builder() -> LedgerActivityPaymentProductBuilder {
        <LedgerActivityPaymentProductBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LedgerActivityPaymentProductBuilder {
    id: Option<String>,
    name: Option<String>,
}

impl LedgerActivityPaymentProductBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LedgerActivityPaymentProduct`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](LedgerActivityPaymentProductBuilder::id)
    /// - [`name`](LedgerActivityPaymentProductBuilder::name)
    pub fn build(self) -> Result<LedgerActivityPaymentProduct, BuildError> {
        Ok(LedgerActivityPaymentProduct {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
        })
    }
}
