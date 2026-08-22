pub use crate::prelude::*;

/// Information about pagination in a connection.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PageInfo {
    /// When paginating forwards, the cursor to continue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_cursor: Option<String>,
    /// When paginating forwards, are there more items?
    #[serde(default)]
    pub has_next_page: bool,
    /// When paginating backwards, are there more items?
    #[serde(default)]
    pub has_previous_page: bool,
    /// When paginating backwards, the cursor to continue.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<String>,
}

impl PageInfo {
    pub fn builder() -> PageInfoBuilder {
        <PageInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageInfoBuilder {
    end_cursor: Option<String>,
    has_next_page: Option<bool>,
    has_previous_page: Option<bool>,
    start_cursor: Option<String>,
}

impl PageInfoBuilder {
    pub fn end_cursor(mut self, value: impl Into<String>) -> Self {
        self.end_cursor = Some(value.into());
        self
    }

    pub fn has_next_page(mut self, value: bool) -> Self {
        self.has_next_page = Some(value);
        self
    }

    pub fn has_previous_page(mut self, value: bool) -> Self {
        self.has_previous_page = Some(value);
        self
    }

    pub fn start_cursor(mut self, value: impl Into<String>) -> Self {
        self.start_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PageInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_next_page`](PageInfoBuilder::has_next_page)
    /// - [`has_previous_page`](PageInfoBuilder::has_previous_page)
    pub fn build(self) -> Result<PageInfo, BuildError> {
        Ok(PageInfo {
            end_cursor: self.end_cursor,
            has_next_page: self
                .has_next_page
                .ok_or_else(|| BuildError::missing_field("has_next_page"))?,
            has_previous_page: self
                .has_previous_page
                .ok_or_else(|| BuildError::missing_field("has_previous_page"))?,
            start_cursor: self.start_cursor,
        })
    }
}
