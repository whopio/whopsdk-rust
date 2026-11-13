pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainSubdomain {
    /// Whether Whop has published the subdomain's DNS records in its root domain's zone. It stays `pending` until the root domain is set up.
    pub state: DomainSubdomainState,
}

impl DomainSubdomain {
    pub fn builder() -> DomainSubdomainBuilder {
        <DomainSubdomainBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainSubdomainBuilder {
    state: Option<DomainSubdomainState>,
}

impl DomainSubdomainBuilder {
    pub fn state(mut self, value: DomainSubdomainState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainSubdomain`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](DomainSubdomainBuilder::state)
    pub fn build(self) -> Result<DomainSubdomain, BuildError> {
        Ok(DomainSubdomain {
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
