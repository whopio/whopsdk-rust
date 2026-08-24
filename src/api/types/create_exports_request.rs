pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateExportsRequest {
    /// The account to export from, prefixed `biz_`. Defaults to the credential's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Column keys to include. Empty means all columns for the resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    /// Resource-specific filters. For native REST resources (`payouts`, `transfers`, `memberships`) these are the resource's own list query params; for dashboard tables they mirror the dashboard table filters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, serde_json::Value>>,
    /// The resource to export, e.g. `payouts`, `receipts`, or `members`.
    pub resource: CreateExportsRequestResource,
    /// IANA timezone for date columns, e.g. `America/New_York`. Defaults to `UTC`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
}

impl CreateExportsRequest {
    pub fn builder() -> CreateExportsRequestBuilder {
        <CreateExportsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateExportsRequestBuilder {
    account_id: Option<String>,
    columns: Option<Vec<String>>,
    filters: Option<HashMap<String, serde_json::Value>>,
    resource: Option<CreateExportsRequestResource>,
    timezone: Option<String>,
}

impl CreateExportsRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn columns(mut self, value: Vec<String>) -> Self {
        self.columns = Some(value);
        self
    }

    pub fn filters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn resource(mut self, value: CreateExportsRequestResource) -> Self {
        self.resource = Some(value);
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateExportsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`resource`](CreateExportsRequestBuilder::resource)
    pub fn build(self) -> Result<CreateExportsRequest, BuildError> {
        Ok(CreateExportsRequest {
            account_id: self.account_id,
            columns: self.columns,
            filters: self.filters,
            resource: self
                .resource
                .ok_or_else(|| BuildError::missing_field("resource"))?,
            timezone: self.timezone,
        })
    }
}
