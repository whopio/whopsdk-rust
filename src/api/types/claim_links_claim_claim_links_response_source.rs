pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ClaimClaimLinksResponseSource {
    #[serde(default)]
    pub account_id: String,
}

impl ClaimClaimLinksResponseSource {
    pub fn builder() -> ClaimClaimLinksResponseSourceBuilder {
        <ClaimClaimLinksResponseSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ClaimClaimLinksResponseSourceBuilder {
    account_id: Option<String>,
}

impl ClaimClaimLinksResponseSourceBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ClaimClaimLinksResponseSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ClaimClaimLinksResponseSourceBuilder::account_id)
    pub fn build(self) -> Result<ClaimClaimLinksResponseSource, BuildError> {
        Ok(ClaimClaimLinksResponseSource {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
        })
    }
}
