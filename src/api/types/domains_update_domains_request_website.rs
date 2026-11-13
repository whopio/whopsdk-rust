pub use crate::prelude::*;

/// Serve a Whop website on the domain. Pass `null` to stop serving it; the domain keeps its other capabilities.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDomainsRequestWebsite {
    /// App ID, prefixed app_, of the website to serve. The app must belong to the account. Leave it out to point the domain at Whop before the website exists; it shows a placeholder until you choose one. Pass `null` to detach the app and keep the domain pointed at Whop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
}

impl UpdateDomainsRequestWebsite {
    pub fn builder() -> UpdateDomainsRequestWebsiteBuilder {
        <UpdateDomainsRequestWebsiteBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDomainsRequestWebsiteBuilder {
    app_id: Option<String>,
}

impl UpdateDomainsRequestWebsiteBuilder {
    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateDomainsRequestWebsite`].
    pub fn build(self) -> Result<UpdateDomainsRequestWebsite, BuildError> {
        Ok(UpdateDomainsRequestWebsite {
            app_id: self.app_id,
        })
    }
}
