pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct LeadFormsSocialAccountsResponse {
    #[serde(default)]
    pub data: Vec<SocialAccountLeadForm>,
}

impl LeadFormsSocialAccountsResponse {
    pub fn builder() -> LeadFormsSocialAccountsResponseBuilder {
        <LeadFormsSocialAccountsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LeadFormsSocialAccountsResponseBuilder {
    data: Option<Vec<SocialAccountLeadForm>>,
}

impl LeadFormsSocialAccountsResponseBuilder {
    pub fn data(mut self, value: Vec<SocialAccountLeadForm>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LeadFormsSocialAccountsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](LeadFormsSocialAccountsResponseBuilder::data)
    pub fn build(self) -> Result<LeadFormsSocialAccountsResponse, BuildError> {
        Ok(LeadFormsSocialAccountsResponse {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
