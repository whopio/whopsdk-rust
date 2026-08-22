pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdLeadForm {
    /// Screen shown after the form is submitted. `null` when the form uses the default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<AdLeadFormCompletion>,
    /// Custom consent disclaimer shown before submission. `null` when the form has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclaimer: Option<AdLeadFormDisclaimer>,
    /// `more_volume` is quickest to submit; `higher_intent` adds a confirmation step before submission.
    pub form_type: AdLeadFormFormType,
    /// Intro screen shown before the questions. `null` when the form has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<AdLeadFormIntro>,
    /// Internal name of the form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Whether the phone number must be verified by SMS before submitting.
    #[serde(default)]
    pub phone_verification: bool,
    /// Your privacy policy, linked from the form. `null` when unset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<AdLeadFormPrivacyPolicy>,
    #[serde(default)]
    pub questions: Vec<AdLeadFormQuestion>,
}

impl AdLeadForm {
    pub fn builder() -> AdLeadFormBuilder {
        <AdLeadFormBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdLeadFormBuilder {
    completion: Option<AdLeadFormCompletion>,
    disclaimer: Option<AdLeadFormDisclaimer>,
    form_type: Option<AdLeadFormFormType>,
    intro: Option<AdLeadFormIntro>,
    name: Option<String>,
    phone_verification: Option<bool>,
    privacy_policy: Option<AdLeadFormPrivacyPolicy>,
    questions: Option<Vec<AdLeadFormQuestion>>,
}

impl AdLeadFormBuilder {
    pub fn completion(mut self, value: AdLeadFormCompletion) -> Self {
        self.completion = Some(value);
        self
    }

    pub fn disclaimer(mut self, value: AdLeadFormDisclaimer) -> Self {
        self.disclaimer = Some(value);
        self
    }

    pub fn form_type(mut self, value: AdLeadFormFormType) -> Self {
        self.form_type = Some(value);
        self
    }

    pub fn intro(mut self, value: AdLeadFormIntro) -> Self {
        self.intro = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn phone_verification(mut self, value: bool) -> Self {
        self.phone_verification = Some(value);
        self
    }

    pub fn privacy_policy(mut self, value: AdLeadFormPrivacyPolicy) -> Self {
        self.privacy_policy = Some(value);
        self
    }

    pub fn questions(mut self, value: Vec<AdLeadFormQuestion>) -> Self {
        self.questions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdLeadForm`].
    /// This method will fail if any of the following fields are not set:
    /// - [`form_type`](AdLeadFormBuilder::form_type)
    /// - [`phone_verification`](AdLeadFormBuilder::phone_verification)
    /// - [`questions`](AdLeadFormBuilder::questions)
    pub fn build(self) -> Result<AdLeadForm, BuildError> {
        Ok(AdLeadForm {
            completion: self.completion,
            disclaimer: self.disclaimer,
            form_type: self
                .form_type
                .ok_or_else(|| BuildError::missing_field("form_type"))?,
            intro: self.intro,
            name: self.name,
            phone_verification: self
                .phone_verification
                .ok_or_else(|| BuildError::missing_field("phone_verification"))?,
            privacy_policy: self.privacy_policy,
            questions: self
                .questions
                .ok_or_else(|| BuildError::missing_field("questions"))?,
        })
    }
}
