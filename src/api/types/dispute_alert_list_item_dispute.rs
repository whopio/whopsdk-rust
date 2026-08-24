pub use crate::prelude::*;

/// The dispute associated with the dispute alert.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DisputeAlertListItemDispute {
    /// The unique identifier for the dispute.
    #[serde(default)]
    pub id: String,
}

impl DisputeAlertListItemDispute {
    pub fn builder() -> DisputeAlertListItemDisputeBuilder {
        <DisputeAlertListItemDisputeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DisputeAlertListItemDisputeBuilder {
    id: Option<String>,
}

impl DisputeAlertListItemDisputeBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DisputeAlertListItemDispute`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](DisputeAlertListItemDisputeBuilder::id)
    pub fn build(self) -> Result<DisputeAlertListItemDispute, BuildError> {
        Ok(DisputeAlertListItemDispute {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
