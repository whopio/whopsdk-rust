pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppDeployment {
    /// The app being deployed, prefixed `app_`.
    #[serde(default)]
    pub app_id: String,
    /// The build the deployment produced, prefixed `abld_`, or `null` until it succeeds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub build_id: Option<String>,
    /// Whether the running or last deployment uploaded a build without making it live.
    #[serde(default)]
    pub draft: bool,
    /// Why the deployment failed, or `null` when it did not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// How long this deployment is expected to take in total, estimated from previous runs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_duration_ms: Option<i64>,
    /// How much longer the deployment is expected to take. Held above zero until it actually finishes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_remaining_ms: Option<i64>,
    /// When the deployment ended, in milliseconds since the epoch, or `null` while it is still running.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<i64>,
    /// The stage a running deployment has reached, or `null` when none is running. Later phases dominate the wall clock: `process_archive` waits on the upload pipeline and `promote` waits for the build to go live.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<AppDeploymentPhase>,
    /// Fraction of the deployment estimated to be complete, from 0 to 1. Stops just short of 1 until the run ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub progress: Option<f64>,
    /// When the deployment began, in milliseconds since the epoch, or `null` when none has run.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub started_at: Option<i64>,
    /// Whether the app has anything to publish, and what a publish in flight is doing. `unpublished` means publishing would ship something new; `no_source` means the sandbox holds no copy of this app, so there is nothing to publish from.
    pub status: AppDeploymentStatus,
    /// Where the deployed site is served, or `null` unless the deployment went live.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl AppDeployment {
    pub fn builder() -> AppDeploymentBuilder {
        <AppDeploymentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppDeploymentBuilder {
    app_id: Option<String>,
    build_id: Option<String>,
    draft: Option<bool>,
    error: Option<String>,
    estimated_duration_ms: Option<i64>,
    estimated_remaining_ms: Option<i64>,
    finished_at: Option<i64>,
    phase: Option<AppDeploymentPhase>,
    progress: Option<f64>,
    started_at: Option<i64>,
    status: Option<AppDeploymentStatus>,
    url: Option<String>,
}

impl AppDeploymentBuilder {
    pub fn app_id(mut self, value: impl Into<String>) -> Self {
        self.app_id = Some(value.into());
        self
    }

    pub fn build_id(mut self, value: impl Into<String>) -> Self {
        self.build_id = Some(value.into());
        self
    }

    pub fn draft(mut self, value: bool) -> Self {
        self.draft = Some(value);
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn estimated_duration_ms(mut self, value: i64) -> Self {
        self.estimated_duration_ms = Some(value);
        self
    }

    pub fn estimated_remaining_ms(mut self, value: i64) -> Self {
        self.estimated_remaining_ms = Some(value);
        self
    }

    pub fn finished_at(mut self, value: i64) -> Self {
        self.finished_at = Some(value);
        self
    }

    pub fn phase(mut self, value: AppDeploymentPhase) -> Self {
        self.phase = Some(value);
        self
    }

    pub fn progress(mut self, value: f64) -> Self {
        self.progress = Some(value);
        self
    }

    pub fn started_at(mut self, value: i64) -> Self {
        self.started_at = Some(value);
        self
    }

    pub fn status(mut self, value: AppDeploymentStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AppDeployment`].
    /// This method will fail if any of the following fields are not set:
    /// - [`app_id`](AppDeploymentBuilder::app_id)
    /// - [`draft`](AppDeploymentBuilder::draft)
    /// - [`status`](AppDeploymentBuilder::status)
    pub fn build(self) -> Result<AppDeployment, BuildError> {
        Ok(AppDeployment {
            app_id: self
                .app_id
                .ok_or_else(|| BuildError::missing_field("app_id"))?,
            build_id: self.build_id,
            draft: self
                .draft
                .ok_or_else(|| BuildError::missing_field("draft"))?,
            error: self.error,
            estimated_duration_ms: self.estimated_duration_ms,
            estimated_remaining_ms: self.estimated_remaining_ms,
            finished_at: self.finished_at,
            phase: self.phase,
            progress: self.progress,
            started_at: self.started_at,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            url: self.url,
        })
    }
}
