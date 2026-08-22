pub use crate::prelude::*;

/// A temporary link to a generated withdrawal PDF invoice.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeneratePdfWithdrawalsResponse {
    /// The timestamp after which the withdrawal PDF URL is no longer valid.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub expires_at: DateTime<FixedOffset>,
    /// The temporary URL for downloading the withdrawal PDF invoice.
    #[serde(default)]
    pub url: String,
}

impl GeneratePdfWithdrawalsResponse {
    pub fn builder() -> GeneratePdfWithdrawalsResponseBuilder {
        <GeneratePdfWithdrawalsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneratePdfWithdrawalsResponseBuilder {
    expires_at: Option<DateTime<FixedOffset>>,
    url: Option<String>,
}

impl GeneratePdfWithdrawalsResponseBuilder {
    pub fn expires_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GeneratePdfWithdrawalsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expires_at`](GeneratePdfWithdrawalsResponseBuilder::expires_at)
    /// - [`url`](GeneratePdfWithdrawalsResponseBuilder::url)
    pub fn build(self) -> Result<GeneratePdfWithdrawalsResponse, BuildError> {
        Ok(GeneratePdfWithdrawalsResponse {
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            url: self.url.ok_or_else(|| BuildError::missing_field("url"))?,
        })
    }
}
