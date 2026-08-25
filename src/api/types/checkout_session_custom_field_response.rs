pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckoutSessionCustomFieldResponse {
    /// The plan custom field this answers, prefixed `field_` — the `id` of the matching entry published by the `custom_fields` requirement.
    #[serde(default)]
    pub id: String,
    /// The buyer's answer, as typed.
    #[serde(default)]
    pub value: String,
}

impl CheckoutSessionCustomFieldResponse {
    pub fn builder() -> CheckoutSessionCustomFieldResponseBuilder {
        <CheckoutSessionCustomFieldResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckoutSessionCustomFieldResponseBuilder {
    id: Option<String>,
    value: Option<String>,
}

impl CheckoutSessionCustomFieldResponseBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckoutSessionCustomFieldResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CheckoutSessionCustomFieldResponseBuilder::id)
    /// - [`value`](CheckoutSessionCustomFieldResponseBuilder::value)
    pub fn build(self) -> Result<CheckoutSessionCustomFieldResponse, BuildError> {
        Ok(CheckoutSessionCustomFieldResponse {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
