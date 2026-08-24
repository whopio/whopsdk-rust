pub use crate::prelude::*;

/// Fiat pairs only: the source currency.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateSwapsResponseFromToken {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

impl CreateSwapsResponseFromToken {
    pub fn builder() -> CreateSwapsResponseFromTokenBuilder {
        <CreateSwapsResponseFromTokenBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateSwapsResponseFromTokenBuilder {
    symbol: Option<String>,
}

impl CreateSwapsResponseFromTokenBuilder {
    pub fn symbol(mut self, value: impl Into<String>) -> Self {
        self.symbol = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateSwapsResponseFromToken`].
    pub fn build(self) -> Result<CreateSwapsResponseFromToken, BuildError> {
        Ok(CreateSwapsResponseFromToken {
            symbol: self.symbol,
        })
    }
}
