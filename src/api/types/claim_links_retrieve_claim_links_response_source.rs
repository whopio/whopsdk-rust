pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveClaimLinksResponseSource {
    #[serde(default)]
    pub account_id: String,
}

impl RetrieveClaimLinksResponseSource {
    pub fn builder() -> RetrieveClaimLinksResponseSourceBuilder {
        <RetrieveClaimLinksResponseSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveClaimLinksResponseSourceBuilder {
    account_id: Option<String>,
}

impl RetrieveClaimLinksResponseSourceBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrieveClaimLinksResponseSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](RetrieveClaimLinksResponseSourceBuilder::account_id)
    pub fn build(self) -> Result<RetrieveClaimLinksResponseSource, BuildError> {
        Ok(RetrieveClaimLinksResponseSource {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
        })
    }
}
