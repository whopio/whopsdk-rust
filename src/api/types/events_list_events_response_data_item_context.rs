pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEventsResponseDataItemContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_campaign_id: Option<String>,
    /// Stable identity for the ad click this event belongs to. Every event from one click carries the same value, so events group into clicks without re-deriving them.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_click_id: Option<String>,
    /// How the ad click was identified: the network's click-id param (`fbclid`, `ttclid`, `gclid`, `gbraid`, `wbraid`, `twclid`) or `synthetic` when the click carried none.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_click_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_set_id: Option<String>,
    /// How this event counts as an acquisition touch, using the same rule attribution credits a conversion with. `ad_click` and `lead_form` resolved to a Whop ad; `external_ad_click` is a paid click on a campaign run outside Whop; `referrer` is organic. Null when the event is not a touch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<ListEventsResponseDataItemContextSourceType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_campaign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_medium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_term: Option<String>,
}

impl ListEventsResponseDataItemContext {
    pub fn builder() -> ListEventsResponseDataItemContextBuilder {
        <ListEventsResponseDataItemContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemContextBuilder {
    ad_campaign_id: Option<String>,
    ad_click_id: Option<String>,
    ad_click_type: Option<String>,
    ad_id: Option<String>,
    ad_set_id: Option<String>,
    source_type: Option<ListEventsResponseDataItemContextSourceType>,
    utm_campaign: Option<String>,
    utm_content: Option<String>,
    utm_medium: Option<String>,
    utm_source: Option<String>,
    utm_term: Option<String>,
}

impl ListEventsResponseDataItemContextBuilder {
    pub fn ad_campaign_id(mut self, value: impl Into<String>) -> Self {
        self.ad_campaign_id = Some(value.into());
        self
    }

    pub fn ad_click_id(mut self, value: impl Into<String>) -> Self {
        self.ad_click_id = Some(value.into());
        self
    }

    pub fn ad_click_type(mut self, value: impl Into<String>) -> Self {
        self.ad_click_type = Some(value.into());
        self
    }

    pub fn ad_id(mut self, value: impl Into<String>) -> Self {
        self.ad_id = Some(value.into());
        self
    }

    pub fn ad_set_id(mut self, value: impl Into<String>) -> Self {
        self.ad_set_id = Some(value.into());
        self
    }

    pub fn source_type(mut self, value: ListEventsResponseDataItemContextSourceType) -> Self {
        self.source_type = Some(value);
        self
    }

    pub fn utm_campaign(mut self, value: impl Into<String>) -> Self {
        self.utm_campaign = Some(value.into());
        self
    }

    pub fn utm_content(mut self, value: impl Into<String>) -> Self {
        self.utm_content = Some(value.into());
        self
    }

    pub fn utm_medium(mut self, value: impl Into<String>) -> Self {
        self.utm_medium = Some(value.into());
        self
    }

    pub fn utm_source(mut self, value: impl Into<String>) -> Self {
        self.utm_source = Some(value.into());
        self
    }

    pub fn utm_term(mut self, value: impl Into<String>) -> Self {
        self.utm_term = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListEventsResponseDataItemContext`].
    pub fn build(self) -> Result<ListEventsResponseDataItemContext, BuildError> {
        Ok(ListEventsResponseDataItemContext {
            ad_campaign_id: self.ad_campaign_id,
            ad_click_id: self.ad_click_id,
            ad_click_type: self.ad_click_type,
            ad_id: self.ad_id,
            ad_set_id: self.ad_set_id,
            source_type: self.source_type,
            utm_campaign: self.utm_campaign,
            utm_content: self.utm_content,
            utm_medium: self.utm_medium,
            utm_source: self.utm_source,
            utm_term: self.utm_term,
        })
    }
}
