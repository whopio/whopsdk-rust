pub use crate::prelude::*;

/// Tracking and attribution context.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateEventsRequestContext {
    /// Ad campaign ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_campaign_id: Option<String>,
    /// Ad ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_id: Option<String>,
    /// Ad set ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_set_id: Option<String>,
    /// Facebook click cookie (_fbc, format fb.1.{timestamp}.{fbclid}).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbc: Option<String>,
    /// Facebook click ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbclid: Option<String>,
    /// Facebook browser pixel ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fbp: Option<String>,
    /// Client-side device fingerprint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// Confidence score (0-1) for the device fingerprint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint_confidence: Option<f64>,
    /// Google Analytics client ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ga: Option<String>,
    /// Google Ads gbraid click ID (iOS privacy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbraid: Option<String>,
    /// Google click ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gclid: Option<String>,
    /// Instagram session ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ig_sid: Option<String>,
    /// IP address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// Browser language (e.g. en-US).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// LinkedIn click ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub li_fat_id: Option<String>,
    /// Microsoft Advertising (Bing) click ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msclkid: Option<String>,
    /// Reddit click ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdt_cid: Option<String>,
    /// Whop SC identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sc: Option<String>,
    /// Snapchat click ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sccid: Option<String>,
    /// Screen resolution (e.g. 1920x1080).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub screen_resolution: Option<String>,
    /// IANA timezone (e.g. America/New_York).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// TikTok click ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttclid: Option<String>,
    /// TikTok pixel ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttp: Option<String>,
    /// X (Twitter) click ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twclid: Option<String>,
    /// Browser user agent string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// UTM campaign parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_campaign: Option<String>,
    /// UTM content parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_content: Option<String>,
    /// UTM ID parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_id: Option<String>,
    /// UTM medium parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_medium: Option<String>,
    /// UTM source parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_source: Option<String>,
    /// UTM term parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub utm_term: Option<String>,
    /// Google Ads wbraid click ID (iOS privacy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wbraid: Option<String>,
}

impl CreateEventsRequestContext {
    pub fn builder() -> CreateEventsRequestContextBuilder {
        <CreateEventsRequestContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateEventsRequestContextBuilder {
    ad_campaign_id: Option<String>,
    ad_id: Option<String>,
    ad_set_id: Option<String>,
    fbc: Option<String>,
    fbclid: Option<String>,
    fbp: Option<String>,
    fingerprint: Option<String>,
    fingerprint_confidence: Option<f64>,
    ga: Option<String>,
    gbraid: Option<String>,
    gclid: Option<String>,
    ig_sid: Option<String>,
    ip_address: Option<String>,
    language: Option<String>,
    li_fat_id: Option<String>,
    msclkid: Option<String>,
    rdt_cid: Option<String>,
    sc: Option<String>,
    sccid: Option<String>,
    screen_resolution: Option<String>,
    timezone: Option<String>,
    ttclid: Option<String>,
    ttp: Option<String>,
    twclid: Option<String>,
    user_agent: Option<String>,
    utm_campaign: Option<String>,
    utm_content: Option<String>,
    utm_id: Option<String>,
    utm_medium: Option<String>,
    utm_source: Option<String>,
    utm_term: Option<String>,
    wbraid: Option<String>,
}

impl CreateEventsRequestContextBuilder {
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

    pub fn fbc(mut self, value: impl Into<String>) -> Self {
        self.fbc = Some(value.into());
        self
    }

    pub fn fbclid(mut self, value: impl Into<String>) -> Self {
        self.fbclid = Some(value.into());
        self
    }

    pub fn fbp(mut self, value: impl Into<String>) -> Self {
        self.fbp = Some(value.into());
        self
    }

    pub fn fingerprint(mut self, value: impl Into<String>) -> Self {
        self.fingerprint = Some(value.into());
        self
    }

    pub fn fingerprint_confidence(mut self, value: f64) -> Self {
        self.fingerprint_confidence = Some(value);
        self
    }

    pub fn ga(mut self, value: impl Into<String>) -> Self {
        self.ga = Some(value.into());
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

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
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

    pub fn sc(mut self, value: impl Into<String>) -> Self {
        self.sc = Some(value.into());
        self
    }

    pub fn sccid(mut self, value: impl Into<String>) -> Self {
        self.sccid = Some(value.into());
        self
    }

    pub fn screen_resolution(mut self, value: impl Into<String>) -> Self {
        self.screen_resolution = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn ttclid(mut self, value: impl Into<String>) -> Self {
        self.ttclid = Some(value.into());
        self
    }

    pub fn ttp(mut self, value: impl Into<String>) -> Self {
        self.ttp = Some(value.into());
        self
    }

    pub fn twclid(mut self, value: impl Into<String>) -> Self {
        self.twclid = Some(value.into());
        self
    }

    pub fn user_agent(mut self, value: impl Into<String>) -> Self {
        self.user_agent = Some(value.into());
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

    /// Consumes the builder and constructs a [`CreateEventsRequestContext`].
    pub fn build(self) -> Result<CreateEventsRequestContext, BuildError> {
        Ok(CreateEventsRequestContext {
            ad_campaign_id: self.ad_campaign_id,
            ad_id: self.ad_id,
            ad_set_id: self.ad_set_id,
            fbc: self.fbc,
            fbclid: self.fbclid,
            fbp: self.fbp,
            fingerprint: self.fingerprint,
            fingerprint_confidence: self.fingerprint_confidence,
            ga: self.ga,
            gbraid: self.gbraid,
            gclid: self.gclid,
            ig_sid: self.ig_sid,
            ip_address: self.ip_address,
            language: self.language,
            li_fat_id: self.li_fat_id,
            msclkid: self.msclkid,
            rdt_cid: self.rdt_cid,
            sc: self.sc,
            sccid: self.sccid,
            screen_resolution: self.screen_resolution,
            timezone: self.timezone,
            ttclid: self.ttclid,
            ttp: self.ttp,
            twclid: self.twclid,
            user_agent: self.user_agent,
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
