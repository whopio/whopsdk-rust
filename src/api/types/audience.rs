pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Audience {
    /// `custom` = a customer list (uploaded, or built from saved People filters); `lookalike` = Meta lookalike built from a custom audience.
    pub audience_type: AudienceAudienceType,
    /// Whether membership keeps updating. `true` rebuilds it from the saved filters twice a day, so people join and leave as they start and stop matching. `false` keeps whoever matched when it was built and never rebuilds. Always `false` for uploaded lists and lookalikes.
    #[serde(default)]
    pub auto_refresh: bool,
    /// When the audience was created, as an ISO 8601 timestamp.
    #[serde(default)]
    pub created_at: String,
    /// Processing error message. `null` unless processing is partial or failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// For audiences built from People filters: the filters that define membership, keyed exactly as `GET /people` accepts them — for example `{"os": "iOS", "country": "US"}`. `null` for uploaded lists and lookalikes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<HashMap<String, serde_json::Value>>,
    /// Audience ID, prefixed `adaud_`.
    #[serde(default)]
    pub id: String,
    /// When the audience membership was last rebuilt, as an ISO 8601 timestamp. `null` until the first build completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_refreshed_at: Option<String>,
    /// For lookalikes: the upper bound of the similarity band as a fraction (0.02 = top 2%). `null` for custom audiences.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lookalike_ratio: Option<f64>,
    /// For lookalikes: the lower bound of the similarity band as a fraction. `null` for custom audiences and first-tier lookalikes.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub lookalike_starting_ratio: Option<f64>,
    #[serde(default)]
    pub match_rates: Vec<AudienceMatchRate>,
    /// Members successfully uploaded to connected ad accounts. Always 0 for lookalikes.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub matched_rows: f64,
    /// Audience display name.
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub platform_audience_ids: Vec<String>,
    /// Members processed from the source so far. Always 0 for lookalikes.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub processed_rows: f64,
    /// Processing progress from 0 to 100.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub progress_percent: f64,
    /// For lookalikes: the audience this lookalike was built from. `null` for custom audiences.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_audience_id: Option<String>,
    /// Where members come from. `csv_upload` = an uploaded customer list; `people_filter` = built from saved People filters. See `auto_refresh` for whether a `people_filter` audience keeps updating.
    pub source_type: AudienceSourceType,
    /// Current state of the audience import. `syncing` means Whop is sending matched rows to connected ad accounts. When status is `partial` or `failed`, `error_message` explains what went wrong.
    pub status: AudienceStatus,
    /// Total members detected in the source — CSV rows for uploaded lists, matching people for automatic audiences. Always 0 for lookalikes.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_rows: f64,
    /// When the audience was last updated, as an ISO 8601 timestamp.
    #[serde(default)]
    pub updated_at: String,
}

impl Audience {
    pub fn builder() -> AudienceBuilder {
        <AudienceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AudienceBuilder {
    audience_type: Option<AudienceAudienceType>,
    auto_refresh: Option<bool>,
    created_at: Option<String>,
    error_message: Option<String>,
    filters: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    last_refreshed_at: Option<String>,
    lookalike_ratio: Option<f64>,
    lookalike_starting_ratio: Option<f64>,
    match_rates: Option<Vec<AudienceMatchRate>>,
    matched_rows: Option<f64>,
    name: Option<String>,
    platform_audience_ids: Option<Vec<String>>,
    processed_rows: Option<f64>,
    progress_percent: Option<f64>,
    source_audience_id: Option<String>,
    source_type: Option<AudienceSourceType>,
    status: Option<AudienceStatus>,
    total_rows: Option<f64>,
    updated_at: Option<String>,
}

impl AudienceBuilder {
    pub fn audience_type(mut self, value: AudienceAudienceType) -> Self {
        self.audience_type = Some(value);
        self
    }

    pub fn auto_refresh(mut self, value: bool) -> Self {
        self.auto_refresh = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn error_message(mut self, value: impl Into<String>) -> Self {
        self.error_message = Some(value.into());
        self
    }

    pub fn filters(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.filters = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn last_refreshed_at(mut self, value: impl Into<String>) -> Self {
        self.last_refreshed_at = Some(value.into());
        self
    }

    pub fn lookalike_ratio(mut self, value: f64) -> Self {
        self.lookalike_ratio = Some(value);
        self
    }

    pub fn lookalike_starting_ratio(mut self, value: f64) -> Self {
        self.lookalike_starting_ratio = Some(value);
        self
    }

    pub fn match_rates(mut self, value: Vec<AudienceMatchRate>) -> Self {
        self.match_rates = Some(value);
        self
    }

    pub fn matched_rows(mut self, value: f64) -> Self {
        self.matched_rows = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn platform_audience_ids(mut self, value: Vec<String>) -> Self {
        self.platform_audience_ids = Some(value);
        self
    }

    pub fn processed_rows(mut self, value: f64) -> Self {
        self.processed_rows = Some(value);
        self
    }

    pub fn progress_percent(mut self, value: f64) -> Self {
        self.progress_percent = Some(value);
        self
    }

    pub fn source_audience_id(mut self, value: impl Into<String>) -> Self {
        self.source_audience_id = Some(value.into());
        self
    }

    pub fn source_type(mut self, value: AudienceSourceType) -> Self {
        self.source_type = Some(value);
        self
    }

    pub fn status(mut self, value: AudienceStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn total_rows(mut self, value: f64) -> Self {
        self.total_rows = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Audience`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audience_type`](AudienceBuilder::audience_type)
    /// - [`auto_refresh`](AudienceBuilder::auto_refresh)
    /// - [`created_at`](AudienceBuilder::created_at)
    /// - [`id`](AudienceBuilder::id)
    /// - [`match_rates`](AudienceBuilder::match_rates)
    /// - [`matched_rows`](AudienceBuilder::matched_rows)
    /// - [`name`](AudienceBuilder::name)
    /// - [`platform_audience_ids`](AudienceBuilder::platform_audience_ids)
    /// - [`processed_rows`](AudienceBuilder::processed_rows)
    /// - [`progress_percent`](AudienceBuilder::progress_percent)
    /// - [`source_type`](AudienceBuilder::source_type)
    /// - [`status`](AudienceBuilder::status)
    /// - [`total_rows`](AudienceBuilder::total_rows)
    /// - [`updated_at`](AudienceBuilder::updated_at)
    pub fn build(self) -> Result<Audience, BuildError> {
        Ok(Audience {
            audience_type: self
                .audience_type
                .ok_or_else(|| BuildError::missing_field("audience_type"))?,
            auto_refresh: self
                .auto_refresh
                .ok_or_else(|| BuildError::missing_field("auto_refresh"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            error_message: self.error_message,
            filters: self.filters,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            last_refreshed_at: self.last_refreshed_at,
            lookalike_ratio: self.lookalike_ratio,
            lookalike_starting_ratio: self.lookalike_starting_ratio,
            match_rates: self
                .match_rates
                .ok_or_else(|| BuildError::missing_field("match_rates"))?,
            matched_rows: self
                .matched_rows
                .ok_or_else(|| BuildError::missing_field("matched_rows"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            platform_audience_ids: self
                .platform_audience_ids
                .ok_or_else(|| BuildError::missing_field("platform_audience_ids"))?,
            processed_rows: self
                .processed_rows
                .ok_or_else(|| BuildError::missing_field("processed_rows"))?,
            progress_percent: self
                .progress_percent
                .ok_or_else(|| BuildError::missing_field("progress_percent"))?,
            source_audience_id: self.source_audience_id,
            source_type: self
                .source_type
                .ok_or_else(|| BuildError::missing_field("source_type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            total_rows: self
                .total_rows
                .ok_or_else(|| BuildError::missing_field("total_rows"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
