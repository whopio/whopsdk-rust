pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateDomainsRequest {
    /// App ID, prefixed app_. Must belong to the same account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Replacement custom string keys and values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
}

impl UpdateDomainsRequest {
    pub fn builder() -> UpdateDomainsRequestBuilder {
        <UpdateDomainsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDomainsRequestBuilder {
    app_id: Option<String>,
    metadata: Option<HashMap<String, String>>,
}

impl UpdateDomainsRequestBuilder {
    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, String>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDomainsRequest`].
    pub fn build(self) -> Result<UpdateDomainsRequest, BuildError> {
        Ok(UpdateDomainsRequest {
            app_id: self.app_id,
            metadata: self.metadata,
        })
    }
}
