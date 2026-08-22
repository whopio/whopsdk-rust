pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItem {
    /// Answer for `address` items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address:
        Option<UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemAddress>,
    /// Answer for an `id_document` item: the same slot keys Create Verification takes, so the key names both the document and the side. Send every slot for the ID you are uploading — `PASSPORT` is `passport_front`; `ID_CARD`, `DRIVERS` and `RESIDENCE_PERMIT` take a front and a back. Each value is a direct upload ID, or a `file_`-prefixed attachment ID to reuse an uploaded document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents:
        Option<UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemDocuments>,
    /// Answer for a `files` item — one document, as a list of its pages, first page first. Each entry is a direct upload ID, or a `file_`-prefixed attachment ID to reuse an uploaded document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    /// Item ID from `requested_information`.
    #[serde(default)]
    pub id: String,
    /// Answer for `text`, `date`, `phone`, and `select` items, and the chosen document type for a `file` item that lists `options`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Whether `value` is raw input or a vault token.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_type:
        Option<UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemValueType>,
}

impl UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItem {
    pub fn builder() -> UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemBuilder
    {
        <UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemBuilder {
    address: Option<UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemAddress>,
    documents:
        Option<UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemDocuments>,
    files: Option<Vec<String>>,
    id: Option<String>,
    value: Option<String>,
    value_type:
        Option<UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemValueType>,
}

impl UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemBuilder {
    pub fn address(
        mut self,
        value: UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemAddress,
    ) -> Self {
        self.address = Some(value);
        self
    }

    pub fn documents(
        mut self,
        value: UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemDocuments,
    ) -> Self {
        self.documents = Some(value);
        self
    }

    pub fn files(mut self, value: Vec<String>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn value_type(
        mut self,
        value: UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemValueType,
    ) -> Self {
        self.value_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItemBuilder::id)
    pub fn build(
        self,
    ) -> Result<UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItem, BuildError>
    {
        Ok(
            UpdateVerificationsRequestBodyPersonalAddressRequestedInformationItem {
                address: self.address,
                documents: self.documents,
                files: self.files,
                id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
                value: self.value,
                value_type: self.value_type,
            },
        )
    }
}
