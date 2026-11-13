pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrieveStatsRequestSteps {
    #[serde(rename = "1")]
    #[serde(default)]
    pub _1: FunnelStep,
    #[serde(rename = "2")]
    #[serde(default)]
    pub _2: FunnelStep,
    #[serde(rename = "3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _3: Option<FunnelStep>,
    #[serde(rename = "4")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _4: Option<FunnelStep>,
    #[serde(rename = "5")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _5: Option<FunnelStep>,
    #[serde(rename = "6")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _6: Option<FunnelStep>,
    #[serde(rename = "7")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _7: Option<FunnelStep>,
    #[serde(rename = "8")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _8: Option<FunnelStep>,
    #[serde(rename = "9")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _9: Option<FunnelStep>,
    #[serde(rename = "10")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _10: Option<FunnelStep>,
}

impl RetrieveStatsRequestSteps {
    pub fn builder() -> RetrieveStatsRequestStepsBuilder {
        <RetrieveStatsRequestStepsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveStatsRequestStepsBuilder {
    _1: Option<FunnelStep>,
    _2: Option<FunnelStep>,
    _3: Option<FunnelStep>,
    _4: Option<FunnelStep>,
    _5: Option<FunnelStep>,
    _6: Option<FunnelStep>,
    _7: Option<FunnelStep>,
    _8: Option<FunnelStep>,
    _9: Option<FunnelStep>,
    _10: Option<FunnelStep>,
}

impl RetrieveStatsRequestStepsBuilder {
    pub fn _1(mut self, value: FunnelStep) -> Self {
        self._1 = Some(value);
        self
    }

    pub fn _2(mut self, value: FunnelStep) -> Self {
        self._2 = Some(value);
        self
    }

    pub fn _3(mut self, value: FunnelStep) -> Self {
        self._3 = Some(value);
        self
    }

    pub fn _4(mut self, value: FunnelStep) -> Self {
        self._4 = Some(value);
        self
    }

    pub fn _5(mut self, value: FunnelStep) -> Self {
        self._5 = Some(value);
        self
    }

    pub fn _6(mut self, value: FunnelStep) -> Self {
        self._6 = Some(value);
        self
    }

    pub fn _7(mut self, value: FunnelStep) -> Self {
        self._7 = Some(value);
        self
    }

    pub fn _8(mut self, value: FunnelStep) -> Self {
        self._8 = Some(value);
        self
    }

    pub fn _9(mut self, value: FunnelStep) -> Self {
        self._9 = Some(value);
        self
    }

    pub fn _10(mut self, value: FunnelStep) -> Self {
        self._10 = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveStatsRequestSteps`].
    /// This method will fail if any of the following fields are not set:
    /// - [`_1`](RetrieveStatsRequestStepsBuilder::_1)
    /// - [`_2`](RetrieveStatsRequestStepsBuilder::_2)
    pub fn build(self) -> Result<RetrieveStatsRequestSteps, BuildError> {
        Ok(RetrieveStatsRequestSteps {
            _1: self._1.ok_or_else(|| BuildError::missing_field("_1"))?,
            _2: self._2.ok_or_else(|| BuildError::missing_field("_2"))?,
            _3: self._3,
            _4: self._4,
            _5: self._5,
            _6: self._6,
            _7: self._7,
            _8: self._8,
            _9: self._9,
            _10: self._10,
        })
    }
}
