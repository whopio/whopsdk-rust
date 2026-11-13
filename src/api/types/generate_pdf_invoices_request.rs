pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeneratePdfInvoicesRequest {
    /// Whether to print each line item's `description` under it. Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_descriptions: Option<bool>,
}

impl GeneratePdfInvoicesRequest {
    pub fn builder() -> GeneratePdfInvoicesRequestBuilder {
        <GeneratePdfInvoicesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneratePdfInvoicesRequestBuilder {
    include_descriptions: Option<bool>,
}

impl GeneratePdfInvoicesRequestBuilder {
    pub fn include_descriptions(mut self, value: bool) -> Self {
        self.include_descriptions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeneratePdfInvoicesRequest`].
    pub fn build(self) -> Result<GeneratePdfInvoicesRequest, BuildError> {
        Ok(GeneratePdfInvoicesRequest {
            include_descriptions: self.include_descriptions,
        })
    }
}
