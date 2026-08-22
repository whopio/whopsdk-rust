pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PartnersBusinessesEarningsListQueryRequest {
    /// Filter by earning status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListEarningsRequestStatus>,
    /// Filter to earnings from these income sources. Repeat the parameter for each one (income_source=sales&income_source=ad_spend).
    #[serde(default)]
    pub income_source: Vec<Option<ListEarningsRequestIncomeSourceItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<String>,
    /// The field to sort earnings by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListEarningsRequestOrder>,
    /// Sort direction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction: Option<ListEarningsRequestDirection>,
    /// Only return earnings created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_before: Option<String>,
    /// Only return earnings created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_after: Option<String>,
}

impl PartnersBusinessesEarningsListQueryRequest {
    pub fn builder() -> PartnersBusinessesEarningsListQueryRequestBuilder {
        <PartnersBusinessesEarningsListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PartnersBusinessesEarningsListQueryRequestBuilder {
    status: Option<ListEarningsRequestStatus>,
    income_source: Option<Vec<Option<ListEarningsRequestIncomeSourceItem>>>,
    first: Option<i64>,
    after: Option<String>,
    last: Option<i64>,
    before: Option<String>,
    order: Option<ListEarningsRequestOrder>,
    direction: Option<ListEarningsRequestDirection>,
    created_before: Option<String>,
    created_after: Option<String>,
}

impl PartnersBusinessesEarningsListQueryRequestBuilder {
    pub fn status(mut self, value: ListEarningsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn income_source(
        mut self,
        value: Vec<Option<ListEarningsRequestIncomeSourceItem>>,
    ) -> Self {
        self.income_source = Some(value);
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

    pub fn order(mut self, value: ListEarningsRequestOrder) -> Self {
        self.order = Some(value);
        self
    }

    pub fn direction(mut self, value: ListEarningsRequestDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn created_before(mut self, value: impl Into<String>) -> Self {
        self.created_before = Some(value.into());
        self
    }

    pub fn created_after(mut self, value: impl Into<String>) -> Self {
        self.created_after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PartnersBusinessesEarningsListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`income_source`](PartnersBusinessesEarningsListQueryRequestBuilder::income_source)
    pub fn build(self) -> Result<PartnersBusinessesEarningsListQueryRequest, BuildError> {
        Ok(PartnersBusinessesEarningsListQueryRequest {
            status: self.status,
            income_source: self
                .income_source
                .ok_or_else(|| BuildError::missing_field("income_source"))?,
            first: self.first,
            after: self.after,
            last: self.last,
            before: self.before,
            order: self.order,
            direction: self.direction,
            created_before: self.created_before,
            created_after: self.created_after,
        })
    }
}
