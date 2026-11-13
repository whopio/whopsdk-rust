pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateAdGroupsRequestKeywordsItem {
    /// How closely a search has to match: `broad` also reaches related searches, `phrase` needs the meaning of the phrase, and `exact` needs the same meaning as the term. Defaults to `broad`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<UpdateAdGroupsRequestKeywordsItemMatchType>,
    /// Set to `true` to keep the ads from showing for this term instead. Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative: Option<bool>,
    /// The search term, up to 80 characters and 10 words. Symbols such as ! @ % , * = ^ ; ~ | < > ? ( ) { } and quotes aren't allowed: set the match with match_type instead.
    #[serde(default)]
    pub text: String,
}

impl UpdateAdGroupsRequestKeywordsItem {
    pub fn builder() -> UpdateAdGroupsRequestKeywordsItemBuilder {
        <UpdateAdGroupsRequestKeywordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateAdGroupsRequestKeywordsItemBuilder {
    match_type: Option<UpdateAdGroupsRequestKeywordsItemMatchType>,
    negative: Option<bool>,
    text: Option<String>,
}

impl UpdateAdGroupsRequestKeywordsItemBuilder {
    pub fn match_type(mut self, value: UpdateAdGroupsRequestKeywordsItemMatchType) -> Self {
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

    /// Consumes the builder and constructs a [`UpdateAdGroupsRequestKeywordsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`text`](UpdateAdGroupsRequestKeywordsItemBuilder::text)
    pub fn build(self) -> Result<UpdateAdGroupsRequestKeywordsItem, BuildError> {
        Ok(UpdateAdGroupsRequestKeywordsItem {
            match_type: self.match_type,
            negative: self.negative,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
