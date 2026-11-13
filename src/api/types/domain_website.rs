pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DomainWebsite {
    /// The app the domain serves. `null` until you choose one; the domain shows a placeholder page meanwhile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<DomainWebsiteApp>,
    /// How far serving the website has got.
    pub state: DomainWebsiteState,
}

impl DomainWebsite {
    pub fn builder() -> DomainWebsiteBuilder {
        <DomainWebsiteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainWebsiteBuilder {
    app: Option<DomainWebsiteApp>,
    state: Option<DomainWebsiteState>,
}

impl DomainWebsiteBuilder {
    pub fn app(mut self, value: DomainWebsiteApp) -> Self {
        self.app = Some(value);
        self
    }

    pub fn state(mut self, value: DomainWebsiteState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DomainWebsite`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](DomainWebsiteBuilder::state)
    pub fn build(self) -> Result<DomainWebsite, BuildError> {
        Ok(DomainWebsite {
            app: self.app,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
