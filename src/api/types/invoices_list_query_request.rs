pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoicesListQueryRequest {
    /// Returns the elements in the list that come after the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    /// Returns the elements in the list that come before the specified cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// Returns the first _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    /// Returns the last _n_ elements from the list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    /// The unique identifier of the company to list invoices for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /// Filter invoices to only those associated with these specific product identifiers.
    #[serde(default)]
    pub product_ids: Vec<Option<String>>,
    /// Filter invoices by their collection method.
    #[serde(default)]
    pub collection_methods: Vec<Option<InvoiceCollectionMethods>>,
    /// Filter invoices by their current status.
    #[serde(default)]
    pub statuses: Vec<Option<InvoiceStatuses>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<InvoicesSortableColumns>,
    /// Only return invoices created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only return invoices created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
}

impl InvoicesListQueryRequest {
    pub fn builder() -> InvoicesListQueryRequestBuilder {
        <InvoicesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoicesListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    company_id: Option<String>,
    direction: Option<Direction>,
    product_ids: Option<Vec<Option<String>>>,
    collection_methods: Option<Vec<Option<InvoiceCollectionMethods>>>,
    statuses: Option<Vec<Option<InvoiceStatuses>>>,
    order: Option<InvoicesSortableColumns>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
}

impl InvoicesListQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn before(mut self, value: impl Into<String>) -> Self {
        self.before = Some(value.into());
        self
    }

    pub fn first(mut self, value: i64) -> Self {
        self.first = Some(value);
        self
    }

    pub fn last(mut self, value: i64) -> Self {
        self.last = Some(value);
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn direction(mut self, value: Direction) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn product_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.product_ids = Some(value);
        self
    }

    pub fn collection_methods(mut self, value: Vec<Option<InvoiceCollectionMethods>>) -> Self {
        self.collection_methods = Some(value);
        self
    }

    pub fn statuses(mut self, value: Vec<Option<InvoiceStatuses>>) -> Self {
        self.statuses = Some(value);
        self
    }

    pub fn order(mut self, value: InvoicesSortableColumns) -> Self {
        self.order = Some(value);
        self
    }

    pub fn created_before(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_before = Some(value);
        self
    }

    pub fn created_after(mut self, value: DateTime<FixedOffset>) -> Self {
        self.created_after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoicesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`product_ids`](InvoicesListQueryRequestBuilder::product_ids)
    /// - [`collection_methods`](InvoicesListQueryRequestBuilder::collection_methods)
    /// - [`statuses`](InvoicesListQueryRequestBuilder::statuses)
    pub fn build(self) -> Result<InvoicesListQueryRequest, BuildError> {
        Ok(InvoicesListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            company_id: self.company_id,
            direction: self.direction,
            product_ids: self
                .product_ids
                .ok_or_else(|| BuildError::missing_field("product_ids"))?,
            collection_methods: self
                .collection_methods
                .ok_or_else(|| BuildError::missing_field("collection_methods"))?,
            statuses: self
                .statuses
                .ok_or_else(|| BuildError::missing_field("statuses"))?,
            order: self.order,
            created_before: self.created_before,
            created_after: self.created_after,
        })
    }
}
