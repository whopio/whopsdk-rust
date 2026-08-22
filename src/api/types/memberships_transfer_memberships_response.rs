pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferMembershipsResponse {
    /// One-use URL the destination account opens to claim the membership.
    #[serde(default)]
    pub url: String,
}

impl TransferMembershipsResponse {
    pub fn builder() -> TransferMembershipsResponseBuilder {
        <TransferMembershipsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferMembershipsResponseBuilder {
    url: Option<String>,
}

impl TransferMembershipsResponseBuilder {
    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferMembershipsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`url`](TransferMembershipsResponseBuilder::url)
    pub fn build(self) -> Result<TransferMembershipsResponse, BuildError> {
        Ok(TransferMembershipsResponse {
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
