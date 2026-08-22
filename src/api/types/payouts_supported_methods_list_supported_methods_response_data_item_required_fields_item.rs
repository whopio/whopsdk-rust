pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListSupportedMethodsResponseDataItemRequiredFieldsItem {
    /// Stable field ID (`fld_` + the semantic type), used as the field key when creating the payout method. Safe to hardcode — it never changes for a given field.
    #[serde(default)]
    pub id: String,
    /// How to collect the value: `text`, `options`, or `date`.
    #[serde(default)]
    pub input_type: String,
    /// Human-readable field name.
    #[serde(default)]
    pub label: String,
    pub object: ListSupportedMethodsResponseDataItemRequiredFieldsItemObject,
    /// Allowed values for options fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    /// Example value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Whether the field must be provided.
    #[serde(default)]
    pub required: bool,
    /// Whether the value is vaulted in transit and never stored raw.
    #[serde(default)]
    pub sensitive: bool,
    /// Semantic field type, for example `bank_account_number` or `swift`.
    #[serde(default)]
    pub r#type: String,
    /// Regex the value must match. Null for options fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<String>,
}

impl ListSupportedMethodsResponseDataItemRequiredFieldsItem {
    pub fn builder() -> ListSupportedMethodsResponseDataItemRequiredFieldsItemBuilder {
        <ListSupportedMethodsResponseDataItemRequiredFieldsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSupportedMethodsResponseDataItemRequiredFieldsItemBuilder {
    id: Option<String>,
    input_type: Option<String>,
    label: Option<String>,
    object: Option<ListSupportedMethodsResponseDataItemRequiredFieldsItemObject>,
    options: Option<Vec<String>>,
    placeholder: Option<String>,
    required: Option<bool>,
    sensitive: Option<bool>,
    r#type: Option<String>,
    validation: Option<String>,
}

impl ListSupportedMethodsResponseDataItemRequiredFieldsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn input_type(mut self, value: impl Into<String>) -> Self {
        self.input_type = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn object(
        mut self,
        value: ListSupportedMethodsResponseDataItemRequiredFieldsItemObject,
    ) -> Self {
        self.object = Some(value);
        self
    }

    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
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

    pub fn sensitive(mut self, value: bool) -> Self {
        self.sensitive = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn validation(mut self, value: impl Into<String>) -> Self {
        self.validation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListSupportedMethodsResponseDataItemRequiredFieldsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ListSupportedMethodsResponseDataItemRequiredFieldsItemBuilder::id)
    /// - [`input_type`](ListSupportedMethodsResponseDataItemRequiredFieldsItemBuilder::input_type)
    /// - [`label`](ListSupportedMethodsResponseDataItemRequiredFieldsItemBuilder::label)
    /// - [`object`](ListSupportedMethodsResponseDataItemRequiredFieldsItemBuilder::object)
    /// - [`required`](ListSupportedMethodsResponseDataItemRequiredFieldsItemBuilder::required)
    /// - [`sensitive`](ListSupportedMethodsResponseDataItemRequiredFieldsItemBuilder::sensitive)
    /// - [`r#type`](ListSupportedMethodsResponseDataItemRequiredFieldsItemBuilder::r#type)
    pub fn build(
        self,
    ) -> Result<ListSupportedMethodsResponseDataItemRequiredFieldsItem, BuildError> {
        Ok(ListSupportedMethodsResponseDataItemRequiredFieldsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            input_type: self
                .input_type
                .ok_or_else(|| BuildError::missing_field("input_type"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            object: self
                .object
                .ok_or_else(|| BuildError::missing_field("object"))?,
            options: self.options,
            placeholder: self.placeholder,
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
            sensitive: self
                .sensitive
                .ok_or_else(|| BuildError::missing_field("sensitive"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            validation: self.validation,
        })
    }
}
