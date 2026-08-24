pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePlansRequestCustomFieldsItem {
    /// The type of the custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<CreatePlansRequestCustomFieldsItemFieldType>,
    /// The ID of the custom field (if being updated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// The name of the custom field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The order of the field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<i64>,
    /// An example response displayed in the input field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Whether or not the field is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

impl CreatePlansRequestCustomFieldsItem {
    pub fn builder() -> CreatePlansRequestCustomFieldsItemBuilder {
        <CreatePlansRequestCustomFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePlansRequestCustomFieldsItemBuilder {
    field_type: Option<CreatePlansRequestCustomFieldsItemFieldType>,
    id: Option<String>,
    name: Option<String>,
    order: Option<i64>,
    placeholder: Option<String>,
    required: Option<bool>,
}

impl CreatePlansRequestCustomFieldsItemBuilder {
    pub fn field_type(mut self, value: CreatePlansRequestCustomFieldsItemFieldType) -> Self {
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

    /// Consumes the builder and constructs a [`CreatePlansRequestCustomFieldsItem`].
    pub fn build(self) -> Result<CreatePlansRequestCustomFieldsItem, BuildError> {
        Ok(CreatePlansRequestCustomFieldsItem {
            field_type: self.field_type,
            id: self.id,
            name: self.name,
            order: self.order,
            placeholder: self.placeholder,
            required: self.required,
        })
    }
}
