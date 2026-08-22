pub use crate::prelude::*;

/// Fiat pairs only: the destination currency.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSwapsResponseToToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

impl CreateSwapsResponseToToken {
    pub fn builder() -> CreateSwapsResponseToTokenBuilder {
        <CreateSwapsResponseToTokenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSwapsResponseToTokenBuilder {
    symbol: Option<String>,
}

impl CreateSwapsResponseToTokenBuilder {
    pub fn symbol(mut self, value: impl Into<String>) -> Self {
        self.symbol = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSwapsResponseToToken`].
    pub fn build(self) -> Result<CreateSwapsResponseToToken, BuildError> {
        Ok(CreateSwapsResponseToToken {
            symbol: self.symbol,
        })
    }
}
