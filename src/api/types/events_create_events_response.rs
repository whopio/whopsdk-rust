pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateEventsResponse {
    #[serde(default)]
    pub id: String,
}

impl CreateEventsResponse {
    pub fn builder() -> CreateEventsResponseBuilder {
        <CreateEventsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEventsResponseBuilder {
    id: Option<String>,
}

impl CreateEventsResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateEventsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CreateEventsResponseBuilder::id)
    pub fn build(self) -> Result<CreateEventsResponse, BuildError> {
        Ok(CreateEventsResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
