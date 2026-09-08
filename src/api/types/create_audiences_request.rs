pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateAudiencesRequest {
    /// Account ID, prefixed `biz_`.
    #[serde(default)]
    pub account_id: String,
    /// Audience type. Defaults to `custom`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience_type: Option<CreateAudiencesRequestAudienceType>,
    /// Filter audiences only, and set only at creation. `true` (the default) rebuilds membership from the filters twice a day. `false` keeps whoever matched at creation and never rebuilds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_refresh: Option<bool>,
    /// CSV audiences only. Maps supported identity fields to CSV column headers. Map at least one of `email` or `phone`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_mapping: Option<CreateAudiencesRequestColumnMapping>,
    /// Lookalikes only. Number of lookalike audiences to create (1–6).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Rules for membership based on social engagement. Requires a connected social account with advertising access.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub engagement: Option<CreateAudiencesRequestEngagement>,
    /// CSV audiences only. The uploaded customer CSV — a file id (`file_...`) returned by `POST /files`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    /// Filter audiences only. The People filters that define membership, keyed exactly as `GET /people` accepts them — for example `{"os": "iOS", "country": "US"}`. Date filters must be rolling windows — `first_seen_within_days` or `last_seen_within_days` — so the audience re-anchors on every refresh; fixed dates such as `first_seen_after` are rejected. Source values are canonical source paths (`whop:<campaign>:<group>:<ad>`, `ext:<platform>:...`, `referrer:<domain>`, `direct`), exact or with a trailing `:*` wildcard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, serde_json::Value>>,
    /// Audience display name. Required for custom audiences; lookalike names are generated from the source audience.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Lookalikes only. Total similarity reach as a whole percent (1–20), sliced evenly across `count` — must be divisible by `count`. For example, 3 audiences at 6% creates 0–2%, 2–4%, and 4–6% bands.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i64>,
    /// Lookalikes only. The ready custom audience (`adaud_`) to build from; uploaded and People audiences need at least 100 matched people. Meta validates engagement audience eligibility when creating the lookalike.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_audience_id: Option<String>,
    /// Custom audience source. Inferred from `engagement`, then `filters`, otherwise defaults to `csv_upload`. Supply only the fields for the selected source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<CreateAudiencesRequestSourceType>,
}

impl CreateAudiencesRequest {
    pub fn builder() -> CreateAudiencesRequestBuilder {
        <CreateAudiencesRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateAudiencesRequestBuilder {
    account_id: Option<String>,
    audience_type: Option<CreateAudiencesRequestAudienceType>,
    auto_refresh: Option<bool>,
    column_mapping: Option<CreateAudiencesRequestColumnMapping>,
    count: Option<i64>,
    engagement: Option<CreateAudiencesRequestEngagement>,
    file_id: Option<String>,
    filters: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
    percentage: Option<i64>,
    source_audience_id: Option<String>,
    source_type: Option<CreateAudiencesRequestSourceType>,
}

impl CreateAudiencesRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn audience_type(mut self, value: CreateAudiencesRequestAudienceType) -> Self {
        self.audience_type = Some(value);
        self
    }

    pub fn auto_refresh(mut self, value: bool) -> Self {
        self.auto_refresh = Some(value);
        self
    }

    pub fn column_mapping(mut self, value: CreateAudiencesRequestColumnMapping) -> Self {
        self.column_mapping = Some(value);
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn engagement(mut self, value: CreateAudiencesRequestEngagement) -> Self {
        self.engagement = Some(value);
        self
    }

    pub fn file_id(mut self, value: impl Into<String>) -> Self {
        self.file_id = Some(value.into());
        self
    }

    pub fn filters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn percentage(mut self, value: i64) -> Self {
        self.percentage = Some(value);
        self
    }

    pub fn source_audience_id(mut self, value: impl Into<String>) -> Self {
        self.source_audience_id = Some(value.into());
        self
    }

    pub fn source_type(mut self, value: CreateAudiencesRequestSourceType) -> Self {
        self.source_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateAudiencesRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](CreateAudiencesRequestBuilder::account_id)
    pub fn build(self) -> Result<CreateAudiencesRequest, BuildError> {
        Ok(CreateAudiencesRequest {
            account_id: self
                .account_id
                .ok_or_else(|| BuildError::missing_field("account_id"))?,
            audience_type: self.audience_type,
            auto_refresh: self.auto_refresh,
            column_mapping: self.column_mapping,
            count: self.count,
            engagement: self.engagement,
            file_id: self.file_id,
            filters: self.filters,
            name: self.name,
            percentage: self.percentage,
            source_audience_id: self.source_audience_id,
            source_type: self.source_type,
        })
    }
}
