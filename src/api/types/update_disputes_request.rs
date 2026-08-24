pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDisputesRequest {
    /// The evidence packet to send to the processor. Only the fields you provide are changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<UpdateDisputesRequestEvidence>,
}

impl UpdateDisputesRequest {
    pub fn builder() -> UpdateDisputesRequestBuilder {
        <UpdateDisputesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDisputesRequestBuilder {
    evidence: Option<UpdateDisputesRequestEvidence>,
}

impl UpdateDisputesRequestBuilder {
    pub fn evidence(mut self, value: UpdateDisputesRequestEvidence) -> Self {
        self.evidence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDisputesRequest`].
    pub fn build(self) -> Result<UpdateDisputesRequest, BuildError> {
        Ok(UpdateDisputesRequest {
            evidence: self.evidence,
        })
    }
}
