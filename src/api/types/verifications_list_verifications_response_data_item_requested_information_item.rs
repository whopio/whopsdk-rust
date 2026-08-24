pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListVerificationsResponseDataItemRequestedInformationItem {
    /// Present after a rejected submission.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ListVerificationsResponseDataItemRequestedInformationItemErrorsItem>>,
    /// Requested information item ID, prefixed `inrqi_`.
    #[serde(default)]
    pub id: String,
    /// Instruction to show the user. Carries the reviewer's note verbatim when there is one.
    #[serde(default)]
    pub label: String,
    /// `true` when the item can be skipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional: Option<bool>,
    /// The values `value` may take on a `select` item. On an `id_document` item these are the ID types accepted, and the chosen one decides which `documents` slots to send. Absent when the item has no choice to make.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    /// What is needed: a document name such as `bank_statement`, or a field key such as `ssn` or `identity_document`. Handle unrecognized values by `type`.
    #[serde(default)]
    pub requirement: String,
    /// What to send as the answer, so you never have to infer it: `files` (a document, as a list of its pages), `id_document` (send `documents` with the slot keys for the ID you are uploading), `text`, `date`, `phone` or `select` (send `value`), or `address` (send `address`).
    #[serde(default)]
    pub r#type: String,
}

impl ListVerificationsResponseDataItemRequestedInformationItem {
    pub fn builder() -> ListVerificationsResponseDataItemRequestedInformationItemBuilder {
        <ListVerificationsResponseDataItemRequestedInformationItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVerificationsResponseDataItemRequestedInformationItemBuilder {
    errors: Option<Vec<ListVerificationsResponseDataItemRequestedInformationItemErrorsItem>>,
    id: Option<String>,
    label: Option<String>,
    optional: Option<bool>,
    options: Option<Vec<String>>,
    requirement: Option<String>,
    r#type: Option<String>,
}

impl ListVerificationsResponseDataItemRequestedInformationItemBuilder {
    pub fn errors(
        mut self,
        value: Vec<ListVerificationsResponseDataItemRequestedInformationItemErrorsItem>,
    ) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn optional(mut self, value: bool) -> Self {
        self.optional = Some(value);
        self
    }

    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn requirement(mut self, value: impl Into<String>) -> Self {
        self.requirement = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListVerificationsResponseDataItemRequestedInformationItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListVerificationsResponseDataItemRequestedInformationItemBuilder::id)
    /// - [`label`](ListVerificationsResponseDataItemRequestedInformationItemBuilder::label)
    /// - [`requirement`](ListVerificationsResponseDataItemRequestedInformationItemBuilder::requirement)
    /// - [`r#type`](ListVerificationsResponseDataItemRequestedInformationItemBuilder::r#type)
    pub fn build(
        self,
    ) -> Result<ListVerificationsResponseDataItemRequestedInformationItem, BuildError> {
        Ok(ListVerificationsResponseDataItemRequestedInformationItem {
            errors: self.errors,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            optional: self.optional,
            options: self.options,
            requirement: self
                .requirement
                .ok_or_else(|| BuildError::missing_field("requirement"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
