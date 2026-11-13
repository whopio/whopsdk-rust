pub use crate::prelude::*;

/// Fields that can be updated on a business entity (KYB) verification. At least one field is required.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateVerificationsRequestBodyBusinessAddress {
    /// Business address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_address: Option<UpdateVerificationsRequestBodyBusinessAddressBusinessAddress>,
    /// Legal business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    /// Legal entity structure of the business, such as `private_corporation` or `sole_proprietorship`. Supported values vary by country of incorporation — see [Business structures](/developer/verification/business-structures).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_structure: Option<String>,
    /// The business ID number of the company, as appropriate for the company's country. Examples are an Employer Identification Number (EIN) in the US, a Business Number in Canada, or a Company Number in the UK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_tax_identification_number: Option<String>,
    /// Two-letter ISO 3166-1 country code, for example `US`, `DE`, or `GB`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// Business representative's date of birth, formatted as `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// Company formation documents, each the ID of a file uploaded with Create File and not yet attached to anything else, prefixed `file_`. Send the ID once its `upload_status` is `ready`. Each file must be a PDF, JPEG, or PNG of up to 10MB. They are filed under the business's country, from `country`, `address`, or an earlier request. A document of a kind already on file is kept unless it was rejected, so a retried request does not add a second copy. The hosted session at `session_url` collects anything still missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<UpdateVerificationsRequestBodyBusinessAddressDocuments>,
    /// First name of the business representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Date the business was incorporated, formatted as `YYYY-MM-DD`. Cannot be in the future. Sent to the verification provider and returned in the response; `requested_information` asks for it while it is still missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incorporation_date: Option<NaiveDate>,
    /// Last name of the business representative.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Owners of the business, up to 10 per request. Once the company details, the company documents, and the owners are in and every new owner's documents were sent, Create Verification sends the business for review: in the request that completes it, as soon as the last owner's documents finish sending in the background, or when called again after Update Verification. Owners can still be added, changed, or removed while the review waits on them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owners: Option<Vec<UpdateVerificationsRequestBodyBusinessAddressOwnersItem>>,
    /// State or region where the business is incorporated. Sent to the verification provider and returned in the response; `requested_information` asks for it while it is still missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_of_incorporation: Option<String>,
    /// Number the business was registered under in its place of incorporation, such as a state file number in the US or a Company Number in the UK. Sent to the verification provider and returned in the response; `requested_information` asks for it while it is still missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_number: Option<String>,
    /// Answers to items in `requested_information`. Each entry pairs the item `id` with one answer payload matching its `type`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_information:
        Option<Vec<UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItem>>,
    /// The government-issued ID number of the person being verified — the individual for a KYC verification, or the business representative for a KYB verification — as appropriate for their country. Examples are a Social Security Number (SSN) in the US, or a Social Insurance Number in Canada.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_identification_number: Option<String>,
}

impl UpdateVerificationsRequestBodyBusinessAddress {
    pub fn builder() -> UpdateVerificationsRequestBodyBusinessAddressBuilder {
        <UpdateVerificationsRequestBodyBusinessAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVerificationsRequestBodyBusinessAddressBuilder {
    business_address: Option<UpdateVerificationsRequestBodyBusinessAddressBusinessAddress>,
    business_name: Option<String>,
    business_structure: Option<String>,
    business_tax_identification_number: Option<String>,
    country: Option<String>,
    date_of_birth: Option<String>,
    documents: Option<UpdateVerificationsRequestBodyBusinessAddressDocuments>,
    first_name: Option<String>,
    incorporation_date: Option<NaiveDate>,
    last_name: Option<String>,
    owners: Option<Vec<UpdateVerificationsRequestBodyBusinessAddressOwnersItem>>,
    place_of_incorporation: Option<String>,
    registration_number: Option<String>,
    requested_information:
        Option<Vec<UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItem>>,
    tax_identification_number: Option<String>,
}

impl UpdateVerificationsRequestBodyBusinessAddressBuilder {
    pub fn business_address(
        mut self,
        value: UpdateVerificationsRequestBodyBusinessAddressBusinessAddress,
    ) -> Self {
        self.business_address = Some(value);
        self
    }

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

    pub fn documents(
        mut self,
        value: UpdateVerificationsRequestBodyBusinessAddressDocuments,
    ) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn incorporation_date(mut self, value: NaiveDate) -> Self {
        self.incorporation_date = Some(value);
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn owners(
        mut self,
        value: Vec<UpdateVerificationsRequestBodyBusinessAddressOwnersItem>,
    ) -> Self {
        self.owners = Some(value);
        self
    }

    pub fn place_of_incorporation(mut self, value: impl Into<String>) -> Self {
        self.place_of_incorporation = Some(value.into());
        self
    }

    pub fn registration_number(mut self, value: impl Into<String>) -> Self {
        self.registration_number = Some(value.into());
        self
    }

    pub fn requested_information(
        mut self,
        value: Vec<UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItem>,
    ) -> Self {
        self.requested_information = Some(value);
        self
    }

    pub fn tax_identification_number(mut self, value: impl Into<String>) -> Self {
        self.tax_identification_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateVerificationsRequestBodyBusinessAddress`].
    pub fn build(self) -> Result<UpdateVerificationsRequestBodyBusinessAddress, BuildError> {
        Ok(UpdateVerificationsRequestBodyBusinessAddress {
            business_address: self.business_address,
            business_name: self.business_name,
            business_structure: self.business_structure,
            business_tax_identification_number: self.business_tax_identification_number,
            country: self.country,
            date_of_birth: self.date_of_birth,
            documents: self.documents,
            first_name: self.first_name,
            incorporation_date: self.incorporation_date,
            last_name: self.last_name,
            owners: self.owners,
            place_of_incorporation: self.place_of_incorporation,
            registration_number: self.registration_number,
            requested_information: self.requested_information,
            tax_identification_number: self.tax_identification_number,
        })
    }
}
