pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListExternalAccountsResponsePageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_cursor: Option<String>,
    #[serde(default)]
    pub has_next_page: bool,
    #[serde(default)]
    pub has_previous_page: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<String>,
}

impl ListExternalAccountsResponsePageInfo {
    pub fn builder() -> ListExternalAccountsResponsePageInfoBuilder {
        <ListExternalAccountsResponsePageInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListExternalAccountsResponsePageInfoBuilder {
    end_cursor: Option<String>,
    has_next_page: Option<bool>,
    has_previous_page: Option<bool>,
    start_cursor: Option<String>,
}

impl ListExternalAccountsResponsePageInfoBuilder {
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

    /// Consumes the builder and constructs a [`ListExternalAccountsResponsePageInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_next_page`](ListExternalAccountsResponsePageInfoBuilder::has_next_page)
    /// - [`has_previous_page`](ListExternalAccountsResponsePageInfoBuilder::has_previous_page)
    pub fn build(self) -> Result<ListExternalAccountsResponsePageInfo, BuildError> {
        Ok(ListExternalAccountsResponsePageInfo {
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
