pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialAccountLeadForm {
    /// Screen shown after the form is submitted. `null` when the form has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion: Option<AdLeadFormCompletion>,
    /// When the form was created, as an ISO 8601 timestamp.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Custom consent disclaimer shown before submission. `null` when the form has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disclaimer: Option<AdLeadFormDisclaimer>,
    /// `more_volume` is quickest to submit; `higher_intent` adds a confirmation step before submission.
    pub form_type: SocialAccountLeadFormFormType,
    /// The ad platform's identifier for the form. Use it as lead_gen_form_id on an ad to reuse the form.
    #[serde(default)]
    pub id: String,
    /// Intro screen shown before the questions. `null` when the form has none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intro: Option<AdLeadFormIntro>,
    /// Language the form is shown in, such as en_US.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// Advertiser-facing form name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Custom link text for the privacy policy. `null` when the default is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_link_text: Option<String>,
    /// Privacy policy URL configured on the form.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,
    #[serde(default)]
    pub question_labels: Vec<String>,
    #[serde(default)]
    pub questions: Vec<AdLeadFormQuestion>,
}

impl SocialAccountLeadForm {
    pub fn builder() -> SocialAccountLeadFormBuilder {
        <SocialAccountLeadFormBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SocialAccountLeadFormBuilder {
    completion: Option<AdLeadFormCompletion>,
    created_at: Option<String>,
    disclaimer: Option<AdLeadFormDisclaimer>,
    form_type: Option<SocialAccountLeadFormFormType>,
    id: Option<String>,
    intro: Option<AdLeadFormIntro>,
    locale: Option<String>,
    name: Option<String>,
    privacy_policy_link_text: Option<String>,
    privacy_policy_url: Option<String>,
    question_labels: Option<Vec<String>>,
    questions: Option<Vec<AdLeadFormQuestion>>,
}

impl SocialAccountLeadFormBuilder {
    pub fn completion(mut self, value: AdLeadFormCompletion) -> Self {
        self.completion = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn disclaimer(mut self, value: AdLeadFormDisclaimer) -> Self {
        self.disclaimer = Some(value);
        self
    }

    pub fn form_type(mut self, value: SocialAccountLeadFormFormType) -> Self {
        self.form_type = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn intro(mut self, value: AdLeadFormIntro) -> Self {
        self.intro = Some(value);
        self
    }

    pub fn locale(mut self, value: impl Into<String>) -> Self {
        self.locale = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn privacy_policy_link_text(mut self, value: impl Into<String>) -> Self {
        self.privacy_policy_link_text = Some(value.into());
        self
    }

    pub fn privacy_policy_url(mut self, value: impl Into<String>) -> Self {
        self.privacy_policy_url = Some(value.into());
        self
    }

    pub fn question_labels(mut self, value: Vec<String>) -> Self {
        self.question_labels = Some(value);
        self
    }

    pub fn questions(mut self, value: Vec<AdLeadFormQuestion>) -> Self {
        self.questions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SocialAccountLeadForm`].
    /// This method will fail if any of the following fields are not set:
    /// - [`form_type`](SocialAccountLeadFormBuilder::form_type)
    /// - [`id`](SocialAccountLeadFormBuilder::id)
    /// - [`question_labels`](SocialAccountLeadFormBuilder::question_labels)
    /// - [`questions`](SocialAccountLeadFormBuilder::questions)
    pub fn build(self) -> Result<SocialAccountLeadForm, BuildError> {
        Ok(SocialAccountLeadForm {
            completion: self.completion,
            created_at: self.created_at,
            disclaimer: self.disclaimer,
            form_type: self
                .form_type
                .ok_or_else(|| BuildError::missing_field("form_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            intro: self.intro,
            locale: self.locale,
            name: self.name,
            privacy_policy_link_text: self.privacy_policy_link_text,
            privacy_policy_url: self.privacy_policy_url,
            question_labels: self
                .question_labels
                .ok_or_else(|| BuildError::missing_field("question_labels"))?,
            questions: self
                .questions
                .ok_or_else(|| BuildError::missing_field("questions"))?,
        })
    }
}
