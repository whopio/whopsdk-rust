pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPromoCodesResponsePageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_cursor: Option<String>,
    #[serde(default)]
    pub has_next_page: bool,
    #[serde(default)]
    pub has_previous_page: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_cursor: Option<String>,
    /// Number of results matching the filters, across every page. Counting stops at 10,000. `null` when `include_total_count` isn't `true`, or when the count ran out of time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    /// Whether counting stopped at the limit. `true` means `total_count` is 10,000 or more.
    #[serde(default)]
    pub total_count_capped: bool,
}

impl ListPromoCodesResponsePageInfo {
    pub fn builder() -> ListPromoCodesResponsePageInfoBuilder {
        <ListPromoCodesResponsePageInfoBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPromoCodesResponsePageInfoBuilder {
    end_cursor: Option<String>,
    has_next_page: Option<bool>,
    has_previous_page: Option<bool>,
    start_cursor: Option<String>,
    total_count: Option<i64>,
    total_count_capped: Option<bool>,
}

impl ListPromoCodesResponsePageInfoBuilder {
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

    pub fn total_count(mut self, value: i64) -> Self {
        self.total_count = Some(value);
        self
    }

    pub fn total_count_capped(mut self, value: bool) -> Self {
        self.total_count_capped = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPromoCodesResponsePageInfo`].
    /// This method will fail if any of the following fields are not set:
    /// - [`has_next_page`](ListPromoCodesResponsePageInfoBuilder::has_next_page)
    /// - [`has_previous_page`](ListPromoCodesResponsePageInfoBuilder::has_previous_page)
    /// - [`total_count_capped`](ListPromoCodesResponsePageInfoBuilder::total_count_capped)
    pub fn build(self) -> Result<ListPromoCodesResponsePageInfo, BuildError> {
        Ok(ListPromoCodesResponsePageInfo {
            end_cursor: self.end_cursor,
            has_next_page: self
                .has_next_page
                .ok_or_else(|| BuildError::missing_field("has_next_page"))?,
            has_previous_page: self
                .has_previous_page
                .ok_or_else(|| BuildError::missing_field("has_previous_page"))?,
            start_cursor: self.start_cursor,
            total_count: self.total_count,
            total_count_capped: self
                .total_count_capped
                .ok_or_else(|| BuildError::missing_field("total_count_capped"))?,
        })
    }
}
