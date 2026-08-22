pub use crate::prelude::*;

/// Identity document files, each value the file's raw bytes base64-encoded (JPEG, PNG, or PDF, up to 5MB per file before encoding). Sending this object verifies the person from the files in this request instead of a hosted session — individual verifications only, and the request must also carry `document_type`, `first_name`, `last_name`, `date_of_birth`, `country`, `phone`, `tax_identification_number`, and an `address` with `line1`, `city`, `state`, and `postal_code`. Send every slot for your `document_type` — a missing or rejected file fails the whole request and nothing is submitted; review starts automatically once every document is accepted. See [Identity documents](/developer/verification/identity-documents) for a full walkthrough.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateVerificationsRequestBodyIndividualDocuments {
    /// Back of the driver's license, base64-encoded. Required when `document_type` is `DRIVERS`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers_back: Option<String>,
    /// Front of the driver's license, base64-encoded. Required when `document_type` is `DRIVERS`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers_front: Option<String>,
    /// Back of the ID card, base64-encoded. Required when `document_type` is `ID_CARD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_card_back: Option<String>,
    /// Front of the ID card, base64-encoded. Required when `document_type` is `ID_CARD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_card_front: Option<String>,
    /// Photo page of the passport, base64-encoded. Required when `document_type` is `PASSPORT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_front: Option<String>,
    /// Back of the residence permit, base64-encoded. Required when `document_type` is `RESIDENCE_PERMIT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_permit_back: Option<String>,
    /// Front of the residence permit, base64-encoded. Required when `document_type` is `RESIDENCE_PERMIT`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_permit_front: Option<String>,
    /// Photo of the person's face, base64-encoded. Always required, with every document type. Must be JPEG or PNG.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<String>,
}

impl CreateVerificationsRequestBodyIndividualDocuments {
    pub fn builder() -> CreateVerificationsRequestBodyIndividualDocumentsBuilder {
        <CreateVerificationsRequestBodyIndividualDocumentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateVerificationsRequestBodyIndividualDocumentsBuilder {
    drivers_back: Option<String>,
    drivers_front: Option<String>,
    id_card_back: Option<String>,
    id_card_front: Option<String>,
    passport_front: Option<String>,
    residence_permit_back: Option<String>,
    residence_permit_front: Option<String>,
    selfie: Option<String>,
}

impl CreateVerificationsRequestBodyIndividualDocumentsBuilder {
    pub fn drivers_back(mut self, value: impl Into<String>) -> Self {
        self.drivers_back = Some(value.into());
        self
    }

    pub fn drivers_front(mut self, value: impl Into<String>) -> Self {
        self.drivers_front = Some(value.into());
        self
    }

    pub fn id_card_back(mut self, value: impl Into<String>) -> Self {
        self.id_card_back = Some(value.into());
        self
    }

    pub fn id_card_front(mut self, value: impl Into<String>) -> Self {
        self.id_card_front = Some(value.into());
        self
    }

    pub fn passport_front(mut self, value: impl Into<String>) -> Self {
        self.passport_front = Some(value.into());
        self
    }

    pub fn residence_permit_back(mut self, value: impl Into<String>) -> Self {
        self.residence_permit_back = Some(value.into());
        self
    }

    pub fn residence_permit_front(mut self, value: impl Into<String>) -> Self {
        self.residence_permit_front = Some(value.into());
        self
    }

    pub fn selfie(mut self, value: impl Into<String>) -> Self {
        self.selfie = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateVerificationsRequestBodyIndividualDocuments`].
    pub fn build(self) -> Result<CreateVerificationsRequestBodyIndividualDocuments, BuildError> {
        Ok(CreateVerificationsRequestBodyIndividualDocuments {
            drivers_back: self.drivers_back,
            drivers_front: self.drivers_front,
            id_card_back: self.id_card_back,
            id_card_front: self.id_card_front,
            passport_front: self.passport_front,
            residence_permit_back: self.residence_permit_back,
            residence_permit_front: self.residence_permit_front,
            selfie: self.selfie,
        })
    }
}
