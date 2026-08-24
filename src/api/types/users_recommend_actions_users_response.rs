pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecommendActionsUsersResponse {
    #[serde(default)]
    pub data: Vec<UserRecommendedAction>,
}

impl RecommendActionsUsersResponse {
    pub fn builder() -> RecommendActionsUsersResponseBuilder {
        <RecommendActionsUsersResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecommendActionsUsersResponseBuilder {
    data: Option<Vec<UserRecommendedAction>>,
}

impl RecommendActionsUsersResponseBuilder {
    pub fn data(mut self, value: Vec<UserRecommendedAction>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RecommendActionsUsersResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](RecommendActionsUsersResponseBuilder::data)
    pub fn build(self) -> Result<RecommendActionsUsersResponse, BuildError> {
        Ok(RecommendActionsUsersResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
