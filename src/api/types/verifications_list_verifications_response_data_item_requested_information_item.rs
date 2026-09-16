pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListVerificationsResponseDataItemRequestedInformationItem {
    /// URL for a related action, such as completing liveness verification or viewing a payment. Absent when no action is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_url: Option<String>,
    /// Follow-up prompt shown with this requirement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_label: Option<String>,
    /// Whether the follow-up response is required when visible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_required: Option<bool>,
    /// Selected option values that make the follow-up prompt visible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_visible_for: Option<Vec<String>>,
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
    /// Optional native input format for a text response.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_type:
        Option<ListVerificationsResponseDataItemRequestedInformationItemResponseType>,
    /// Whether a question with `options` accepts one value or multiple values.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection_mode:
        Option<ListVerificationsResponseDataItemRequestedInformationItemSelectionMode>,
    /// Documents supplied with the requirement for context.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_documents: Option<Vec<File>>,
    /// Whether a written explanation may replace required supporting files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_files_explanation_allowed: Option<bool>,
    /// Whether this requirement also needs supporting files.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_files_required: Option<bool>,
    /// Selected option values that make the supporting-file input visible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_files_visible_for: Option<Vec<String>>,
    /// What to send as the answer, so you never have to infer it: `files` (a document, as a list of its pages), `id_document` (send `documents` with the slot keys for the ID you are uploading), `text`, `date`, `phone` or `select` (send `value`), `text_with_files` (send `value` and optional `files`), `address` (send `address`), or `liveness` (open `action_url`, then send `value` as `true` after completion).
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
    action_url: Option<String>,
    details_label: Option<String>,
    details_required: Option<bool>,
    details_visible_for: Option<Vec<String>>,
    errors: Option<Vec<ListVerificationsResponseDataItemRequestedInformationItemErrorsItem>>,
    id: Option<String>,
    label: Option<String>,
    optional: Option<bool>,
    options: Option<Vec<String>>,
    requirement: Option<String>,
    response_type: Option<ListVerificationsResponseDataItemRequestedInformationItemResponseType>,
    selection_mode: Option<ListVerificationsResponseDataItemRequestedInformationItemSelectionMode>,
    supporting_documents: Option<Vec<File>>,
    supporting_files_explanation_allowed: Option<bool>,
    supporting_files_required: Option<bool>,
    supporting_files_visible_for: Option<Vec<String>>,
    r#type: Option<String>,
}

impl ListVerificationsResponseDataItemRequestedInformationItemBuilder {
    pub fn action_url(mut self, value: impl Into<String>) -> Self {
        self.action_url = Some(value.into());
        self
    }

    pub fn details_label(mut self, value: impl Into<String>) -> Self {
        self.details_label = Some(value.into());
        self
    }

    pub fn details_required(mut self, value: bool) -> Self {
        self.details_required = Some(value);
        self
    }

    pub fn details_visible_for(mut self, value: Vec<String>) -> Self {
        self.details_visible_for = Some(value);
        self
    }

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

    pub fn response_type(
        mut self,
        value: ListVerificationsResponseDataItemRequestedInformationItemResponseType,
    ) -> Self {
        self.response_type = Some(value);
        self
    }

    pub fn selection_mode(
        mut self,
        value: ListVerificationsResponseDataItemRequestedInformationItemSelectionMode,
    ) -> Self {
        self.selection_mode = Some(value);
        self
    }

    pub fn supporting_documents(mut self, value: Vec<File>) -> Self {
        self.supporting_documents = Some(value);
        self
    }

    pub fn supporting_files_explanation_allowed(mut self, value: bool) -> Self {
        self.supporting_files_explanation_allowed = Some(value);
        self
    }

    pub fn supporting_files_required(mut self, value: bool) -> Self {
        self.supporting_files_required = Some(value);
        self
    }

    pub fn supporting_files_visible_for(mut self, value: Vec<String>) -> Self {
        self.supporting_files_visible_for = Some(value);
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
            action_url: self.action_url,
            details_label: self.details_label,
            details_required: self.details_required,
            details_visible_for: self.details_visible_for,
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
            response_type: self.response_type,
            selection_mode: self.selection_mode,
            supporting_documents: self.supporting_documents,
            supporting_files_explanation_allowed: self.supporting_files_explanation_allowed,
            supporting_files_required: self.supporting_files_required,
            supporting_files_visible_for: self.supporting_files_visible_for,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
