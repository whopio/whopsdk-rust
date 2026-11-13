pub use crate::prelude::*;

/// UTM parameters, ad click ids and ad ids, as the page reported them. Keys the event did not carry are left out.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PostEventsCreatedPayloadDataEventsItemContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_campaign_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_set_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbclid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbraid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gclid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ig_sid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub li_fat_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msclkid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdt_cid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sccid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttclid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twclid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_campaign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_medium: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_term: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wbraid: Option<String>,
}

impl PostEventsCreatedPayloadDataEventsItemContext {
    pub fn builder() -> PostEventsCreatedPayloadDataEventsItemContextBuilder {
        <PostEventsCreatedPayloadDataEventsItemContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PostEventsCreatedPayloadDataEventsItemContextBuilder {
    ad_campaign_id: Option<String>,
    ad_id: Option<String>,
    ad_set_id: Option<String>,
    fbclid: Option<String>,
    gbraid: Option<String>,
    gclid: Option<String>,
    ig_sid: Option<String>,
    li_fat_id: Option<String>,
    msclkid: Option<String>,
    rdt_cid: Option<String>,
    sccid: Option<String>,
    ttclid: Option<String>,
    twclid: Option<String>,
    utm_campaign: Option<String>,
    utm_content: Option<String>,
    utm_id: Option<String>,
    utm_medium: Option<String>,
    utm_source: Option<String>,
    utm_term: Option<String>,
    wbraid: Option<String>,
}

impl PostEventsCreatedPayloadDataEventsItemContextBuilder {
    pub fn ad_campaign_id(mut self, value: impl Into<String>) -> Self {
        self.ad_campaign_id = Some(value.into());
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

    pub fn fbclid(mut self, value: impl Into<String>) -> Self {
        self.fbclid = Some(value.into());
        self
    }

    pub fn gbraid(mut self, value: impl Into<String>) -> Self {
        self.gbraid = Some(value.into());
        self
    }

    pub fn gclid(mut self, value: impl Into<String>) -> Self {
        self.gclid = Some(value.into());
        self
    }

    pub fn ig_sid(mut self, value: impl Into<String>) -> Self {
        self.ig_sid = Some(value.into());
        self
    }

    pub fn li_fat_id(mut self, value: impl Into<String>) -> Self {
        self.li_fat_id = Some(value.into());
        self
    }

    pub fn msclkid(mut self, value: impl Into<String>) -> Self {
        self.msclkid = Some(value.into());
        self
    }

    pub fn rdt_cid(mut self, value: impl Into<String>) -> Self {
        self.rdt_cid = Some(value.into());
        self
    }

    pub fn sccid(mut self, value: impl Into<String>) -> Self {
        self.sccid = Some(value.into());
        self
    }

    pub fn ttclid(mut self, value: impl Into<String>) -> Self {
        self.ttclid = Some(value.into());
        self
    }

    pub fn twclid(mut self, value: impl Into<String>) -> Self {
        self.twclid = Some(value.into());
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

    pub fn utm_id(mut self, value: impl Into<String>) -> Self {
        self.utm_id = Some(value.into());
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

    pub fn wbraid(mut self, value: impl Into<String>) -> Self {
        self.wbraid = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PostEventsCreatedPayloadDataEventsItemContext`].
    pub fn build(self) -> Result<PostEventsCreatedPayloadDataEventsItemContext, BuildError> {
        Ok(PostEventsCreatedPayloadDataEventsItemContext {
            ad_campaign_id: self.ad_campaign_id,
            ad_id: self.ad_id,
            ad_set_id: self.ad_set_id,
            fbclid: self.fbclid,
            gbraid: self.gbraid,
            gclid: self.gclid,
            ig_sid: self.ig_sid,
            li_fat_id: self.li_fat_id,
            msclkid: self.msclkid,
            rdt_cid: self.rdt_cid,
            sccid: self.sccid,
            ttclid: self.ttclid,
            twclid: self.twclid,
            utm_campaign: self.utm_campaign,
            utm_content: self.utm_content,
            utm_id: self.utm_id,
            utm_medium: self.utm_medium,
            utm_source: self.utm_source,
            utm_term: self.utm_term,
            wbraid: self.wbraid,
        })
    }
}
