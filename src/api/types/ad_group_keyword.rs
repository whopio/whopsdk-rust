pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AdGroupKeyword {
    /// How closely a search has to match: `broad` also reaches related searches, `phrase` needs the meaning of the phrase, and `exact` needs the same meaning as the term. Defaults to `broad`.
    pub match_type: AdGroupKeywordMatchType,
    /// Whether this term keeps the ads from showing instead of making them eligible. Defaults to `false`.
    #[serde(default)]
    pub negative: bool,
    /// The search term, up to 80 characters and 10 words.
    #[serde(default)]
    pub text: String,
}

impl AdGroupKeyword {
    pub fn builder() -> AdGroupKeywordBuilder {
        <AdGroupKeywordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdGroupKeywordBuilder {
    match_type: Option<AdGroupKeywordMatchType>,
    negative: Option<bool>,
    text: Option<String>,
}

impl AdGroupKeywordBuilder {
    pub fn match_type(mut self, value: AdGroupKeywordMatchType) -> Self {
        self.match_type = Some(value);
        self
    }

    pub fn negative(mut self, value: bool) -> Self {
        self.negative = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AdGroupKeyword`].
    /// This method will fail if any of the following fields are not set:
    /// - [`match_type`](AdGroupKeywordBuilder::match_type)
    /// - [`negative`](AdGroupKeywordBuilder::negative)
    /// - [`text`](AdGroupKeywordBuilder::text)
    pub fn build(self) -> Result<AdGroupKeyword, BuildError> {
        Ok(AdGroupKeyword {
            match_type: self
                .match_type
                .ok_or_else(|| BuildError::missing_field("match_type"))?,
            negative: self
                .negative
                .ok_or_else(|| BuildError::missing_field("negative"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
