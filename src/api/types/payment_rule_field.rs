pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaymentRuleField {
    /// The payment attribute this condition reads.
    pub field: PaymentRuleFieldField,
    /// The largest value this field accepts, or `null` when the field is not numeric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<i64>,
    /// The smallest value this field accepts, or `null` when the field is not numeric.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<i64>,
    #[serde(default)]
    pub operators: Vec<PaymentRuleFieldOperatorsItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<PaymentRuleFieldOption>>,
    /// The type of value this field compares against.
    pub r#type: PaymentRuleFieldType,
}

impl PaymentRuleField {
    pub fn builder() -> PaymentRuleFieldBuilder {
        <PaymentRuleFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentRuleFieldBuilder {
    field: Option<PaymentRuleFieldField>,
    maximum: Option<i64>,
    minimum: Option<i64>,
    operators: Option<Vec<PaymentRuleFieldOperatorsItem>>,
    options: Option<Vec<PaymentRuleFieldOption>>,
    r#type: Option<PaymentRuleFieldType>,
}

impl PaymentRuleFieldBuilder {
    pub fn field(mut self, value: PaymentRuleFieldField) -> Self {
        self.field = Some(value);
        self
    }

    pub fn maximum(mut self, value: i64) -> Self {
        self.maximum = Some(value);
        self
    }

    pub fn minimum(mut self, value: i64) -> Self {
        self.minimum = Some(value);
        self
    }

    pub fn operators(mut self, value: Vec<PaymentRuleFieldOperatorsItem>) -> Self {
        self.operators = Some(value);
        self
    }

    pub fn options(mut self, value: Vec<PaymentRuleFieldOption>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn r#type(mut self, value: PaymentRuleFieldType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentRuleField`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PaymentRuleFieldBuilder::field)
    /// - [`operators`](PaymentRuleFieldBuilder::operators)
    /// - [`r#type`](PaymentRuleFieldBuilder::r#type)
    pub fn build(self) -> Result<PaymentRuleField, BuildError> {
        Ok(PaymentRuleField {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            maximum: self.maximum,
            minimum: self.minimum,
            operators: self
                .operators
                .ok_or_else(|| BuildError::missing_field("operators"))?,
            options: self.options,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
        })
    }
}
