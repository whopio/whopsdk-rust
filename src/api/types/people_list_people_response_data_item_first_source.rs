pub use crate::prelude::*;

/// Where a visit came from: a whop ad click, a lead form, an external ad, or a referring site.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ListPeopleResponseDataItemFirstSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad: Option<ListPeopleResponseDataItemFirstSourceAd>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_group: Option<ListPeopleResponseDataItemFirstSourceAdGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<ListPeopleResponseDataItemFirstSourceCampaign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurred_at: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform: Option<String>,
    pub r#type: ListPeopleResponseDataItemFirstSourceType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_source: Option<String>,
}

impl ListPeopleResponseDataItemFirstSource {
    pub fn builder() -> ListPeopleResponseDataItemFirstSourceBuilder {
        <ListPeopleResponseDataItemFirstSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPeopleResponseDataItemFirstSourceBuilder {
    ad: Option<ListPeopleResponseDataItemFirstSourceAd>,
    ad_group: Option<ListPeopleResponseDataItemFirstSourceAdGroup>,
    campaign: Option<ListPeopleResponseDataItemFirstSourceCampaign>,
    domain: Option<String>,
    occurred_at: Option<DateTime<FixedOffset>>,
    platform: Option<String>,
    r#type: Option<ListPeopleResponseDataItemFirstSourceType>,
    utm_source: Option<String>,
}

impl ListPeopleResponseDataItemFirstSourceBuilder {
    pub fn ad(mut self, value: ListPeopleResponseDataItemFirstSourceAd) -> Self {
        self.ad = Some(value);
        self
    }

    pub fn ad_group(mut self, value: ListPeopleResponseDataItemFirstSourceAdGroup) -> Self {
        self.ad_group = Some(value);
        self
    }

    pub fn campaign(mut self, value: ListPeopleResponseDataItemFirstSourceCampaign) -> Self {
        self.campaign = Some(value);
        self
    }

    pub fn domain(mut self, value: impl Into<String>) -> Self {
        self.domain = Some(value.into());
        self
    }

    pub fn occurred_at(mut self, value: DateTime<FixedOffset>) -> Self {
        self.occurred_at = Some(value);
        self
    }

    pub fn platform(mut self, value: impl Into<String>) -> Self {
        self.platform = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: ListPeopleResponseDataItemFirstSourceType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn utm_source(mut self, value: impl Into<String>) -> Self {
        self.utm_source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPeopleResponseDataItemFirstSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](ListPeopleResponseDataItemFirstSourceBuilder::r#type)
    pub fn build(self) -> Result<ListPeopleResponseDataItemFirstSource, BuildError> {
        Ok(ListPeopleResponseDataItemFirstSource {
            ad: self.ad,
            ad_group: self.ad_group,
            campaign: self.campaign,
            domain: self.domain,
            occurred_at: self.occurred_at,
            platform: self.platform,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            utm_source: self.utm_source,
        })
    }
}
