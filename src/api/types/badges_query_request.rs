pub use crate::prelude::*;

/// Query parameters for badges
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BadgesQueryRequest {
    /// Only return badges for these experiences (`exp_` tags).
    #[serde(default)]
    pub experience_ids: Vec<Option<String>>,
    /// The client's last fetched-at ISO 8601 timestamp, used to partially refresh badges after a websocket message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fetched_at: Option<String>,
}

impl BadgesQueryRequest {
    pub fn builder() -> BadgesQueryRequestBuilder {
        <BadgesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BadgesQueryRequestBuilder {
    experience_ids: Option<Vec<Option<String>>>,
    last_fetched_at: Option<String>,
}

impl BadgesQueryRequestBuilder {
    pub fn experience_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.experience_ids = Some(value);
        self
    }

    pub fn last_fetched_at(mut self, value: impl Into<String>) -> Self {
        self.last_fetched_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BadgesQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`experience_ids`](BadgesQueryRequestBuilder::experience_ids)
    pub fn build(self) -> Result<BadgesQueryRequest, BuildError> {
        Ok(BadgesQueryRequest {
            experience_ids: self
                .experience_ids
                .ok_or_else(|| BuildError::missing_field("experience_ids"))?,
            last_fetched_at: self.last_fetched_at,
        })
    }
}
