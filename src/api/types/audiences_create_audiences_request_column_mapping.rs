pub use crate::prelude::*;

/// Custom audiences only. Maps supported identity fields to CSV column headers. Map at least one of `email` or `phone`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAudiencesRequestColumnMapping {
    /// CSV header for ISO 3166-1 alpha-2 country codes, such as `US`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// CSV header for email addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// CSV header for first names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// CSV header for last names.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// CSV header for each customer's lifetime value — a non-negative number, currency symbols allowed. When mapped, Meta creates the audience as value-based, so lookalikes built from it favor people similar to the highest-value customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ltv: Option<String>,
    /// CSV header for phone numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl CreateAudiencesRequestColumnMapping {
    pub fn builder() -> CreateAudiencesRequestColumnMappingBuilder {
        <CreateAudiencesRequestColumnMappingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAudiencesRequestColumnMappingBuilder {
    country: Option<String>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    ltv: Option<String>,
    phone: Option<String>,
}

impl CreateAudiencesRequestColumnMappingBuilder {
    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn ltv(mut self, value: impl Into<String>) -> Self {
        self.ltv = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateAudiencesRequestColumnMapping`].
    pub fn build(self) -> Result<CreateAudiencesRequestColumnMapping, BuildError> {
        Ok(CreateAudiencesRequestColumnMapping {
            country: self.country,
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            ltv: self.ltv,
            phone: self.phone,
        })
    }
}
