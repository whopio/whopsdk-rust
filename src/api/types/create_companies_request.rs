pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateCompaniesRequest {
    /// The country the company is located in. Defaults to the parent company's country for connected accounts, or the owner's IP-derived country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<Countries>,
    /// A promotional pitch displayed to potential customers on the company's store page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The email address of the user who will own the connected account. Required when parent_company_id is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The company's logo image. Accepts PNG, JPEG, or GIF format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<CreateCompaniesRequestLogo>,
    /// A key-value JSON object of custom metadata to store on the company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// The unique identifier of the parent platform company. When provided, creates a connected account under that platform. Omit to create a company for the current user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_company_id: Option<String>,
    /// Whether Whop sends transactional emails to customers on behalf of this company. Only applies when creating a connected account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_customer_emails: Option<bool>,
    /// The display name of the company shown to customers.
    #[serde(default)]
    pub title: String,
}

impl CreateCompaniesRequest {
    pub fn builder() -> CreateCompaniesRequestBuilder {
        <CreateCompaniesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCompaniesRequestBuilder {
    country: Option<Countries>,
    description: Option<String>,
    email: Option<String>,
    logo: Option<CreateCompaniesRequestLogo>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    parent_company_id: Option<String>,
    send_customer_emails: Option<bool>,
    title: Option<String>,
}

impl CreateCompaniesRequestBuilder {
    pub fn country(mut self, value: Countries) -> Self {
        self.country = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn logo(mut self, value: CreateCompaniesRequestLogo) -> Self {
        self.logo = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn parent_company_id(mut self, value: impl Into<String>) -> Self {
        self.parent_company_id = Some(value.into());
        self
    }

    pub fn send_customer_emails(mut self, value: bool) -> Self {
        self.send_customer_emails = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateCompaniesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](CreateCompaniesRequestBuilder::title)
    pub fn build(self) -> Result<CreateCompaniesRequest, BuildError> {
        Ok(CreateCompaniesRequest {
            country: self.country,
            description: self.description,
            email: self.email,
            logo: self.logo,
            metadata: self.metadata,
            parent_company_id: self.parent_company_id,
            send_customer_emails: self.send_customer_emails,
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
        })
    }
}
