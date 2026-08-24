pub use crate::prelude::*;

/// The saved audience this event came from. Present on the identify events an audience ingest writes for each of its members.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEventsResponseDataItemRelatedAudience {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_type: Option<ListEventsResponseDataItemRelatedAudienceAudienceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<ListEventsResponseDataItemRelatedAudienceSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ListEventsResponseDataItemRelatedAudience {
    pub fn builder() -> ListEventsResponseDataItemRelatedAudienceBuilder {
        <ListEventsResponseDataItemRelatedAudienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemRelatedAudienceBuilder {
    audience_type: Option<ListEventsResponseDataItemRelatedAudienceAudienceType>,
    file_name: Option<String>,
    id: Option<String>,
    source_type: Option<ListEventsResponseDataItemRelatedAudienceSourceType>,
    title: Option<String>,
}

impl ListEventsResponseDataItemRelatedAudienceBuilder {
    pub fn audience_type(
        mut self,
        value: ListEventsResponseDataItemRelatedAudienceAudienceType,
    ) -> Self {
        self.audience_type = Some(value);
        self
    }

    pub fn file_name(mut self, value: impl Into<String>) -> Self {
        self.file_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn source_type(
        mut self,
        value: ListEventsResponseDataItemRelatedAudienceSourceType,
    ) -> Self {
        self.source_type = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEventsResponseDataItemRelatedAudience`].
    pub fn build(self) -> Result<ListEventsResponseDataItemRelatedAudience, BuildError> {
        Ok(ListEventsResponseDataItemRelatedAudience {
            audience_type: self.audience_type,
            file_name: self.file_name,
            id: self.id,
            source_type: self.source_type,
            title: self.title,
        })
    }
}
