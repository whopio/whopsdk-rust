pub use crate::prelude::*;

/// An owner of the business. Send `verification_id` to add someone with an approved verification on this account or on a member of its team, or a new owner's details and documents to verify them from this request. An owner already on the verification is not added twice, so a retried request is safe.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateVerificationsRequestBodyBusinessOwnersItem {
    /// The new owner's home address, with `line1`, `city`, `state`, and `postal_code`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<CreateVerificationsRequestBodyBusinessOwnersItemAddress>,
    /// The new owner's country, as a two-letter ISO 3166-1 code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The new owner's date of birth, formatted as `YYYY-MM-DD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<NaiveDate>,
    /// Identity document in `documents`. Decides which slots to send: `id_card` → `id_card_front` + `id_card_back`; `drivers` → `drivers_front` + `drivers_back`; `residence_permit` → `residence_permit_front` + `residence_permit_back`; `passport` → `passport_front`. Every type also takes `selfie`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document_type: Option<CreateVerificationsRequestBodyBusinessOwnersItemDocumentType>,
    /// The owner's identity document and selfie, each the ID of a ready file uploaded with Create File and not yet attached to anything else, prefixed `file_`. Send every slot for `document_type` plus `selfie`. Document files can be a PDF, JPEG, or PNG and the selfie a JPEG or PNG, each up to 10MB. They're sent to the verification provider in the background: the owner reads `processing` until then, and `action_required`, with a `rejection_reason` on each document, if a file couldn't be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents: Option<CreateVerificationsRequestBodyBusinessOwnersItemDocuments>,
    /// The new owner's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The new owner's first name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The new owner's last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Share of the business the person owns, from 0 to 100. Across every owner it can't add up to more than 100.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ownership_percentage: Option<f64>,
    /// The new owner's phone number, with country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// What the person is to the business: `ubo` for an ultimate beneficial owner, `director`, `representative` for someone who acts for the business, or `shareholder`. Required for a new owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<CreateVerificationsRequestBodyBusinessOwnersItemRolesItem>>,
    /// The new owner's government-issued ID number, such as a Social Security Number in the US.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_identification_number: Option<String>,
    /// ID of an `approved` individual verification to add as this owner, prefixed `idpf_`. It must be the account's own verification or a team member's. Takes only `roles` and `ownership_percentage` alongside it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_id: Option<String>,
}

impl CreateVerificationsRequestBodyBusinessOwnersItem {
    pub fn builder() -> CreateVerificationsRequestBodyBusinessOwnersItemBuilder {
        <CreateVerificationsRequestBodyBusinessOwnersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVerificationsRequestBodyBusinessOwnersItemBuilder {
    address: Option<CreateVerificationsRequestBodyBusinessOwnersItemAddress>,
    country: Option<String>,
    date_of_birth: Option<NaiveDate>,
    document_type: Option<CreateVerificationsRequestBodyBusinessOwnersItemDocumentType>,
    documents: Option<CreateVerificationsRequestBodyBusinessOwnersItemDocuments>,
    email: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    ownership_percentage: Option<f64>,
    phone: Option<String>,
    roles: Option<Vec<CreateVerificationsRequestBodyBusinessOwnersItemRolesItem>>,
    tax_identification_number: Option<String>,
    verification_id: Option<String>,
}

impl CreateVerificationsRequestBodyBusinessOwnersItemBuilder {
    pub fn address(
        mut self,
        value: CreateVerificationsRequestBodyBusinessOwnersItemAddress,
    ) -> Self {
        self.address = Some(value);
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn date_of_birth(mut self, value: NaiveDate) -> Self {
        self.date_of_birth = Some(value);
        self
    }

    pub fn document_type(
        mut self,
        value: CreateVerificationsRequestBodyBusinessOwnersItemDocumentType,
    ) -> Self {
        self.document_type = Some(value);
        self
    }

    pub fn documents(
        mut self,
        value: CreateVerificationsRequestBodyBusinessOwnersItemDocuments,
    ) -> Self {
        self.documents = Some(value);
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

    pub fn ownership_percentage(mut self, value: f64) -> Self {
        self.ownership_percentage = Some(value);
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn roles(
        mut self,
        value: Vec<CreateVerificationsRequestBodyBusinessOwnersItemRolesItem>,
    ) -> Self {
        self.roles = Some(value);
        self
    }

    pub fn tax_identification_number(mut self, value: impl Into<String>) -> Self {
        self.tax_identification_number = Some(value.into());
        self
    }

    pub fn verification_id(mut self, value: impl Into<String>) -> Self {
        self.verification_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateVerificationsRequestBodyBusinessOwnersItem`].
    pub fn build(self) -> Result<CreateVerificationsRequestBodyBusinessOwnersItem, BuildError> {
        Ok(CreateVerificationsRequestBodyBusinessOwnersItem {
            address: self.address,
            country: self.country,
            date_of_birth: self.date_of_birth,
            document_type: self.document_type,
            documents: self.documents,
            email: self.email,
            first_name: self.first_name,
            last_name: self.last_name,
            ownership_percentage: self.ownership_percentage,
            phone: self.phone,
            roles: self.roles,
            tax_identification_number: self.tax_identification_number,
            verification_id: self.verification_id,
        })
    }
}
