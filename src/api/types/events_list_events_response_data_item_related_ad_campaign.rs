pub use crate::prelude::*;

/// The Whop ad campaign this event's click resolved to, read from the ad entity tree rather than the click's url params.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEventsResponseDataItemRelatedAdCampaign {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

impl ListEventsResponseDataItemRelatedAdCampaign {
    pub fn builder() -> ListEventsResponseDataItemRelatedAdCampaignBuilder {
        <ListEventsResponseDataItemRelatedAdCampaignBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemRelatedAdCampaignBuilder {
    id: Option<String>,
    platform: Option<String>,
    title: Option<String>,
}

impl ListEventsResponseDataItemRelatedAdCampaignBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn platform(mut self, value: impl Into<String>) -> Self {
        self.platform = Some(value.into());
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEventsResponseDataItemRelatedAdCampaign`].
    pub fn build(self) -> Result<ListEventsResponseDataItemRelatedAdCampaign, BuildError> {
        Ok(ListEventsResponseDataItemRelatedAdCampaign {
            id: self.id,
            platform: self.platform,
            title: self.title,
        })
    }
}
