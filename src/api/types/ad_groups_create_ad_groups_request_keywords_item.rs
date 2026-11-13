pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateAdGroupsRequestKeywordsItem {
    /// How closely a search has to match: `broad` also reaches related searches, `phrase` needs the meaning of the phrase, and `exact` needs the same meaning as the term. Defaults to `broad`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<CreateAdGroupsRequestKeywordsItemMatchType>,
    /// Set to `true` to keep the ads from showing for this term instead. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative: Option<bool>,
    /// The search term, up to 80 characters and 10 words. Symbols such as ! @ % , * = ^ ; ~ | < > ? ( ) { } and quotes aren't allowed: set the match with match_type instead.
    #[serde(default)]
    pub text: String,
}

impl CreateAdGroupsRequestKeywordsItem {
    pub fn builder() -> CreateAdGroupsRequestKeywordsItemBuilder {
        <CreateAdGroupsRequestKeywordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAdGroupsRequestKeywordsItemBuilder {
    match_type: Option<CreateAdGroupsRequestKeywordsItemMatchType>,
    negative: Option<bool>,
    text: Option<String>,
}

impl CreateAdGroupsRequestKeywordsItemBuilder {
    pub fn match_type(mut self, value: CreateAdGroupsRequestKeywordsItemMatchType) -> Self {
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

    /// Consumes the builder and constructs a [`CreateAdGroupsRequestKeywordsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](CreateAdGroupsRequestKeywordsItemBuilder::text)
    pub fn build(self) -> Result<CreateAdGroupsRequestKeywordsItem, BuildError> {
        Ok(CreateAdGroupsRequestKeywordsItem {
            match_type: self.match_type,
            negative: self.negative,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
