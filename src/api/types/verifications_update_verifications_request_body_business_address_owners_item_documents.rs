pub use crate::prelude::*;

/// The owner's identity document and selfie, each the ID of a ready file uploaded with Create File and not yet attached to anything else, prefixed `file_`. Send every slot for `document_type` plus `selfie`. Document files can be a PDF, JPEG, or PNG and the selfie a JPEG or PNG, each up to 10MB. They're sent to the verification provider in the background: the owner reads `processing` until then, and `action_required`, with a `rejection_reason` on each document, if a file couldn't be used.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateVerificationsRequestBodyBusinessAddressOwnersItemDocuments {
    /// File ID of the back of the driver's license. Required when `document_type` is `drivers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers_back: Option<String>,
    /// File ID of the front of the driver's license. Required when `document_type` is `drivers`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers_front: Option<String>,
    /// File ID of the back of the ID card. Required when `document_type` is `id_card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_card_back: Option<String>,
    /// File ID of the front of the ID card. Required when `document_type` is `id_card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_card_front: Option<String>,
    /// File ID of the passport's photo page. Required when `document_type` is `passport`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_front: Option<String>,
    /// File ID of the back of the residence permit. Required when `document_type` is `residence_permit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_permit_back: Option<String>,
    /// File ID of the front of the residence permit. Required when `document_type` is `residence_permit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_permit_front: Option<String>,
    /// File ID of a photo of the owner's face. Must be a JPEG or PNG. Required, except on Update Verification for an owner whose selfie already passed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<String>,
}

impl UpdateVerificationsRequestBodyBusinessAddressOwnersItemDocuments {
    pub fn builder() -> UpdateVerificationsRequestBodyBusinessAddressOwnersItemDocumentsBuilder {
        <UpdateVerificationsRequestBodyBusinessAddressOwnersItemDocumentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVerificationsRequestBodyBusinessAddressOwnersItemDocumentsBuilder {
    drivers_back: Option<String>,
    drivers_front: Option<String>,
    id_card_back: Option<String>,
    id_card_front: Option<String>,
    passport_front: Option<String>,
    residence_permit_back: Option<String>,
    residence_permit_front: Option<String>,
    selfie: Option<String>,
}

impl UpdateVerificationsRequestBodyBusinessAddressOwnersItemDocumentsBuilder {
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

    /// Consumes the builder and constructs a [`UpdateVerificationsRequestBodyBusinessAddressOwnersItemDocuments`].
    pub fn build(
        self,
    ) -> Result<UpdateVerificationsRequestBodyBusinessAddressOwnersItemDocuments, BuildError> {
        Ok(
            UpdateVerificationsRequestBodyBusinessAddressOwnersItemDocuments {
                drivers_back: self.drivers_back,
                drivers_front: self.drivers_front,
                id_card_back: self.id_card_back,
                id_card_front: self.id_card_front,
                passport_front: self.passport_front,
                residence_permit_back: self.residence_permit_back,
                residence_permit_front: self.residence_permit_front,
                selfie: self.selfie,
            },
        )
    }
}
