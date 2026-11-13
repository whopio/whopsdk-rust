pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LivestreamsListQueryRequest {
    /// ID of a livestreams experience, prefixed `exp_`.
    #[serde(default)]
    pub experience_id: String,
    /// When `true`, only pinned livestreams, most recently pinned first. When `false`, only livestreams that are not pinned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned: Option<bool>,
    /// When `true`, only ended livestreams that were recorded, including recordings still processing. `false` or omitted lists every livestream.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded: Option<bool>,
    /// Number of results to return from the start of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Return results after this cursor. Use `page_info.end_cursor` from the previous response to fetch the next page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Number of results to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// Return results before this cursor. Use `page_info.start_cursor` from the previous response to fetch the previous page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl LivestreamsListQueryRequest {
    pub fn builder() -> LivestreamsListQueryRequestBuilder {
        <LivestreamsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LivestreamsListQueryRequestBuilder {
    experience_id: Option<String>,
    pinned: Option<bool>,
    recorded: Option<bool>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl LivestreamsListQueryRequestBuilder {
    pub fn experience_id(mut self, value: impl Into<String>) -> Self {
        self.experience_id = Some(value.into());
        self
    }

    pub fn pinned(mut self, value: bool) -> Self {
        self.pinned = Some(value);
        self
    }

    pub fn recorded(mut self, value: bool) -> Self {
        self.recorded = Some(value);
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LivestreamsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`experience_id`](LivestreamsListQueryRequestBuilder::experience_id)
    pub fn build(self) -> Result<LivestreamsListQueryRequest, BuildError> {
        Ok(LivestreamsListQueryRequest {
            experience_id: self
                .experience_id
                .ok_or_else(|| BuildError::missing_field("experience_id"))?,
            pinned: self.pinned,
            recorded: self.recorded,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
