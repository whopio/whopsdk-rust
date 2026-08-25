pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CheckoutSessionCustomField {
    /// The seller's longer explanation of the question, or `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// What kind of input to render. `text` today.
    pub field_type: CheckoutSessionCustomFieldFieldType,
    /// The question's ID, prefixed `field_`. Echo it as the `id` of the matching `custom_field_responses` entry.
    #[serde(default)]
    pub id: String,
    /// The question as the seller wrote it — the field's label.
    #[serde(default)]
    pub name: String,
    /// Placeholder text for the input, or `null`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Whether confirm refuses without an answer. An unanswered required question is refused with `custom_field_invalid`.
    #[serde(default)]
    pub required: bool,
}

impl CheckoutSessionCustomField {
    pub fn builder() -> CheckoutSessionCustomFieldBuilder {
        <CheckoutSessionCustomFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionCustomFieldBuilder {
    description: Option<String>,
    field_type: Option<CheckoutSessionCustomFieldFieldType>,
    id: Option<String>,
    name: Option<String>,
    placeholder: Option<String>,
    required: Option<bool>,
}

impl CheckoutSessionCustomFieldBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn field_type(mut self, value: CheckoutSessionCustomFieldFieldType) -> Self {
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

    pub fn placeholder(mut self, value: impl Into<String>) -> Self {
        self.placeholder = Some(value.into());
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionCustomField`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field_type`](CheckoutSessionCustomFieldBuilder::field_type)
    /// - [`id`](CheckoutSessionCustomFieldBuilder::id)
    /// - [`name`](CheckoutSessionCustomFieldBuilder::name)
    /// - [`required`](CheckoutSessionCustomFieldBuilder::required)
    pub fn build(self) -> Result<CheckoutSessionCustomField, BuildError> {
        Ok(CheckoutSessionCustomField {
            description: self.description,
            field_type: self
                .field_type
                .ok_or_else(|| BuildError::missing_field("field_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            placeholder: self.placeholder,
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
        })
    }
}
