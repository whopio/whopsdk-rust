pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateShipmentsRequest {
    /// The new carrier-assigned tracking number.
    #[serde(default)]
    pub tracking_number: String,
}

impl UpdateShipmentsRequest {
    pub fn builder() -> UpdateShipmentsRequestBuilder {
        <UpdateShipmentsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateShipmentsRequestBuilder {
    tracking_number: Option<String>,
}

impl UpdateShipmentsRequestBuilder {
    pub fn tracking_number(mut self, value: impl Into<String>) -> Self {
        self.tracking_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateShipmentsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tracking_number`](UpdateShipmentsRequestBuilder::tracking_number)
    pub fn build(self) -> Result<UpdateShipmentsRequest, BuildError> {
        Ok(UpdateShipmentsRequest {
            tracking_number: self
                .tracking_number
                .ok_or_else(|| BuildError::missing_field("tracking_number"))?,
        })
    }
}
