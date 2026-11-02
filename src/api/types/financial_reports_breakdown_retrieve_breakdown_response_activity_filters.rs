pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrieveBreakdownResponseActivityFilters {
    pub direction: RetrieveBreakdownResponseActivityFiltersDirection,
    #[serde(default)]
    pub line_types: Vec<String>,
}

impl RetrieveBreakdownResponseActivityFilters {
    pub fn builder() -> RetrieveBreakdownResponseActivityFiltersBuilder {
        <RetrieveBreakdownResponseActivityFiltersBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrieveBreakdownResponseActivityFiltersBuilder {
    direction: Option<RetrieveBreakdownResponseActivityFiltersDirection>,
    line_types: Option<Vec<String>>,
}

impl RetrieveBreakdownResponseActivityFiltersBuilder {
    pub fn direction(mut self, value: RetrieveBreakdownResponseActivityFiltersDirection) -> Self {
        self.direction = Some(value);
        self
    }

    pub fn line_types(mut self, value: Vec<String>) -> Self {
        self.line_types = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrieveBreakdownResponseActivityFilters`].
    /// This method will fail if any of the following fields are not set:
    /// - [`direction`](RetrieveBreakdownResponseActivityFiltersBuilder::direction)
    /// - [`line_types`](RetrieveBreakdownResponseActivityFiltersBuilder::line_types)
    pub fn build(self) -> Result<RetrieveBreakdownResponseActivityFilters, BuildError> {
        Ok(RetrieveBreakdownResponseActivityFilters {
            direction: self
                .direction
                .ok_or_else(|| BuildError::missing_field("direction"))?,
            line_types: self
                .line_types
                .ok_or_else(|| BuildError::missing_field("line_types"))?,
        })
    }
}
