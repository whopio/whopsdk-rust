pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FilesListQueryRequest {
    /// The files to return, each prefixed `file_`. Repeat the parameter to pass several, up to 250 per request. Batches of up to 100 answer in one page by default; larger batches page at up to 100 per response.
    #[serde(default)]
    pub file_ids: Vec<Option<String>>,
    /// The field to sort by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListFilesRequestOrder>,
    /// The sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListFilesRequestDirection>,
    /// The number of files to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// A cursor; returns files after this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// The number of files to return from the end of the range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// A cursor; returns files before this position.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
}

impl FilesListQueryRequest {
    pub fn builder() -> FilesListQueryRequestBuilder {
        <FilesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FilesListQueryRequestBuilder {
    file_ids: Option<Vec<Option<String>>>,
    order: Option<ListFilesRequestOrder>,
    direction: Option<ListFilesRequestDirection>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
}

impl FilesListQueryRequestBuilder {
    pub fn file_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.file_ids = Some(value);
        self
    }

    pub fn order(mut self, value: ListFilesRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListFilesRequestDirection) -> Self {
        self.direction = Some(value);
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

    /// Consumes the builder and constructs a [`FilesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`file_ids`](FilesListQueryRequestBuilder::file_ids)
    pub fn build(self) -> Result<FilesListQueryRequest, BuildError> {
        Ok(FilesListQueryRequest {
            file_ids: self
                .file_ids
                .ok_or_else(|| BuildError::missing_field("file_ids"))?,
            order: self.order,
            direction: self.direction,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
        })
    }
}
