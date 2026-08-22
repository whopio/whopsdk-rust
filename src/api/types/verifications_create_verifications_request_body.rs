pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
#[non_exhaustive]
pub enum CreateVerificationsRequestBody {
    #[serde(rename = "individual")]
    #[non_exhaustive]
    Individual {
        #[serde(skip_serializing_if = "Option::is_none")]
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        #[serde(skip_serializing_if = "Option::is_none")]
        business_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        business_structure: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        business_tax_identification_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        business_website: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        date_of_birth: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        #[serde(skip_serializing_if = "Option::is_none")]
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        #[serde(skip_serializing_if = "Option::is_none")]
        first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        phone: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        share_token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tax_identification_number: Option<String>,
    },

    #[serde(rename = "business")]
    #[non_exhaustive]
    Business {
        #[serde(skip_serializing_if = "Option::is_none")]
        address: Option<CreateVerificationsRequestBodyBusinessAddress>,
        #[serde(skip_serializing_if = "Option::is_none")]
        business_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        business_structure: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        business_tax_identification_number: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        business_website: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        country: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        date_of_birth: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        first_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        last_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        place_of_incorporation: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        share_token: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        tax_identification_number: Option<String>,
    },

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl CreateVerificationsRequestBody {
    pub fn individual() -> Self {
        Self::Individual {
            address: None,
            business_name: None,
            business_structure: None,
            business_tax_identification_number: None,
            business_website: None,
            country: None,
            date_of_birth: None,
            document_type: None,
            documents: None,
            first_name: None,
            last_name: None,
            phone: None,
            share_token: None,
            tax_identification_number: None,
        }
    }

    pub fn business() -> Self {
        Self::Business {
            address: None,
            business_name: None,
            business_structure: None,
            business_tax_identification_number: None,
            business_website: None,
            country: None,
            date_of_birth: None,
            first_name: None,
            last_name: None,
            place_of_incorporation: None,
            share_token: None,
            tax_identification_number: None,
        }
    }

    pub fn individual_with_address(
        address: CreateVerificationsRequestBodyIndividualAddress,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address: Some(address),
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            document_type,
            documents,
            first_name,
            last_name,
            phone,
            share_token,
            tax_identification_number,
        }
    }

    pub fn individual_with_business_name(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: String,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address,
            business_name: Some(business_name),
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            document_type,
            documents,
            first_name,
            last_name,
            phone,
            share_token,
            tax_identification_number,
        }
    }

    pub fn individual_with_business_structure(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: Option<String>,
        business_structure: String,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address,
            business_name,
            business_structure: Some(business_structure),
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            document_type,
            documents,
            first_name,
            last_name,
            phone,
            share_token,
            tax_identification_number,
        }
    }

    pub fn individual_with_business_tax_identification_number(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: String,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address,
            business_name,
            business_structure,
            business_tax_identification_number: Some(business_tax_identification_number),
            business_website,
            country,
            date_of_birth,
            document_type,
            documents,
            first_name,
            last_name,
            phone,
            share_token,
            tax_identification_number,
        }
    }

    pub fn individual_with_business_website(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: String,
        country: Option<String>,
        date_of_birth: Option<String>,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website: Some(business_website),
            country,
            date_of_birth,
            document_type,
            documents,
            first_name,
            last_name,
            phone,
            share_token,
            tax_identification_number,
        }
    }

    pub fn individual_with_country(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: String,
        date_of_birth: Option<String>,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country: Some(country),
            date_of_birth,
            document_type,
            documents,
            first_name,
            last_name,
            phone,
            share_token,
            tax_identification_number,
        }
    }

    pub fn individual_with_date_of_birth(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: String,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth: Some(date_of_birth),
            document_type,
            documents,
            first_name,
            last_name,
            phone,
            share_token,
            tax_identification_number,
        }
    }

    pub fn individual_with_document_type(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        document_type: CreateVerificationsRequestBodyIndividualDocumentType,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            document_type: Some(document_type),
            documents,
            first_name,
            last_name,
            phone,
            share_token,
            tax_identification_number,
        }
    }

    pub fn individual_with_documents(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: CreateVerificationsRequestBodyIndividualDocuments,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            document_type,
            documents: Some(documents),
            first_name,
            last_name,
            phone,
            share_token,
            tax_identification_number,
        }
    }

    pub fn individual_with_first_name(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: String,
        last_name: Option<String>,
        phone: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            document_type,
            documents,
            first_name: Some(first_name),
            last_name,
            phone,
            share_token,
            tax_identification_number,
        }
    }

    pub fn individual_with_last_name(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: Option<String>,
        last_name: String,
        phone: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            document_type,
            documents,
            first_name,
            last_name: Some(last_name),
            phone,
            share_token,
            tax_identification_number,
        }
    }

    pub fn individual_with_phone(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: String,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            document_type,
            documents,
            first_name,
            last_name,
            phone: Some(phone),
            share_token,
            tax_identification_number,
        }
    }

    pub fn individual_with_share_token(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        share_token: String,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Individual {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            document_type,
            documents,
            first_name,
            last_name,
            phone,
            share_token: Some(share_token),
            tax_identification_number,
        }
    }

    pub fn individual_with_tax_identification_number(
        address: Option<CreateVerificationsRequestBodyIndividualAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        document_type: Option<CreateVerificationsRequestBodyIndividualDocumentType>,
        documents: Option<CreateVerificationsRequestBodyIndividualDocuments>,
        first_name: Option<String>,
        last_name: Option<String>,
        phone: Option<String>,
        share_token: Option<String>,
        tax_identification_number: String,
    ) -> Self {
        Self::Individual {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            document_type,
            documents,
            first_name,
            last_name,
            phone,
            share_token,
            tax_identification_number: Some(tax_identification_number),
        }
    }

    pub fn business_with_address(
        address: CreateVerificationsRequestBodyBusinessAddress,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        place_of_incorporation: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Business {
            address: Some(address),
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            first_name,
            last_name,
            place_of_incorporation,
            share_token,
            tax_identification_number,
        }
    }

    pub fn business_with_business_name(
        address: Option<CreateVerificationsRequestBodyBusinessAddress>,
        business_name: String,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        place_of_incorporation: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Business {
            address,
            business_name: Some(business_name),
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            first_name,
            last_name,
            place_of_incorporation,
            share_token,
            tax_identification_number,
        }
    }

    pub fn business_with_business_structure(
        address: Option<CreateVerificationsRequestBodyBusinessAddress>,
        business_name: Option<String>,
        business_structure: String,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        place_of_incorporation: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Business {
            address,
            business_name,
            business_structure: Some(business_structure),
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            first_name,
            last_name,
            place_of_incorporation,
            share_token,
            tax_identification_number,
        }
    }

    pub fn business_with_business_tax_identification_number(
        address: Option<CreateVerificationsRequestBodyBusinessAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: String,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        place_of_incorporation: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Business {
            address,
            business_name,
            business_structure,
            business_tax_identification_number: Some(business_tax_identification_number),
            business_website,
            country,
            date_of_birth,
            first_name,
            last_name,
            place_of_incorporation,
            share_token,
            tax_identification_number,
        }
    }

    pub fn business_with_business_website(
        address: Option<CreateVerificationsRequestBodyBusinessAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: String,
        country: Option<String>,
        date_of_birth: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        place_of_incorporation: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Business {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website: Some(business_website),
            country,
            date_of_birth,
            first_name,
            last_name,
            place_of_incorporation,
            share_token,
            tax_identification_number,
        }
    }

    pub fn business_with_country(
        address: Option<CreateVerificationsRequestBodyBusinessAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: String,
        date_of_birth: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        place_of_incorporation: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Business {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country: Some(country),
            date_of_birth,
            first_name,
            last_name,
            place_of_incorporation,
            share_token,
            tax_identification_number,
        }
    }

    pub fn business_with_date_of_birth(
        address: Option<CreateVerificationsRequestBodyBusinessAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: String,
        first_name: Option<String>,
        last_name: Option<String>,
        place_of_incorporation: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Business {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth: Some(date_of_birth),
            first_name,
            last_name,
            place_of_incorporation,
            share_token,
            tax_identification_number,
        }
    }

    pub fn business_with_first_name(
        address: Option<CreateVerificationsRequestBodyBusinessAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        first_name: String,
        last_name: Option<String>,
        place_of_incorporation: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Business {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            first_name: Some(first_name),
            last_name,
            place_of_incorporation,
            share_token,
            tax_identification_number,
        }
    }

    pub fn business_with_last_name(
        address: Option<CreateVerificationsRequestBodyBusinessAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        first_name: Option<String>,
        last_name: String,
        place_of_incorporation: Option<String>,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Business {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            first_name,
            last_name: Some(last_name),
            place_of_incorporation,
            share_token,
            tax_identification_number,
        }
    }

    pub fn business_with_place_of_incorporation(
        address: Option<CreateVerificationsRequestBodyBusinessAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        place_of_incorporation: String,
        share_token: Option<String>,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Business {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            first_name,
            last_name,
            place_of_incorporation: Some(place_of_incorporation),
            share_token,
            tax_identification_number,
        }
    }

    pub fn business_with_share_token(
        address: Option<CreateVerificationsRequestBodyBusinessAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        place_of_incorporation: Option<String>,
        share_token: String,
        tax_identification_number: Option<String>,
    ) -> Self {
        Self::Business {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            first_name,
            last_name,
            place_of_incorporation,
            share_token: Some(share_token),
            tax_identification_number,
        }
    }

    pub fn business_with_tax_identification_number(
        address: Option<CreateVerificationsRequestBodyBusinessAddress>,
        business_name: Option<String>,
        business_structure: Option<String>,
        business_tax_identification_number: Option<String>,
        business_website: Option<String>,
        country: Option<String>,
        date_of_birth: Option<String>,
        first_name: Option<String>,
        last_name: Option<String>,
        place_of_incorporation: Option<String>,
        share_token: Option<String>,
        tax_identification_number: String,
    ) -> Self {
        Self::Business {
            address,
            business_name,
            business_structure,
            business_tax_identification_number,
            business_website,
            country,
            date_of_birth,
            first_name,
            last_name,
            place_of_incorporation,
            share_token,
            tax_identification_number: Some(tax_identification_number),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
