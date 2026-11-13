pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PinChannelsRequest {
    /// Zero-based position in the resulting list. Must not exceed the last index after insertion. Omit to append a new pin or preserve an existing position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
}

impl PinChannelsRequest {
    pub fn builder() -> PinChannelsRequestBuilder {
        <PinChannelsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PinChannelsRequestBuilder {
    position: Option<i64>,
}

impl PinChannelsRequestBuilder {
    pub fn position(mut self, value: i64) -> Self {
        self.position = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PinChannelsRequest`].
    pub fn build(self) -> Result<PinChannelsRequest, BuildError> {
        Ok(PinChannelsRequest {
            position: self.position,
        })
    }
}
