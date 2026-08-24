pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateAccountLinksRequest {
    /// The unique identifier of the company to generate the link for, starting with 'biz_'. Must be a sub-merchant of the API key's company.
    #[serde(default)]
    pub company_id: String,
    /// The URL to redirect the user to if the session expires and needs to be re-authenticated, such as 'https://example.com/refresh'.
    #[serde(default)]
    pub refresh_url: String,
    /// The URL to redirect the user to when they want to return to your site, such as 'https://example.com/return'.
    #[serde(default)]
    pub return_url: String,
    /// The purpose of the account link, such as hosted payouts portal or hosted KYC onboarding.
    pub use_case: AccountLinkUseCases,
}

impl CreateAccountLinksRequest {
    pub fn builder() -> CreateAccountLinksRequestBuilder {
        <CreateAccountLinksRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAccountLinksRequestBuilder {
    company_id: Option<String>,
    refresh_url: Option<String>,
    return_url: Option<String>,
    use_case: Option<AccountLinkUseCases>,
}

impl CreateAccountLinksRequestBuilder {
    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn refresh_url(mut self, value: impl Into<String>) -> Self {
        self.refresh_url = Some(value.into());
        self
    }

    pub fn return_url(mut self, value: impl Into<String>) -> Self {
        self.return_url = Some(value.into());
        self
    }

    pub fn use_case(mut self, value: AccountLinkUseCases) -> Self {
        self.use_case = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAccountLinksRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`company_id`](CreateAccountLinksRequestBuilder::company_id)
    /// - [`refresh_url`](CreateAccountLinksRequestBuilder::refresh_url)
    /// - [`return_url`](CreateAccountLinksRequestBuilder::return_url)
    /// - [`use_case`](CreateAccountLinksRequestBuilder::use_case)
    pub fn build(self) -> Result<CreateAccountLinksRequest, BuildError> {
        Ok(CreateAccountLinksRequest {
            company_id: self
                .company_id
                .ok_or_else(|| BuildError::missing_field("company_id"))?,
            refresh_url: self
                .refresh_url
                .ok_or_else(|| BuildError::missing_field("refresh_url"))?,
            return_url: self
                .return_url
                .ok_or_else(|| BuildError::missing_field("return_url"))?,
            use_case: self
                .use_case
                .ok_or_else(|| BuildError::missing_field("use_case"))?,
        })
    }
}
