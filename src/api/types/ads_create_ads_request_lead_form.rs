pub use crate::prelude::*;

/// Instant lead form for the ad. Only allowed when the ad group's conversion_location is an instant-form destination (instant_forms, instant_forms_and_messenger, website_and_instant_forms). Mutually exclusive with lead_form_id.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdsRequestLeadForm {
    /// Optional completion screen shown after submission; url sets the follow-up website button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<CreateAdsRequestLeadFormCompletion>,
    /// Optional custom consent disclaimer with checkboxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclaimer: Option<CreateAdsRequestLeadFormDisclaimer>,
    /// more_volume (default) is quickest to submit; higher_intent adds a confirmation step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_type: Option<CreateAdsRequestLeadFormFormType>,
    /// Optional intro screen shown before the questions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<CreateAdsRequestLeadFormIntro>,
    /// Internal name for the form. Auto-generated if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Require SMS verification of the phone number (higher_intent forms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_verification: Option<bool>,
    /// Your privacy policy. url is required by the ad platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<CreateAdsRequestLeadFormPrivacyPolicy>,
    /// The questions on the form. Standard prefill types need only a type; a custom question needs a label and a format (plus options for multiple_choice). Options carry an optional key and answer-routing logic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub questions: Option<Vec<CreateAdsRequestLeadFormQuestionsItem>>,
}

impl CreateAdsRequestLeadForm {
    pub fn builder() -> CreateAdsRequestLeadFormBuilder {
        <CreateAdsRequestLeadFormBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdsRequestLeadFormBuilder {
    completion: Option<CreateAdsRequestLeadFormCompletion>,
    disclaimer: Option<CreateAdsRequestLeadFormDisclaimer>,
    form_type: Option<CreateAdsRequestLeadFormFormType>,
    intro: Option<CreateAdsRequestLeadFormIntro>,
    name: Option<String>,
    phone_verification: Option<bool>,
    privacy_policy: Option<CreateAdsRequestLeadFormPrivacyPolicy>,
    questions: Option<Vec<CreateAdsRequestLeadFormQuestionsItem>>,
}

impl CreateAdsRequestLeadFormBuilder {
    pub fn completion(mut self, value: CreateAdsRequestLeadFormCompletion) -> Self {
        self.completion = Some(value);
        self
    }

    pub fn disclaimer(mut self, value: CreateAdsRequestLeadFormDisclaimer) -> Self {
        self.disclaimer = Some(value);
        self
    }

    pub fn form_type(mut self, value: CreateAdsRequestLeadFormFormType) -> Self {
        self.form_type = Some(value);
        self
    }

    pub fn intro(mut self, value: CreateAdsRequestLeadFormIntro) -> Self {
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

    pub fn privacy_policy(mut self, value: CreateAdsRequestLeadFormPrivacyPolicy) -> Self {
        self.privacy_policy = Some(value);
        self
    }

    pub fn questions(mut self, value: Vec<CreateAdsRequestLeadFormQuestionsItem>) -> Self {
        self.questions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAdsRequestLeadForm`].
    pub fn build(self) -> Result<CreateAdsRequestLeadForm, BuildError> {
        Ok(CreateAdsRequestLeadForm {
            completion: self.completion,
            disclaimer: self.disclaimer,
            form_type: self.form_type,
            intro: self.intro,
            name: self.name,
            phone_verification: self.phone_verification,
            privacy_policy: self.privacy_policy,
            questions: self.questions,
        })
    }
}
