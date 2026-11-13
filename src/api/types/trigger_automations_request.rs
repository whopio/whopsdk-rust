pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TriggerAutomationsRequest {
    /// Who to enroll: up to 1,000 member IDs, or an object of `GET /members` filters. An empty object enrolls every member.
    pub members: TriggerAutomationsRequestMembers,
}

impl TriggerAutomationsRequest {
    pub fn builder() -> TriggerAutomationsRequestBuilder {
        <TriggerAutomationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TriggerAutomationsRequestBuilder {
    members: Option<TriggerAutomationsRequestMembers>,
}

impl TriggerAutomationsRequestBuilder {
    pub fn members(mut self, value: TriggerAutomationsRequestMembers) -> Self {
        self.members = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TriggerAutomationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`members`](TriggerAutomationsRequestBuilder::members)
    pub fn build(self) -> Result<TriggerAutomationsRequest, BuildError> {
        Ok(TriggerAutomationsRequest {
            members: self
                .members
                .ok_or_else(|| BuildError::missing_field("members"))?,
        })
    }
}
