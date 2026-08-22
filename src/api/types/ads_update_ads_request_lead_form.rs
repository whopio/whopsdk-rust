pub use crate::prelude::*;

/// Instant lead form for the ad. Only allowed when the ad group's conversion_location is an instant-form destination (instant_forms, instant_forms_and_messenger, website_and_instant_forms). Mutually exclusive with lead_form_id.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdsRequestLeadForm {
    /// Optional completion screen shown after submission; url sets the follow-up website button.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<UpdateAdsRequestLeadFormCompletion>,
    /// Optional custom consent disclaimer with checkboxes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclaimer: Option<UpdateAdsRequestLeadFormDisclaimer>,
    /// more_volume (default) is quickest to submit; higher_intent adds a confirmation step.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_type: Option<UpdateAdsRequestLeadFormFormType>,
    /// Optional intro screen shown before the questions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<UpdateAdsRequestLeadFormIntro>,
    /// Internal name for the form. Auto-generated if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Require SMS verification of the phone number (higher_intent forms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_verification: Option<bool>,
    /// Your privacy policy. url is required by the ad platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy: Option<UpdateAdsRequestLeadFormPrivacyPolicy>,
    /// The questions on the form. Standard prefill types need only a type; a custom question needs a label and a format (plus options for multiple_choice). Options carry an optional key and answer-routing logic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub questions: Option<Vec<UpdateAdsRequestLeadFormQuestionsItem>>,
}

impl UpdateAdsRequestLeadForm {
    pub fn builder() -> UpdateAdsRequestLeadFormBuilder {
        <UpdateAdsRequestLeadFormBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdsRequestLeadFormBuilder {
    completion: Option<UpdateAdsRequestLeadFormCompletion>,
    disclaimer: Option<UpdateAdsRequestLeadFormDisclaimer>,
    form_type: Option<UpdateAdsRequestLeadFormFormType>,
    intro: Option<UpdateAdsRequestLeadFormIntro>,
    name: Option<String>,
    phone_verification: Option<bool>,
    privacy_policy: Option<UpdateAdsRequestLeadFormPrivacyPolicy>,
    questions: Option<Vec<UpdateAdsRequestLeadFormQuestionsItem>>,
}

impl UpdateAdsRequestLeadFormBuilder {
    pub fn completion(mut self, value: UpdateAdsRequestLeadFormCompletion) -> Self {
        self.completion = Some(value);
        self
    }

    pub fn disclaimer(mut self, value: UpdateAdsRequestLeadFormDisclaimer) -> Self {
        self.disclaimer = Some(value);
        self
    }

    pub fn form_type(mut self, value: UpdateAdsRequestLeadFormFormType) -> Self {
        self.form_type = Some(value);
        self
    }

    pub fn intro(mut self, value: UpdateAdsRequestLeadFormIntro) -> Self {
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

    pub fn privacy_policy(mut self, value: UpdateAdsRequestLeadFormPrivacyPolicy) -> Self {
        self.privacy_policy = Some(value);
        self
    }

    pub fn questions(mut self, value: Vec<UpdateAdsRequestLeadFormQuestionsItem>) -> Self {
        self.questions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateAdsRequestLeadForm`].
    pub fn build(self) -> Result<UpdateAdsRequestLeadForm, BuildError> {
        Ok(UpdateAdsRequestLeadForm {
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
