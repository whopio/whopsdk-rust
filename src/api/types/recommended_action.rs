pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecommendedAction {
    /// Recommended action ID, prefixed `reca_`
    #[serde(default)]
    pub id: String,
}

impl RecommendedAction {
    pub fn builder() -> RecommendedActionBuilder {
        <RecommendedActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecommendedActionBuilder {
    id: Option<String>,
}

impl RecommendedActionBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecommendedAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RecommendedActionBuilder::id)
    pub fn build(self) -> Result<RecommendedAction, BuildError> {
        Ok(RecommendedAction {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
