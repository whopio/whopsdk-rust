pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateFinancingApplicationsRequest {
    #[serde(default)]
    pub answers: Vec<UpdateFinancingApplicationsRequestAnswersItem>,
}

impl UpdateFinancingApplicationsRequest {
    pub fn builder() -> UpdateFinancingApplicationsRequestBuilder {
        <UpdateFinancingApplicationsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateFinancingApplicationsRequestBuilder {
    answers: Option<Vec<UpdateFinancingApplicationsRequestAnswersItem>>,
}

impl UpdateFinancingApplicationsRequestBuilder {
    pub fn answers(mut self, value: Vec<UpdateFinancingApplicationsRequestAnswersItem>) -> Self {
        self.answers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateFinancingApplicationsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`answers`](UpdateFinancingApplicationsRequestBuilder::answers)
    pub fn build(self) -> Result<UpdateFinancingApplicationsRequest, BuildError> {
        Ok(UpdateFinancingApplicationsRequest {
            answers: self
                .answers
                .ok_or_else(|| BuildError::missing_field("answers"))?,
        })
    }
}
