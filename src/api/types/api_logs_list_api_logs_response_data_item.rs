pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListApiLogsResponseDataItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_key_id: Option<String>,
    #[serde(default)]
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_method: Option<ListApiLogsResponseDataItemHttpMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub http_path: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListApiLogsResponseDataItemStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

impl ListApiLogsResponseDataItem {
    pub fn builder() -> ListApiLogsResponseDataItemBuilder {
        <ListApiLogsResponseDataItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListApiLogsResponseDataItemBuilder {
    api_key_id: Option<String>,
    created_at: Option<String>,
    duration_ms: Option<i64>,
    http_method: Option<ListApiLogsResponseDataItemHttpMethod>,
    http_path: Option<String>,
    id: Option<String>,
    ip_address: Option<String>,
    operation_name: Option<String>,
    resource_id: Option<String>,
    status: Option<ListApiLogsResponseDataItemStatus>,
    status_code: Option<i64>,
    user_agent: Option<String>,
}

impl ListApiLogsResponseDataItemBuilder {
    pub fn api_key_id(mut self, value: impl Into<String>) -> Self {
        self.api_key_id = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn duration_ms(mut self, value: i64) -> Self {
        self.duration_ms = Some(value);
        self
    }

    pub fn http_method(mut self, value: ListApiLogsResponseDataItemHttpMethod) -> Self {
        self.http_method = Some(value);
        self
    }

    pub fn http_path(mut self, value: impl Into<String>) -> Self {
        self.http_path = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    pub fn operation_name(mut self, value: impl Into<String>) -> Self {
        self.operation_name = Some(value.into());
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ListApiLogsResponseDataItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_code(mut self, value: i64) -> Self {
        self.status_code = Some(value);
        self
    }

    pub fn user_agent(mut self, value: impl Into<String>) -> Self {
        self.user_agent = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListApiLogsResponseDataItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](ListApiLogsResponseDataItemBuilder::created_at)
    /// - [`id`](ListApiLogsResponseDataItemBuilder::id)
    pub fn build(self) -> Result<ListApiLogsResponseDataItem, BuildError> {
        Ok(ListApiLogsResponseDataItem {
            api_key_id: self.api_key_id,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            duration_ms: self.duration_ms,
            http_method: self.http_method,
            http_path: self.http_path,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            ip_address: self.ip_address,
            operation_name: self.operation_name,
            resource_id: self.resource_id,
            status: self.status,
            status_code: self.status_code,
            user_agent: self.user_agent,
        })
    }
}
