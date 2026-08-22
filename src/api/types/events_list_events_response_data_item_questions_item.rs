pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEventsResponseDataItemQuestionsItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub answer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub question: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ListEventsResponseDataItemQuestionsItem {
    pub fn builder() -> ListEventsResponseDataItemQuestionsItemBuilder {
        <ListEventsResponseDataItemQuestionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemQuestionsItemBuilder {
    answer: Option<String>,
    id: Option<String>,
    key: Option<String>,
    options: Option<Vec<String>>,
    question: Option<String>,
    r#type: Option<String>,
}

impl ListEventsResponseDataItemQuestionsItemBuilder {
    pub fn answer(mut self, value: impl Into<String>) -> Self {
        self.answer = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn key(mut self, value: impl Into<String>) -> Self {
        self.key = Some(value.into());
        self
    }

    pub fn options(mut self, value: Vec<String>) -> Self {
        self.options = Some(value);
        self
    }

    pub fn question(mut self, value: impl Into<String>) -> Self {
        self.question = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEventsResponseDataItemQuestionsItem`].
    pub fn build(self) -> Result<ListEventsResponseDataItemQuestionsItem, BuildError> {
        Ok(ListEventsResponseDataItemQuestionsItem {
            answer: self.answer,
            id: self.id,
            key: self.key,
            options: self.options,
            question: self.question,
            r#type: self.r#type,
        })
    }
}
