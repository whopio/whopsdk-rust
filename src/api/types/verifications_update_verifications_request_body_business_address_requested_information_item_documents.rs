pub use crate::prelude::*;

/// Answer for an `id_document` item: the same slot keys Create Verification takes, so the key names both the document and the side. Send every slot for the ID you are uploading — `PASSPORT` is `passport_front`; `ID_CARD`, `DRIVERS` and `RESIDENCE_PERMIT` take a front and a back. Each value is a direct upload ID, or a `file_`-prefixed attachment ID to reuse an uploaded document.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemDocuments {
    /// Back of the driver's license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers_back: Option<String>,
    /// Front of the driver's license.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers_front: Option<String>,
    /// Back of the ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_card_back: Option<String>,
    /// Front of the ID card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_card_front: Option<String>,
    /// Photo page of the passport.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_front: Option<String>,
    /// Back of the residence permit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_permit_back: Option<String>,
    /// Front of the residence permit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_permit_front: Option<String>,
}

impl UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemDocuments {
    pub fn builder(
    ) -> UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemDocumentsBuilder {
        <UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemDocumentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemDocumentsBuilder {
    drivers_back: Option<String>,
    drivers_front: Option<String>,
    id_card_back: Option<String>,
    id_card_front: Option<String>,
    passport_front: Option<String>,
    residence_permit_back: Option<String>,
    residence_permit_front: Option<String>,
}

impl UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemDocumentsBuilder {
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

    /// Consumes the builder and constructs a [`UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemDocuments`].
    pub fn build(
        self,
    ) -> Result<
        UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemDocuments,
        BuildError,
    > {
        Ok(
            UpdateVerificationsRequestBodyBusinessAddressRequestedInformationItemDocuments {
                drivers_back: self.drivers_back,
                drivers_front: self.drivers_front,
                id_card_back: self.id_card_back,
                id_card_front: self.id_card_front,
                passport_front: self.passport_front,
                residence_permit_back: self.residence_permit_back,
                residence_permit_front: self.residence_permit_front,
            },
        )
    }
}
