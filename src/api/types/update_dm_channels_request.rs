pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDmChannelsRequest {
    /// A new custom display name for the DM channel. For example, 'Project Discussion'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
}

impl UpdateDmChannelsRequest {
    pub fn builder() -> UpdateDmChannelsRequestBuilder {
        <UpdateDmChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDmChannelsRequestBuilder {
    custom_name: Option<String>,
}

impl UpdateDmChannelsRequestBuilder {
    pub fn custom_name(mut self, value: impl Into<String>) -> Self {
        self.custom_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateDmChannelsRequest`].
    pub fn build(self) -> Result<UpdateDmChannelsRequest, BuildError> {
        Ok(UpdateDmChannelsRequest {
            custom_name: self.custom_name,
        })
    }
}
