pub use crate::prelude::*;

/// Hydrated details for the records this event references. Only present keys resolved.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListEventsResponseDataItemRelated {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<ListEventsResponseDataItemRelatedAccount>,
    /// The Whop ad this event's click resolved to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad: Option<ListEventsResponseDataItemRelatedAd>,
    /// The Whop ad campaign this event's click resolved to, read from the ad entity tree rather than the click's url params.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_campaign: Option<ListEventsResponseDataItemRelatedAdCampaign>,
    /// The Whop ad group this event's click resolved to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_group: Option<ListEventsResponseDataItemRelatedAdGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app: Option<ListEventsResponseDataItemRelatedApp>,
    /// The saved audience this event came from. Present on the identify events an audience ingest writes for each of its members.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<ListEventsResponseDataItemRelatedAudience>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<ListEventsResponseDataItemRelatedPayment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<ListEventsResponseDataItemRelatedPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<ListEventsResponseDataItemRelatedProduct>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<ListEventsResponseDataItemRelatedUser>,
}

impl ListEventsResponseDataItemRelated {
    pub fn builder() -> ListEventsResponseDataItemRelatedBuilder {
        <ListEventsResponseDataItemRelatedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEventsResponseDataItemRelatedBuilder {
    account: Option<ListEventsResponseDataItemRelatedAccount>,
    ad: Option<ListEventsResponseDataItemRelatedAd>,
    ad_campaign: Option<ListEventsResponseDataItemRelatedAdCampaign>,
    ad_group: Option<ListEventsResponseDataItemRelatedAdGroup>,
    app: Option<ListEventsResponseDataItemRelatedApp>,
    audience: Option<ListEventsResponseDataItemRelatedAudience>,
    payment: Option<ListEventsResponseDataItemRelatedPayment>,
    plan: Option<ListEventsResponseDataItemRelatedPlan>,
    product: Option<ListEventsResponseDataItemRelatedProduct>,
    user: Option<ListEventsResponseDataItemRelatedUser>,
}

impl ListEventsResponseDataItemRelatedBuilder {
    pub fn account(mut self, value: ListEventsResponseDataItemRelatedAccount) -> Self {
        self.account = Some(value);
        self
    }

    pub fn ad(mut self, value: ListEventsResponseDataItemRelatedAd) -> Self {
        self.ad = Some(value);
        self
    }

    pub fn ad_campaign(mut self, value: ListEventsResponseDataItemRelatedAdCampaign) -> Self {
        self.ad_campaign = Some(value);
        self
    }

    pub fn ad_group(mut self, value: ListEventsResponseDataItemRelatedAdGroup) -> Self {
        self.ad_group = Some(value);
        self
    }

    pub fn app(mut self, value: ListEventsResponseDataItemRelatedApp) -> Self {
        self.app = Some(value);
        self
    }

    pub fn audience(mut self, value: ListEventsResponseDataItemRelatedAudience) -> Self {
        self.audience = Some(value);
        self
    }

    pub fn payment(mut self, value: ListEventsResponseDataItemRelatedPayment) -> Self {
        self.payment = Some(value);
        self
    }

    pub fn plan(mut self, value: ListEventsResponseDataItemRelatedPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn product(mut self, value: ListEventsResponseDataItemRelatedProduct) -> Self {
        self.product = Some(value);
        self
    }

    pub fn user(mut self, value: ListEventsResponseDataItemRelatedUser) -> Self {
        self.user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEventsResponseDataItemRelated`].
    pub fn build(self) -> Result<ListEventsResponseDataItemRelated, BuildError> {
        Ok(ListEventsResponseDataItemRelated {
            account: self.account,
            ad: self.ad,
            ad_campaign: self.ad_campaign,
            ad_group: self.ad_group,
            app: self.app,
            audience: self.audience,
            payment: self.payment,
            plan: self.plan,
            product: self.product,
            user: self.user,
        })
    }
}
