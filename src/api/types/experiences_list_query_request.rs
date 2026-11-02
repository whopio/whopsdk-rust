pub use crate::prelude::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExperiencesListQueryRequest {
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
    /// Filter to only experiences attached to this product identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Filter to only experiences powered by this app identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// Only return experiences created before this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_before: Option<DateTime<FixedOffset>>,
    /// Only return experiences created after this timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub created_after: Option<DateTime<FixedOffset>>,
    /// The unique identifier of the company to list experiences for.
    #[serde(default)]
    pub account_id: String,
}

impl ExperiencesListQueryRequest {
    pub fn builder() -> ExperiencesListQueryRequestBuilder {
        <ExperiencesListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExperiencesListQueryRequestBuilder {
    after: Option<String>,
    before: Option<String>,
    first: Option<i64>,
    last: Option<i64>,
    product_id: Option<String>,
    app_id: Option<String>,
    created_before: Option<DateTime<FixedOffset>>,
    created_after: Option<DateTime<FixedOffset>>,
    account_id: Option<String>,
}

impl ExperiencesListQueryRequestBuilder {
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

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
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

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExperiencesListQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](ExperiencesListQueryRequestBuilder::account_id)
    pub fn build(self) -> Result<ExperiencesListQueryRequest, BuildError> {
        Ok(ExperiencesListQueryRequest {
            after: self.after,
            before: self.before,
            first: self.first,
            last: self.last,
            product_id: self.product_id,
            app_id: self.app_id,
            created_before: self.created_before,
            created_after: self.created_after,
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
        })
    }
}
