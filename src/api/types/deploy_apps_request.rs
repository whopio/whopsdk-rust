pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeployAppsRequest {
    /// Upload the build without making it live. Defaults to `false`, which deploys and promotes in one step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft: Option<bool>,
}

impl DeployAppsRequest {
    pub fn builder() -> DeployAppsRequestBuilder {
        <DeployAppsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeployAppsRequestBuilder {
    draft: Option<bool>,
}

impl DeployAppsRequestBuilder {
    pub fn draft(mut self, value: bool) -> Self {
        self.draft = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeployAppsRequest`].
    pub fn build(self) -> Result<DeployAppsRequest, BuildError> {
        Ok(DeployAppsRequest { draft: self.draft })
    }
}
