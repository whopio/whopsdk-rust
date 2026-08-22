pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PlanCustomField {
    /// Custom field input type.
    pub field_type: PlanCustomFieldFieldType,
    /// Custom field ID, prefixed `field_`.
    #[serde(default)]
    pub id: String,
    /// Field label shown to customer at checkout.
    #[serde(default)]
    pub name: String,
    /// Field position on checkout form.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub order: f64,
    /// Placeholder text shown in the empty field. `null` if none is set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
    /// Whether the customer must complete this field to check out.
    #[serde(default)]
    pub required: bool,
}

impl PlanCustomField {
    pub fn builder() -> PlanCustomFieldBuilder {
        <PlanCustomFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PlanCustomFieldBuilder {
    field_type: Option<PlanCustomFieldFieldType>,
    id: Option<String>,
    name: Option<String>,
    order: Option<f64>,
    placeholder: Option<String>,
    required: Option<bool>,
}

impl PlanCustomFieldBuilder {
    pub fn field_type(mut self, value: PlanCustomFieldFieldType) -> Self {
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

    pub fn order(mut self, value: f64) -> Self {
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

    /// Consumes the builder and constructs a [`PlanCustomField`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field_type`](PlanCustomFieldBuilder::field_type)
    /// - [`id`](PlanCustomFieldBuilder::id)
    /// - [`name`](PlanCustomFieldBuilder::name)
    /// - [`order`](PlanCustomFieldBuilder::order)
    /// - [`required`](PlanCustomFieldBuilder::required)
    pub fn build(self) -> Result<PlanCustomField, BuildError> {
        Ok(PlanCustomField {
            field_type: self
                .field_type
                .ok_or_else(|| BuildError::missing_field("field_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            order: self
                .order
                .ok_or_else(|| BuildError::missing_field("order"))?,
            placeholder: self.placeholder,
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
        })
    }
}
