pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateInvoicesRequestProductPlanCustomFieldsItem {
    /// The type of the custom field.
    pub field_type: CustomFieldTypes,
    /// The ID of the custom field (if being updated)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the custom field.
    #[serde(default)]
    pub name: String,
    /// The order of the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    /// The placeholder value of the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Whether or not the field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl CreateInvoicesRequestProductPlanCustomFieldsItem {
    pub fn builder() -> CreateInvoicesRequestProductPlanCustomFieldsItemBuilder {
        <CreateInvoicesRequestProductPlanCustomFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateInvoicesRequestProductPlanCustomFieldsItemBuilder {
    field_type: Option<CustomFieldTypes>,
    id: Option<String>,
    name: Option<String>,
    order: Option<i64>,
    placeholder: Option<String>,
    required: Option<bool>,
}

impl CreateInvoicesRequestProductPlanCustomFieldsItemBuilder {
    pub fn field_type(mut self, value: CustomFieldTypes) -> Self {
        self.field_type = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn order(mut self, value: i64) -> Self {
        self.order = Some(value);
        self
    }

    pub fn placeholder(mut self, value: impl Into<String>) -> Self {
        self.placeholder = Some(value.into());
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateInvoicesRequestProductPlanCustomFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field_type`](CreateInvoicesRequestProductPlanCustomFieldsItemBuilder::field_type)
    /// - [`name`](CreateInvoicesRequestProductPlanCustomFieldsItemBuilder::name)
    pub fn build(self) -> Result<CreateInvoicesRequestProductPlanCustomFieldsItem, BuildError> {
        Ok(CreateInvoicesRequestProductPlanCustomFieldsItem {
            field_type: self
                .field_type
                .ok_or_else(|| BuildError::missing_field("field_type"))?,
            id: self.id,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            order: self.order,
            placeholder: self.placeholder,
            required: self.required,
        })
    }
}
