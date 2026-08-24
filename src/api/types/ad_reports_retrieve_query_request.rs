pub use crate::prelude::*;

/// Query parameters for retrieve
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AdReportsRetrieveQueryRequest {
    /// Scope the report to these ad campaigns (max 100); stats are summed across them. Mutually exclusive with `companyId`, `adGroupIds`, and `adIds`.
    #[serde(default)]
    pub ad_campaign_ids: Vec<Option<String>>,
    /// Scope the report to these ad groups (max 100); stats are summed across them. Mutually exclusive with `companyId`, `adCampaignIds`, and `adIds`.
    #[serde(default)]
    pub ad_group_ids: Vec<Option<String>>,
    /// Scope the report to these ads (max 100); stats are summed across them. Mutually exclusive with `companyId`, `adCampaignIds`, and `adGroupIds`.
    #[serde(default)]
    pub ad_ids: Vec<Option<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<AdReportBreakdownLevels>,
    /// The unique identifier of a company. Mutually exclusive with `adCampaignIds`, `adGroupIds`, and `adIds`. Use with `breakdown` to fan out across every campaign, ad group, or ad in the company without paging.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_id: Option<String>,
    /// ISO 4217 currency code to report `spend` in. Defaults to the company's ads reporting currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Inclusive start of the reporting window.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub from: DateTime<FixedOffset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub granularity: Option<Granularities>,
    /// Inclusive end of the reporting window.
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset")]
    pub to: DateTime<FixedOffset>,
}

impl AdReportsRetrieveQueryRequest {
    pub fn builder() -> AdReportsRetrieveQueryRequestBuilder {
        <AdReportsRetrieveQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AdReportsRetrieveQueryRequestBuilder {
    ad_campaign_ids: Option<Vec<Option<String>>>,
    ad_group_ids: Option<Vec<Option<String>>>,
    ad_ids: Option<Vec<Option<String>>>,
    breakdown: Option<AdReportBreakdownLevels>,
    company_id: Option<String>,
    currency: Option<String>,
    from: Option<DateTime<FixedOffset>>,
    granularity: Option<Granularities>,
    to: Option<DateTime<FixedOffset>>,
}

impl AdReportsRetrieveQueryRequestBuilder {
    pub fn ad_campaign_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.ad_campaign_ids = Some(value);
        self
    }

    pub fn ad_group_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.ad_group_ids = Some(value);
        self
    }

    pub fn ad_ids(mut self, value: Vec<Option<String>>) -> Self {
        self.ad_ids = Some(value);
        self
    }

    pub fn breakdown(mut self, value: AdReportBreakdownLevels) -> Self {
        self.breakdown = Some(value);
        self
    }

    pub fn company_id(mut self, value: impl Into<String>) -> Self {
        self.company_id = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn from(mut self, value: DateTime<FixedOffset>) -> Self {
        self.from = Some(value);
        self
    }

    pub fn granularity(mut self, value: Granularities) -> Self {
        self.granularity = Some(value);
        self
    }

    pub fn to(mut self, value: DateTime<FixedOffset>) -> Self {
        self.to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AdReportsRetrieveQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ad_campaign_ids`](AdReportsRetrieveQueryRequestBuilder::ad_campaign_ids)
    /// - [`ad_group_ids`](AdReportsRetrieveQueryRequestBuilder::ad_group_ids)
    /// - [`ad_ids`](AdReportsRetrieveQueryRequestBuilder::ad_ids)
    /// - [`from`](AdReportsRetrieveQueryRequestBuilder::from)
    /// - [`to`](AdReportsRetrieveQueryRequestBuilder::to)
    pub fn build(self) -> Result<AdReportsRetrieveQueryRequest, BuildError> {
        Ok(AdReportsRetrieveQueryRequest {
            ad_campaign_ids: self
                .ad_campaign_ids
                .ok_or_else(|| BuildError::missing_field("ad_campaign_ids"))?,
            ad_group_ids: self
                .ad_group_ids
                .ok_or_else(|| BuildError::missing_field("ad_group_ids"))?,
            ad_ids: self
                .ad_ids
                .ok_or_else(|| BuildError::missing_field("ad_ids"))?,
            breakdown: self.breakdown,
            company_id: self.company_id,
            currency: self.currency,
            from: self.from.ok_or_else(|| BuildError::missing_field("from"))?,
            granularity: self.granularity,
            to: self.to.ok_or_else(|| BuildError::missing_field("to"))?,
        })
    }
}
