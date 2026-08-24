pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Pagination {
    /// Current page number
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub current_page: f64,
    /// Next page number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub next_page: Option<f64>,
    /// Previous page number
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub prev_page: Option<f64>,
    /// Total number of records
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_count: f64,
    /// Total number of pages
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_pages: f64,
}

impl Pagination {
    pub fn builder() -> PaginationBuilder {
        <PaginationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaginationBuilder {
    current_page: Option<f64>,
    next_page: Option<f64>,
    prev_page: Option<f64>,
    total_count: Option<f64>,
    total_pages: Option<f64>,
}

impl PaginationBuilder {
    pub fn current_page(mut self, value: f64) -> Self {
        self.current_page = Some(value);
        self
    }

    pub fn next_page(mut self, value: f64) -> Self {
        self.next_page = Some(value);
        self
    }

    pub fn prev_page(mut self, value: f64) -> Self {
        self.prev_page = Some(value);
        self
    }

    pub fn total_count(mut self, value: f64) -> Self {
        self.total_count = Some(value);
        self
    }

    pub fn total_pages(mut self, value: f64) -> Self {
        self.total_pages = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Pagination`].
    /// This method will fail if any of the following fields are not set:
    /// - [`current_page`](PaginationBuilder::current_page)
    /// - [`total_count`](PaginationBuilder::total_count)
    /// - [`total_pages`](PaginationBuilder::total_pages)
    pub fn build(self) -> Result<Pagination, BuildError> {
        Ok(Pagination {
            current_page: self
                .current_page
                .ok_or_else(|| BuildError::missing_field("current_page"))?,
            next_page: self.next_page,
            prev_page: self.prev_page,
            total_count: self
                .total_count
                .ok_or_else(|| BuildError::missing_field("total_count"))?,
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
        })
    }
}
