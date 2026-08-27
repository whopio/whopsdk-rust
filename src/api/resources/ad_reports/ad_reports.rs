use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct AdReportsClient {
    pub http_client: HttpClient,
}

impl AdReportsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Performance report for a company, ad campaigns, ad groups, or ads. Always returns aggregate `summary` totals summed across the scope. Set `granularity` to additionally get a time series, or set `breakdown` (`campaign`/`ad_group`/`ad`) to additionally get per-entity rows inside the requested scope. Exactly one of `companyId`, `adCampaignIds`, `adGroupIds`, or `adIds` must be provided.
    ///
    /// Required permissions:
    /// - `ad_campaign:stats:read`
    ///
    /// # Arguments
    ///
    /// * `ad_campaign_ids` - Scope the report to these ad campaigns (max 100); stats are summed across them. Mutually exclusive with `companyId`, `adGroupIds`, and `adIds`.
    /// * `ad_group_ids` - Scope the report to these ad groups (max 100); stats are summed across them. Mutually exclusive with `companyId`, `adCampaignIds`, and `adIds`.
    /// * `ad_ids` - Scope the report to these ads (max 100); stats are summed across them. Mutually exclusive with `companyId`, `adCampaignIds`, and `adGroupIds`.
    /// * `company_id` - The unique identifier of a company. Mutually exclusive with `adCampaignIds`, `adGroupIds`, and `adIds`. Use with `breakdown` to fan out across every campaign, ad group, or ad in the company without paging.
    /// * `currency` - ISO 4217 currency code to report `spend` in. Defaults to the company's ads reporting currency.
    /// * `from` - Inclusive start of the reporting window.
    /// * `to` - Inclusive end of the reporting window.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use whop_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = Whop::new(config).expect("Failed to build client");
    ///     client
    ///         .ad_reports
    ///         .retrieve(
    ///             &AdReportsRetrieveQueryRequest {
    ///                 company_id: Some("biz_xxxxxxxxxxxxxx".to_string()),
    ///                 from: DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap(),
    ///                 to: DateTime::parse_from_rfc3339("2023-12-01T05:00:00Z").unwrap(),
    ///                 ad_campaign_ids: vec![],
    ///                 ad_group_ids: vec![],
    ///                 ad_ids: vec![],
    ///                 breakdown: None,
    ///                 currency: None,
    ///                 granularity: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retrieve(
        &self,
        request: &AdReportsRetrieveQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<AdReport, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.additional_headers
                .entry("Api-Version-Date".to_string())
                .or_insert_with(|| "2026-08-25-2".to_string());
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                "ad_reports",
                None,
                QueryBuilder::new()
                    .string_array("ad_campaign_ids", request.ad_campaign_ids.clone())
                    .string_array("ad_group_ids", request.ad_group_ids.clone())
                    .string_array("ad_ids", request.ad_ids.clone())
                    .serialize("breakdown", request.breakdown.clone())
                    .string("company_id", request.company_id.clone())
                    .string("currency", request.currency.clone())
                    .datetime("from", request.from.clone())
                    .serialize("granularity", request.granularity.clone())
                    .datetime("to", request.to.clone())
                    .build(),
                options,
            )
            .await
    }
}
