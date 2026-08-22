pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateLeadsRequest {
    /// The unique identifier of the company to create the lead for, starting with 'biz_'.
    #[serde(default)]
    pub company_id: String,
    /// A JSON object of custom metadata to attach to the lead for tracking purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The unique identifier of the product the lead is interested in, starting with 'prod_'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// The referral URL that brought the lead to the company, such as 'https://example.com/landing'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
    /// The unique identifier of the user to record as the lead. If authenticated as a user, that user is used automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl CreateLeadsRequest {
    pub fn builder() -> CreateLeadsRequestBuilder {
        <CreateLeadsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateLeadsRequestBuilder {
    company_id: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    product_id: Option<String>,
    referrer: Option<String>,
    user_id: Option<String>,
}

impl CreateLeadsRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn referrer(mut self, value: impl Into<String>) -> Self {
        self.referrer = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateLeadsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](CreateLeadsRequestBuilder::company_id)
    pub fn build(self) -> Result<CreateLeadsRequest, BuildError> {
        Ok(CreateLeadsRequest {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            metadata: self.metadata,
            product_id: self.product_id,
            referrer: self.referrer,
            user_id: self.user_id,
        })
    }
}
