pub use crate::prelude::*;

/// The typename of this object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MetricStatsResponseTypename {
    #[serde(rename = "Result")]
    Result_,
}
impl fmt::Display for MetricStatsResponseTypename {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Result_ => "Result",
        };
        write!(f, "{}", s)
    }
}
