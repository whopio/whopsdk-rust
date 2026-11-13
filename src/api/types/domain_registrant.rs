pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DomainRegistrant {
    /// Registrant postal address as published, with lines separated by line breaks. Often only a state or region. `null` when withheld.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// A web form for contacting the registrant, published when the registrar withholds their email. `null` when there is none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_url: Option<String>,
    /// Registrant country as an ISO 3166-1 alpha-2 code, such as `US`, or `null` when withheld.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Registrant email address, often a privacy forwarding address, or `null` when withheld.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Registrant name, or `null` when withheld.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Registrant organization, often a privacy service, or `null` when withheld.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization: Option<String>,
    /// Registrant phone number, or `null` when withheld.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl DomainRegistrant {
    pub fn builder() -> DomainRegistrantBuilder {
        <DomainRegistrantBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DomainRegistrantBuilder {
    address: Option<String>,
    contact_url: Option<String>,
    country: Option<String>,
    email: Option<String>,
    name: Option<String>,
    organization: Option<String>,
    phone: Option<String>,
}

impl DomainRegistrantBuilder {
    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn contact_url(mut self, value: impl Into<String>) -> Self {
        self.contact_url = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn organization(mut self, value: impl Into<String>) -> Self {
        self.organization = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DomainRegistrant`].
    pub fn build(self) -> Result<DomainRegistrant, BuildError> {
        Ok(DomainRegistrant {
            address: self.address,
            contact_url: self.contact_url,
            country: self.country,
            email: self.email,
            name: self.name,
            organization: self.organization,
            phone: self.phone,
        })
    }
}
