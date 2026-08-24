pub use crate::prelude::*;

/// Fields that can be updated on an individual (KYC) verification. At least one field is required.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateVerificationsRequestBodyPersonalAddress {
    /// Legal business name for a sole proprietor or single-member LLC.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// Entity type for sole proprietors, such as `single_member_llc`. Supported values vary by country of incorporation — see [Business structures](/developer/verification/business-structures).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_structure: Option<String>,
    /// The business ID number of the company, as appropriate for the company's country. Examples are an Employer Identification Number (EIN) in the US, a Business Number in Canada, or a Company Number in the UK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_tax_identification_number: Option<String>,
    /// Two-letter ISO 3166-1 country code, for example `US`, `DE`, or `GB`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Formatted as `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Personal address for the individual.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_address: Option<UpdateVerificationsRequestBodyPersonalAddressPersonalAddress>,
    /// Answers to items in `requested_information`. Each entry pairs the item `id` with one answer payload matching its `type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_information:
        Option<Vec<UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItem>>,
    /// The government-issued ID number of the person being verified — the individual for a KYC verification, or the business representative for a KYB verification — as appropriate for their country. Examples are a Social Security Number (SSN) in the US, or a Social Insurance Number in Canada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_identification_number: Option<String>,
}

impl UpdateVerificationsRequestBodyPersonalAddress {
    pub fn builder() -> UpdateVerificationsRequestBodyPersonalAddressBuilder {
        <UpdateVerificationsRequestBodyPersonalAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVerificationsRequestBodyPersonalAddressBuilder {
    business_name: Option<String>,
    business_structure: Option<String>,
    business_tax_identification_number: Option<String>,
    country: Option<String>,
    date_of_birth: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    personal_address: Option<UpdateVerificationsRequestBodyPersonalAddressPersonalAddress>,
    requested_information:
        Option<Vec<UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItem>>,
    tax_identification_number: Option<String>,
}

impl UpdateVerificationsRequestBodyPersonalAddressBuilder {
    pub fn business_name(mut self, value: impl Into<String>) -> Self {
        self.business_name = Some(value.into());
        self
    }

    pub fn business_structure(mut self, value: impl Into<String>) -> Self {
        self.business_structure = Some(value.into());
        self
    }

    pub fn business_tax_identification_number(mut self, value: impl Into<String>) -> Self {
        self.business_tax_identification_number = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn date_of_birth(mut self, value: impl Into<String>) -> Self {
        self.date_of_birth = Some(value.into());
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

    pub fn personal_address(
        mut self,
        value: UpdateVerificationsRequestBodyPersonalAddressPersonalAddress,
    ) -> Self {
        self.personal_address = Some(value);
        self
    }

    pub fn requested_information(
        mut self,
        value: Vec<UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItem>,
    ) -> Self {
        self.requested_information = Some(value);
        self
    }

    pub fn tax_identification_number(mut self, value: impl Into<String>) -> Self {
        self.tax_identification_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateVerificationsRequestBodyPersonalAddress`].
    pub fn build(self) -> Result<UpdateVerificationsRequestBodyPersonalAddress, BuildError> {
        Ok(UpdateVerificationsRequestBodyPersonalAddress {
            business_name: self.business_name,
            business_structure: self.business_structure,
            business_tax_identification_number: self.business_tax_identification_number,
            country: self.country,
            date_of_birth: self.date_of_birth,
            first_name: self.first_name,
            last_name: self.last_name,
            personal_address: self.personal_address,
            requested_information: self.requested_information,
            tax_identification_number: self.tax_identification_number,
        })
    }
}
