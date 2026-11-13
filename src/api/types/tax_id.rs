pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TaxId {
    /// The registration's type, such as `eu_vat`.
    pub r#type: TaxIdType,
    /// The registration number as the buyer entered it, whitespace removed (`eu_vat` values also drop dots and dashes).
    #[serde(default)]
    pub value: String,
}

impl TaxId {
    pub fn builder() -> TaxIdBuilder {
        <TaxIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TaxIdBuilder {
    r#type: Option<TaxIdType>,
    value: Option<String>,
}

impl TaxIdBuilder {
    pub fn r#type(mut self, value: TaxIdType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TaxId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](TaxIdBuilder::r#type)
    /// - [`value`](TaxIdBuilder::value)
    pub fn build(self) -> Result<TaxId, BuildError> {
        Ok(TaxId {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
