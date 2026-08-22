pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdLeadFormDisclaimerCheckbox {
    /// Whether the checkbox starts ticked.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checked_by_default: Option<bool>,
    /// Stable identifier consent responses are stored under.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Whether the checkbox must be ticked to submit the form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Consent text next to the checkbox.
    #[serde(default)]
    pub text: String,
}

impl AdLeadFormDisclaimerCheckbox {
    pub fn builder() -> AdLeadFormDisclaimerCheckboxBuilder {
        <AdLeadFormDisclaimerCheckboxBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdLeadFormDisclaimerCheckboxBuilder {
    checked_by_default: Option<bool>,
    key: Option<String>,
    required: Option<bool>,
    text: Option<String>,
}

impl AdLeadFormDisclaimerCheckboxBuilder {
    pub fn checked_by_default(mut self, value: bool) -> Self {
        self.checked_by_default = Some(value);
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdLeadFormDisclaimerCheckbox`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](AdLeadFormDisclaimerCheckboxBuilder::text)
    pub fn build(self) -> Result<AdLeadFormDisclaimerCheckbox, BuildError> {
        Ok(AdLeadFormDisclaimerCheckbox {
            checked_by_default: self.checked_by_default,
            key: self.key,
            required: self.required,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
